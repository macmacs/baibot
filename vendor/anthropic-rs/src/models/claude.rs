use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClaudeModel {
    #[serde(rename = "claude-sonnet-4-20250514")]
    ClaudeSonnet420250514,
    #[serde(rename = "claude-3-7-sonnet-20250219")]
    Claude37Sonnet20250219,
    #[serde(rename = "claude-3-7-sonnet-latest")]
    Claude37SonnetLatest,
    #[serde(rename = "claude-3-5-sonnet-latest")]
    Claude35SonnetLatest,
    #[serde(rename = "claude-3-5-sonnet-20240620")]
    Claude35Sonnet,
    #[serde(rename = "claude-3-5-opus-20240620")]
    Claude35Opus,
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet-20240229")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku-20240307")]
    Claude3Haiku,
}

impl ClaudeModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ClaudeSonnet420250514 => "claude-sonnet-4-20250514",
            Self::Claude37Sonnet20250219 => "claude-3-7-sonnet-20250219",
            Self::Claude37SonnetLatest => "claude-3-7-sonnet-latest",
            Self::Claude35SonnetLatest => "claude-3-5-sonnet-latest",
            Self::Claude35Sonnet => "claude-3-5-sonnet-20240620",
            Self::Claude35Opus => "claude-3-5-opus-20240620",
            Self::Claude3Opus => "claude-3-opus-20240229",
            Self::Claude3Sonnet => "claude-3-sonnet-20240229",
            Self::Claude3Haiku => "claude-3-haiku-20240307",
        }
    }
}

impl Default for ClaudeModel {
    fn default() -> Self {
        Self::Claude37SonnetLatest
    }
}

impl FromStr for ClaudeModel {
    type Err = crate::error::AnthropicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "claude-sonnet-4-20250514" => Ok(Self::ClaudeSonnet420250514),
            "claude-3-7-sonnet-20250219" => Ok(Self::Claude37Sonnet20250219),
            "claude-3-7-sonnet-latest" => Ok(Self::Claude37SonnetLatest),
            "claude-3-5-sonnet-latest" => Ok(Self::Claude35SonnetLatest),
            "claude-3-5-sonnet-20240620" => Ok(Self::Claude35Sonnet),
            "claude-3-5-opus-20240620" => Ok(Self::Claude35Opus),
            "claude-3-opus-20240229" => Ok(Self::Claude3Opus),
            "claude-3-sonnet-20240229" => Ok(Self::Claude3Sonnet),
            "claude-3-haiku-20240307" => Ok(Self::Claude3Haiku),
            _ => Err(crate::error::AnthropicError::ModelNotSupported(
                s.to_string(),
            )),
        }
    }
}

impl fmt::Display for ClaudeModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::AnthropicError;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_to_correct_model_names() {
        assert_eq!(
            ClaudeModel::ClaudeSonnet420250514.as_str(),
            "claude-sonnet-4-20250514",
        );
        assert_eq!(
            ClaudeModel::Claude37Sonnet20250219.as_str(),
            "claude-3-7-sonnet-20250219",
        );
        assert_eq!(
            ClaudeModel::Claude37SonnetLatest.as_str(),
            "claude-3-7-sonnet-latest",
        );
        assert_eq!(
            ClaudeModel::Claude35SonnetLatest.as_str(),
            "claude-3-5-sonnet-latest",
        );
        assert_eq!(
            ClaudeModel::Claude35Sonnet.as_str(),
            "claude-3-5-sonnet-20240620",
        );
        assert_eq!(
            ClaudeModel::Claude35Opus.as_str(),
            "claude-3-5-opus-20240620",
        );
        assert_eq!(ClaudeModel::Claude3Opus.as_str(), "claude-3-opus-20240229");
        assert_eq!(
            ClaudeModel::Claude3Sonnet.as_str(),
            "claude-3-sonnet-20240229"
        );
        assert_eq!(
            ClaudeModel::Claude3Haiku.as_str(),
            "claude-3-haiku-20240307"
        );
    }

    #[test]
    fn should_deserialize_to_correct_models() {
        assert_eq!(
            ClaudeModel::ClaudeSonnet420250514,
            ClaudeModel::from_str("claude-sonnet-4-20250514").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude37Sonnet20250219,
            ClaudeModel::from_str("claude-3-7-sonnet-20250219").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude37SonnetLatest,
            ClaudeModel::from_str("claude-3-7-sonnet-latest").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude35SonnetLatest,
            ClaudeModel::from_str("claude-3-5-sonnet-latest").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude35Sonnet,
            ClaudeModel::from_str("claude-3-5-sonnet-20240620").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude35Opus,
            ClaudeModel::from_str("claude-3-5-opus-20240620").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude3Opus,
            ClaudeModel::from_str("claude-3-opus-20240229").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude3Sonnet,
            ClaudeModel::from_str("claude-3-sonnet-20240229").unwrap(),
        );
        assert_eq!(
            ClaudeModel::Claude3Haiku,
            ClaudeModel::from_str("claude-3-haiku-20240307").unwrap(),
        );
    }

    #[test]
    fn should_return_error_for_invalid_model() {
        assert!(matches!(
            ClaudeModel::from_str("claude-invalid-model"),
            Err(AnthropicError::ModelNotSupported(_))
        ));
    }
}
