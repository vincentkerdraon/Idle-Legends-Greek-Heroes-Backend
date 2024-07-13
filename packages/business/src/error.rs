use core::fmt;

use api::FeatID;

#[derive(Debug)]
pub enum BusinessError {
    GenerationError(String),
    FeatUnknownError(FeatID),
    FeatAlreadyDoneError(),
}

impl fmt::Display for BusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BusinessError::GenerationError(msg) => write!(f, "Generation Error: {}", msg),
            BusinessError::FeatUnknownError(feat_id) => {
                write!(f, "Feat Unknown Error: {:?}", feat_id)
            }
            BusinessError::FeatAlreadyDoneError() => {
                write!(f, "Feat Already Done")
            }
        }
    }
}

impl std::error::Error for BusinessError {}
