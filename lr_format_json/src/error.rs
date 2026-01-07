use std::{error::Error, fmt, num, str};

#[derive(Debug)]
pub enum JsonReadError {
    UnsupportedGridVersion(String),
    UnsupportedLineType(String),
    UnsupportedTriggerType(String),
    InvalidTriggerFormat(String),
    Other(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for JsonReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::UnsupportedGridVersion(e) => write!(f, "Unsupported grid version: {}", e),
            Self::UnsupportedLineType(e) => write!(f, "Unsupported line type: {}", e),
            Self::UnsupportedTriggerType(e) => write!(f, "Unsupported trigger type: {}", e),
            Self::InvalidTriggerFormat(e) => write!(f, "Invalid trigger format: {}", e),
            Self::Other(e) => write!(f, "Other error occurred: {}", e),
        }
    }
}

impl Error for JsonReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            JsonReadError::Other(e) => Some(&**e),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for JsonReadError {
    fn from(value: serde_json::Error) -> Self {
        JsonReadError::Other(Box::new(value))
    }
}

impl From<num::TryFromIntError> for JsonReadError {
    fn from(value: num::TryFromIntError) -> Self {
        JsonReadError::Other(Box::new(value))
    }
}

impl From<str::Utf8Error> for JsonReadError {
    fn from(value: str::Utf8Error) -> Self {
        JsonReadError::Other(Box::new(value))
    }
}
