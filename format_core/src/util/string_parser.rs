use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Cursor, Error as IoError, Read};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ParseLengthPrefixedStringError {
    IoError(IoError),
    Utf8Error {
        length: usize,
        source: FromUtf8Error,
    },
}

impl Display for ParseLengthPrefixedStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            ParseLengthPrefixedStringError::IoError(source) => {
                write!(f, "IO error while reading string: {}", source)
            }
            ParseLengthPrefixedStringError::Utf8Error { length, source } => write!(
                f,
                "Invalid UTF-8 while parsing string of length {}: {}",
                length, source
            ),
        }
    }
}

impl Error for ParseLengthPrefixedStringError {}

pub enum StringLength {
    U16,
    Fixed(usize),
}

pub enum Endianness {
    Big,
    Little,
}

/// Generalized function for reading binary length-prefixed strings
pub fn parse_string(
    cursor: &mut Cursor<&Vec<u8>>,
    length_type: StringLength,
    length_endianness: Endianness,
) -> Result<String, ParseLengthPrefixedStringError> {
    let length = match length_type {
        StringLength::U16 => {
            let mut length_bytes: [u8; 2] = [0, 0];
            cursor
                .read_exact(&mut length_bytes)
                .map_err(|source| ParseLengthPrefixedStringError::IoError(source))?;
            let size = match length_endianness {
                Endianness::Big => u16::from_be_bytes(length_bytes),
                Endianness::Little => u16::from_le_bytes(length_bytes),
            };
            usize::from(size)
        }
        StringLength::Fixed(size) => size,
    };

    let mut buffer = vec![0; length];
    cursor
        .read_exact(&mut buffer)
        .map_err(|source| ParseLengthPrefixedStringError::IoError(source))?;
    let string = String::from_utf8(buffer)
        .map_err(|source| ParseLengthPrefixedStringError::Utf8Error { length, source })?;

    Ok(string)
}
