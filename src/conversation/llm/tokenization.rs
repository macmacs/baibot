use tiktoken_rs::CoreBPE;
use tiktoken_rs::get_bpe_from_tokenizer;
use tiktoken_rs::tokenizer;

use super::{Author, Message, MessageContent};

fn get_bpe_for_model(model: &str) -> CoreBPE {
    let tokenizer = tokenizer::get_tokenizer(model)
        .or_else(|| tokenizer::get_tokenizer("gpt-4"))
        .unwrap();

    get_bpe_from_tokenizer(tokenizer).unwrap()
}

pub fn shorten_messages_list_to_context_size(
    model: &str,
    prompt_message: &Option<Message>,
    mut messages: Vec<Message>,
    max_response_tokens: Option<u32>,
    max_context_tokens: u32,
) -> Vec<Message> {
    // Loading the tokenization data is an expensive process, so
    // se construct the BPE instance once and then use it for all messages.
    let bpe = get_bpe_for_model(model);

    // We want to retain the prompt in all cases, so we always count it first.
    // We also always reserve enough tokens for the maximum response we expect.
    let mut current_context_length: u32 = if let Some(prompt_message) = prompt_message {
        calculate_token_size_for_message(&bpe, model, prompt_message)
            + max_response_tokens.unwrap_or(0)
    } else {
        0
    };

    messages.reverse();

    let mut messages_to_keep: Vec<Message> = Vec::new();

    for message in messages {
        let tokens_for_message = calculate_token_size_for_message(&bpe, model, &message);

        if current_context_length + tokens_for_message > max_context_tokens {
            break;
        }

        current_context_length += tokens_for_message;

        messages_to_keep.push(message);
    }

    messages_to_keep.reverse();

    messages_to_keep
}

/// Calculate the token size of a message for a given model, with a preloaded CoreBPE object.
/// Related to `calculate_token_size_for_model_message`.
fn calculate_token_size_for_message(bpe: &CoreBPE, model: &str, message: &Message) -> u32 {
    let (tokens_per_message, tokens_per_name) = if model.starts_with("gpt-3.5") {
        (
            4,  // every message follows <im_start>{role/name}\n{content}<im_end>\n
            -1, // if there's a name, the role is omitted
        )
    } else {
        (3, 1)
    };

    let role_length = match message.author {
        Author::Assistant => bpe.encode_with_special_tokens("assistant").len() as i32,
        Author::User => bpe.encode_with_special_tokens("user").len() as i32,
        Author::Prompt => bpe.encode_with_special_tokens("system").len() as i32,
    };

    let text_length = match &message.content {
        MessageContent::Text(text) => bpe.encode_with_special_tokens(text).len() as i32,
        MessageContent::Image(..) => 0,
    };

    (text_length + role_length + tokens_per_message + tokens_per_name) as u32
}

pub mod test {
    #[test]
    fn message_size_counting_works() {
        let model = "gpt-4";

        let bpe = super::get_bpe_for_model(model);

        let message = super::Message {
            author: super::Author::User,
            content: super::MessageContent::Text("Hello there!".to_string()),
            timestamp: chrono::Utc::now(),
        };

        let tokens = super::calculate_token_size_for_message(&bpe, model, &message);

        assert_eq!(8, tokens);
    }

    #[test]
    fn shortening_works_with_english() {
        let model = "gpt-4";

        let bpe = super::get_bpe_for_model(model);

        let max_response_tokens: Option<u32> = Some(5);

        let prompt = super::Message {
            author: super::Author::Prompt,
            content: super::MessageContent::Text("You are a bot!".to_string()),
            timestamp: chrono::Utc::now(),
        };
        let prompt_length = 10;

        assert_eq!(
            prompt_length,
            super::calculate_token_size_for_message(&bpe, model, &prompt)
        );

        let mut conversation_messages = Vec::new();

        let first = super::Message {
            author: super::Author::User,
            content: super::MessageContent::Text("Hello there!".to_string()),
            timestamp: chrono::Utc::now(),
        };
        let first_length = 8;

        assert_eq!(
            first_length,
            super::calculate_token_size_for_message(&bpe, model, &first)
        );

        conversation_messages.push(first);

        let second = super::Message {
            author: super::Author::Assistant,
            content: super::MessageContent::Text("Hello!".to_string()),
            timestamp: chrono::Utc::now(),
        };
        let second_length = 7;

        assert_eq!(
            second_length,
            super::calculate_token_size_for_message(&bpe, model, &second)
        );

        conversation_messages.push(second);

        let third = super::Message {
            author: super::Author::User,
            content: super::MessageContent::Text(
                "This is the 3rd message in this conversation. It shall be preserved.".to_owned(),
            ),
            timestamp: chrono::Utc::now(),
        };
        let third_length = 21;

        assert_eq!(
            third_length,
            super::calculate_token_size_for_message(&bpe, model, &third)
        );

        conversation_messages.push(third.clone());

        let forth = super::Message {
            author: super::Author::Assistant,
            content: super::MessageContent::Text(
                "This is yet another message that shall be preserved.".to_owned(),
            ),
            timestamp: chrono::Utc::now(),
        };
        let forth_length = 15;

        assert_eq!(
            forth_length,
            super::calculate_token_size_for_message(&bpe, model, &forth)
        );

        conversation_messages.push(forth.clone());

        assert_eq!(4, conversation_messages.len());

        let new_conversation_messages = super::shorten_messages_list_to_context_size(
            model,
            &Some(prompt),
            conversation_messages,
            max_response_tokens,
            prompt_length + max_response_tokens.unwrap_or(0) + forth_length + third_length,
        );

        assert_eq!(2, new_conversation_messages.len());

        assert_eq!(
            new_conversation_messages.first().unwrap().content,
            third.content
        );

        assert_eq!(
            new_conversation_messages.last().unwrap().content,
            forth.content
        );
    }

    #[test]
    fn shortening_works_with_japanese() {
        let model = "gpt-4";

        let bpe = super::get_bpe_for_model(model);

        let max_response_tokens: Option<u32> = Some(5);

        let prompt = super::Message {
            author: super::Author::User,
            content: super::MessageContent::Text("あなたはボットです。".to_string()),
            timestamp: chrono::Utc::now(),
        };
        let prompt_length = 14;

        assert_eq!(
            prompt_length,
            super::calculate_token_size_for_message(&bpe, model, &prompt)
        );

        let mut conversation_messages = Vec::new();

        let first = super::Message {
            author: super::Author::User,
            content: super::MessageContent::Text("こんにちは!".to_string()),
            timestamp: chrono::Utc::now(),
        };
        let first_length = 7;

        assert_eq!(
            first_length,
            super::calculate_token_size_for_message(&bpe, model, &first)
        );

        conversation_messages.push(first);

        let second = super::Message {
            author: super::Author::Assistant,
            content: super::MessageContent::Text("こんにちは。今日は元気ですか。".to_string()),
            timestamp: chrono::Utc::now(),
        };
        let second_length = 15;

        assert_eq!(
            second_length,
            super::calculate_token_size_for_message(&bpe, model, &second)
        );

        conversation_messages.push(second);

        let third = super::Message {
            author: super::Author::User,
            content: super::MessageContent::Text(
                "これは第3のメッセージなので、保存されます。".to_string(),
            ),
            timestamp: chrono::Utc::now(),
        };
        let third_length = 22;

        assert_eq!(
            third_length,
            super::calculate_token_size_for_message(&bpe, model, &third)
        );

        conversation_messages.push(third.clone());

        let forth = super::Message {
            author: super::Author::Assistant,
            content: super::MessageContent::Text(
                "これはもう一つの保存されますメッセージです。".to_string(),
            ),
            timestamp: chrono::Utc::now(),
        };
        let forth_length = 21;

        assert_eq!(
            forth_length,
            super::calculate_token_size_for_message(&bpe, model, &forth)
        );

        conversation_messages.push(forth.clone());

        assert_eq!(4, conversation_messages.len());

        let new_conversation_messages = super::shorten_messages_list_to_context_size(
            model,
            &Some(prompt),
            conversation_messages,
            max_response_tokens,
            prompt_length + max_response_tokens.unwrap_or(0) + forth_length + third_length,
        );

        assert_eq!(2, new_conversation_messages.len());

        assert_eq!(
            new_conversation_messages.first().unwrap().content,
            third.content
        );

        assert_eq!(
            new_conversation_messages.last().unwrap().content,
            forth.content
        );
    }
}
