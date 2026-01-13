use std::{error::Error, fmt, io, num, str};

#[derive(Debug)]
pub enum SolReadError {
    InvalidMagicNumber(String),
    MissingTrackList,
    InvalidTrackList(String),
    InvalidTrackIndex(String),
    InvalidTrack(String),
    InvalidLabel(String),
    InvalidGridVersion(String),
    UnsupportedGridVersion(String),
    InvalidStartLine(String),
    InvalidLinesList(String),
    InvalidLine(String),
    UnsupportedLineType(String),
    Other(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for SolReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::InvalidMagicNumber(e) => write!(f, "Invalid magic number: {}", e),
            Self::MissingTrackList => write!(f, "Missing track list"),
            Self::InvalidTrackList(e) => write!(f, "Invalid track list: {}", e),
            Self::InvalidTrackIndex(e) => write!(f, "Invalid track index: {}", e),
            Self::InvalidTrack(e) => write!(f, "Invalid track: {}", e),
            Self::InvalidLabel(e) => write!(f, "Invalid label: {}", e),
            Self::InvalidGridVersion(e) => write!(f, "Invalid grid version: {}", e),
            Self::UnsupportedGridVersion(e) => write!(f, "Unsupported grid version: {}", e),
            Self::InvalidStartLine(e) => write!(f, "Invalid start line: {}", e),
            Self::InvalidLinesList(e) => write!(f, "Invalid lines list: {}", e),
            Self::InvalidLine(e) => write!(f, "Invalid line: {}", e),
            Self::UnsupportedLineType(e) => write!(f, "Unsupported line type: {}", e),
            Self::Other(e) => write!(f, "Other error occurred: {}", e),
        }
    }
}

impl Error for SolReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            SolReadError::Other(e) => Some(&**e),
            _ => None,
        }
    }
}

impl From<amf0::DeserializationError> for SolReadError {
    fn from(value: amf0::DeserializationError) -> Self {
        SolReadError::Other(Box::new(value))
    }
}

impl From<io::Error> for SolReadError {
    fn from(value: io::Error) -> Self {
        SolReadError::Other(Box::new(value))
    }
}

impl From<str::Utf8Error> for SolReadError {
    fn from(value: str::Utf8Error) -> Self {
        SolReadError::Other(Box::new(value))
    }
}

impl From<num::TryFromIntError> for SolReadError {
    fn from(value: num::TryFromIntError) -> Self {
        SolReadError::Other(Box::new(value))
    }
}
