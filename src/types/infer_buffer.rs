use crate::commom::errors::{decoding_error, DecodingError, DecodingErrors, TypeResult};
use crate::commom::{CoprotoType, Uint8Buff};
use crate::types::SupportedTypes;

use super::{BigInt, Boolean, Double, Integer, Null};

pub fn infer_buffer(buff: Uint8Buff) -> TypeResult<SupportedTypes> {
    let first_byte = match buff.first() {
        Some(fb) => fb,
        None => {
            return Err(decoding_error(DecodingError::new(
                buff,
                "Infer",
                DecodingErrors::CouldNotFind(0, "First byte".to_string()),
            )))
        }
    };

    match *first_byte {
        BigInt::FIRST_BYTE => Ok(SupportedTypes::BigInt(BigInt::decode(buff)?)),
        Boolean::FIRST_BYTE => Ok(SupportedTypes::Boolean(Boolean::decode(buff)?)),
        Double::FIRST_BYTE => Ok(SupportedTypes::Double(Double::decode(buff)?)),
        Integer::FIRST_BYTE => Ok(SupportedTypes::Integer(Integer::decode(buff)?)),
        Null::FIRST_BYTE => Ok(SupportedTypes::Null(Null::decode(buff)?)),
        super::String::FIRST_BYTE => Ok(SupportedTypes::String(super::String::decode(buff)?)),
        _ => Err(decoding_error(DecodingError::new(
            buff.clone(),
            "Infer",
            DecodingErrors::UnknownFirstByte(
                *first_byte,
                vec![
                    BigInt::FIRST_BYTE,
                    Boolean::FIRST_BYTE,
                    Integer::FIRST_BYTE,
                    Null::FIRST_BYTE,
                    super::String::FIRST_BYTE,
                ],
            ),
        ))),
    }
}
