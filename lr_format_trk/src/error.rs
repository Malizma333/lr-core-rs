use std::{error::Error, fmt, io, num, str};

#[derive(Debug)]
pub enum TrkReadError {
    InvalidMagicNumber(String),
    UnsupportedTrackVersion(String),
    InvalidSongFormat(String),
    UnsupportedLineType(String),
    InvalidKeyValue(String),
    EmptyTriggerData,
    UnsupportedTriggerType(String),
    Other(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for TrkReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::InvalidMagicNumber(e) => write!(f, "Invalid magic number: {}", e),
            Self::UnsupportedTrackVersion(e) => write!(f, "Unsupported track version: {}", e),
            Self::InvalidSongFormat(e) => write!(f, "Invalid song format: {}", e),
            Self::UnsupportedLineType(e) => write!(f, "Unsupported line type: {}", e),
            Self::InvalidKeyValue(e) => write!(f, "Invalid key value format: {}", e),
            Self::EmptyTriggerData => write!(f, "Empty trigger data"),
            Self::UnsupportedTriggerType(e) => write!(f, "Unsupported trigger type: {}", e),
            Self::Other(e) => write!(f, "Other error occurred: {}", e),
        }
    }
}

impl Error for TrkReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            TrkReadError::Other(e) => Some(&**e),
            _ => None,
        }
    }
}

impl From<io::Error> for TrkReadError {
    fn from(value: io::Error) -> Self {
        TrkReadError::Other(Box::new(value))
    }
}

impl From<num::TryFromIntError> for TrkReadError {
    fn from(value: num::TryFromIntError) -> Self {
        TrkReadError::Other(Box::new(value))
    }
}

impl From<num::ParseIntError> for TrkReadError {
    fn from(value: num::ParseIntError) -> Self {
        TrkReadError::Other(Box::new(value))
    }
}

impl From<num::ParseFloatError> for TrkReadError {
    fn from(value: num::ParseFloatError) -> Self {
        TrkReadError::Other(Box::new(value))
    }
}

impl From<str::Utf8Error> for TrkReadError {
    fn from(value: str::Utf8Error) -> Self {
        TrkReadError::Other(Box::new(value))
    }
}
