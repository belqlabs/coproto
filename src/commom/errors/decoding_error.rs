use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum DecodingErrors {
    TooMuch(String, u32, u32),      // (found, expexted)
    NotEnough(String, u32, u32),    // (found, expexted)
    InvalidByte(u8, u32, Vec<u8>),  // (byte, position, expected)
    CouldNotFind(u8, String),       // (byte, byte_name)
    FirstByteError(String, u8, u8), // (found, expected)
    SizeConversionError(String, String),
    InternalError(Box<dyn Error>),
}

impl fmt::Display for DecodingErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_str = match self {
            DecodingErrors::TooMuch(name, expected, found) => format!(
                "Too Much {}:\nExpected: {}\nFound: {}",
                name, expected, found
            ),
            DecodingErrors::NotEnough(name, expected, found) => format!(
                "Not Enough {}:\nExpected: {}\nFound: {}",
                name, expected, found
            ),
            DecodingErrors::InvalidByte(byte, position, expected_byte) => format!(
                "Invalid byte {} found at buffer position {}. Expected: {:?}",
                byte, position, expected_byte
            ),
            DecodingErrors::CouldNotFind(byte, byte_name) => {
                format!("Could not find {} ({}). In the buffer", byte, byte_name)
            }
            DecodingErrors::FirstByteError(cannonical, found, should_be) => format!(
                "First byte of {}, should be {}. Found {}",
                cannonical, should_be, found
            ),
            DecodingErrors::SizeConversionError(from, to) => {
                format!("{} could not be converted to {}.", from, to)
            }
            DecodingErrors::InternalError(error) => error.to_string(),
        };

        write!(f, "{}", err_str)
    }
}

#[derive(Debug)]
pub struct DecodingError {
    pub from: Vec<u8>,
    pub to: String,
    pub cause: DecodingErrors,
}

impl DecodingError {
    pub fn new(from: Vec<u8>, to: &str, cause: DecodingErrors) -> Self {
        Self {
            from,
            to: to.to_string(),
            cause,
        }
    }
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[DECODING ERROR] Could not convert {:?} to {}\n  [ERROR ORIGIN] {}",
            self.from, self.to, self.cause
        )
    }
}

impl Error for DecodingError {}

pub type DecodingResult<T> = Result<T, DecodingError>;
