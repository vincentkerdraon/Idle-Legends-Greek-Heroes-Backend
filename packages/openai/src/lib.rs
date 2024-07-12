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

pub fn load_secret() -> Result<String, Error> {
    let secret_key =
        env::var("OPENAI_SECRET").expect("Please set the OPENAI_SECRET environment variable");
    Ok(secret_key)
}

#[derive(Clone)]
pub struct OpenAI {
    secret_key: String,
}
impl OpenAI {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }

    pub async fn generate_text(
        &self,
        prompt_system: &str,
        prompt_user: &str,
    ) -> Result<String, OpenAIError> {
        let client = reqwest::Client::new();
        const URL: &str = "https://api.openai.com/v1/chat/completions";

        let json_request = json!({
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
        });

        let resp = client
            .post(URL)
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .json(&json_request)
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

        Err(OpenAIError::DataError(format!(
            "Unable to find valid response from OpenAI. {}",
            json_response
        )))
    }

    pub async fn generate_image(&self, _prompt: &str) -> Result<String, Error> {
        // Implement image API call logic here
        unimplemented!("Image API call not yet implemented")
    }
}
