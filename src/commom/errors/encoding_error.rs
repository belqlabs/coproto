use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum EncodingErrors {
    DecodedWouldOverflow,
    InvalidValue(String),
    SizeConversionError(String, String),
    InternalError(Box<dyn Error>),
    TableMisfit(usize, usize), // Headers length, Row length
}

impl fmt::Display for EncodingErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_str = match self {
            EncodingErrors::DecodedWouldOverflow => {
                "The encoding would have been fine. But this value would not be decoded correctelly"
                    .to_string()
            }
            EncodingErrors::InvalidValue(value) => format!("Invalid value: {}", value),
            EncodingErrors::SizeConversionError(from, to) => {
                format!("{} could not be converted to {}.", from, to)
            }
            EncodingErrors::InternalError(error) => error.to_string(),
            EncodingErrors::TableMisfit(hl, rl) => {
                format!("Cant fit {} records in {} headers", rl, hl)
            }
        };

        write!(f, "{}", err_str)
    }
}

#[derive(Debug)]
pub struct EncodingError {
    pub from: String,
    pub to: String,
    pub origin: EncodingErrors,
}

impl EncodingError {
    pub fn new(from: &str, to: &str, origin: EncodingErrors) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            origin,
        }
    }
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ENCODING ERROR] Could not convert {} to {}\n  [ERROR ORIGIN] {}",
            self.from, self.to, self.origin
        )
    }
}

impl Error for EncodingError {}

pub type EncodingResult<T> = Result<T, EncodingError>;
