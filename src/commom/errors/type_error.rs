use core::fmt;
use std::error::Error;

use super::{DecodingError, EncodingError};

#[derive(Debug)]
pub enum TypeError {
    Encoding(EncodingError),
    Decoding(DecodingError),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::Encoding(ee) => write!(f, "Type error: {}", ee),
            TypeError::Decoding(de) => write!(f, "Type error: {}", de),
        }
    }
}

impl Error for TypeError {}

pub fn encoding_error(e: EncodingError) -> TypeError {
    TypeError::Encoding(e)
}

pub fn decoding_error(e: DecodingError) -> TypeError {
    TypeError::Decoding(e)
}

pub type TypeResult<T> = Result<T, TypeError>;
