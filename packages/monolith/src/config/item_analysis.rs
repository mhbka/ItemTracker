use std::env::{self, VarError};

use serde::{Deserialize, Serialize};

/// Config for the item analysis module.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemAnalysisConfig {
    // These are used for accessing the Anthropic API.
    pub anthropic_api_endpoint: String,
    pub anthropic_api_key: String,
    pub anthropic_model: String,
    pub anthropic_version: String,
    // These are used for accessing the OpenAI API.
    pub openai_api_endpoint: String,
    pub openai_api_key: String,
    pub openai_model: String
}

impl ItemAnalysisConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        Ok(
            ItemAnalysisConfig {
                anthropic_api_endpoint: env::var("ANTHROPIC_API_ENDPOINT")?,
                anthropic_api_key: env::var("ANTHROPIC_API_KEY")?,
                anthropic_model: env::var("ANTHROPIC_MODEL")?,
                anthropic_version: env::var("ANTHROPIC_VERSION")?,
                openai_api_endpoint: env::var("OPENAI_API_ENDPOINT")?,
                openai_api_key: env::var("OPENAI_API_KEY")?,
                openai_model: env::var("OPENAI_MODEL")?,
            }
        )
    }
}