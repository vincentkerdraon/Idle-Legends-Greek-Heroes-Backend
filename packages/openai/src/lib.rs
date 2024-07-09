use reqwest::Error;
use serde_json::json;
use std::env;

use std::fmt;

#[derive(Debug)]
pub enum OpenAIError {
    ApiError(String),  // Represents errors returned from the OpenAI API
    DataError(String), // Represents errors when data retrieval fails
}

impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenAIError::ApiError(msg) => write!(f, "OpenAI API Error: {}", msg),
            OpenAIError::DataError(msg) => write!(f, "OpenAI Data Error: {}", msg),
        }
    }
}

impl std::error::Error for OpenAIError {}

pub struct OpenAI {
    secret_key: String,
}

impl OpenAI {
    pub fn new() -> Result<Self, Error> {
        let secret_key =
            env::var("OPENAI_SECRET").expect("Please set the OPENAI_SECRET environment variable");
        Ok(Self { secret_key })
    }

    pub async fn call_openai_text(
        &self,
        prompt_system: &str,
        prompt_user: &str,
    ) -> Result<String, OpenAIError> {
        let client = reqwest::Client::new();
        let url = "https://api.openai.com/v1/completions";

        let resp = client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                    {
                        "role": "system",
                        "content": prompt_system,
                    },
                    {
                        "role": "user",
                        "content": prompt_user,
                    },
                ],
                "max_tokens": 100
            }))
            .send()
            .await
            .map_err(|e| OpenAIError::ApiError(format!("Request to OpenAI API failed: {}", e)))?;

        let json_response: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| OpenAIError::ApiError(format!("Failed to parse JSON response: {}", e)))?;

        if let Some(choices) = json_response["choices"].as_array() {
            if let Some(choice) = choices.get(0) {
                if let Some(message) = choice["message"].as_object() {
                    if let Some(content) = message["content"].as_str() {
                        return Ok(content.to_string());
                    }
                }
            }
        }

        Err(OpenAIError::DataError(
            "Unable to find valid response from OpenAI".to_string(),
        ))
    }

    pub async fn call_openai_image(&self, _prompt: &str) -> Result<String, Error> {
        // Implement image API call logic here
        unimplemented!("Image API call not yet implemented")
    }
}
