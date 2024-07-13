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
