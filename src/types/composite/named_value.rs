use crate::{
    commom::{
        delimiters::{BUFFER_END, END_RECORD, START_RECORD, VALUE_DELIMITER},
        errors::{decoding_error, DecodingError, DecodingErrors, TypeResult},
        join_parts, slice_records, split_values, BuffPart, CoprotoType, Uint8Buff, ValueOrBuffer,
    },
    types::{infer_buffer, primitive, BigInt, Boolean, Double, Integer, Null, SupportedTypes},
};

#[derive(Debug)]
pub struct NamedValue {
    pub first_byte: u8,
    pub modifier_byte: Option<u8>,
    pub modifier_char: Option<char>,
    pub first_char: char,
    pub value_of: TypeResult<(String, SupportedTypes)>,
    pub buff: TypeResult<Uint8Buff>,
}

impl CoprotoType<(String, SupportedTypes)> for NamedValue {
    const FIRST_BYTE: u8 = b'@';

    fn new(value: ValueOrBuffer<(String, SupportedTypes)>) -> Self {
        match value {
            ValueOrBuffer::Value(v) => Self {
                first_byte: Self::FIRST_BYTE,
                modifier_byte: None,
                modifier_char: None,
                first_char: '@',
                value_of: Ok(v.clone()),
                buff: Self::encode(v),
            },
            ValueOrBuffer::Buffer(vec) => Self {
                first_byte: Self::FIRST_BYTE,
                modifier_byte: None,
                modifier_char: None,
                first_char: '@',
                value_of: Self::decode(vec.clone()),
                buff: Ok(vec),
            },
        }
    }

    fn encode(value: (String, SupportedTypes)) -> TypeResult<Uint8Buff> {
        let mut parts: Vec<BuffPart> =
            vec![BuffPart::Val(Self::FIRST_BYTE), BuffPart::Val(START_RECORD)];

        let mut command_name = crate::types::String::encode(value.0)?;

        command_name.pop();

        parts.push(BuffPart::Arr(command_name));

        parts.push(BuffPart::Val(VALUE_DELIMITER));

        match value.1 {
            SupportedTypes::BigInt(bi) => {
                let mut bi_encoded = BigInt::encode(bi)?;
                bi_encoded.pop();
                parts.push(BuffPart::Arr(bi_encoded));
            }
            SupportedTypes::Boolean(bol) => {
                let mut bol_encoded = Boolean::encode(bol)?;
                bol_encoded.pop();
                parts.push(BuffPart::Arr(bol_encoded));
            }
            SupportedTypes::Double(db) => {
                let mut db_encoded = Double::encode(db)?;
                db_encoded.pop();
                parts.push(BuffPart::Arr(db_encoded));
            }
            SupportedTypes::Integer(int) => {
                let mut int_encoded = Integer::encode(int)?;
                int_encoded.pop();
                parts.push(BuffPart::Arr(int_encoded));
            }
            SupportedTypes::Null(null_val) => {
                let mut null_encoded = Null::encode(null_val)?;
                null_encoded.pop();
                parts.push(BuffPart::Arr(null_encoded));
            }
            SupportedTypes::String(string) => {
                let mut string_encoded = primitive::String::encode(string.clone())?;
                string_encoded.pop();
                parts.push(BuffPart::Arr(string_encoded));
            }
        }

        parts.push(BuffPart::Val(VALUE_DELIMITER));
        parts.push(BuffPart::Val(END_RECORD));
        parts.push(BuffPart::Val(BUFFER_END));

        Ok(join_parts(parts))
    }

    fn decode(value: Uint8Buff) -> TypeResult<(String, SupportedTypes)> {
        let mut m_value = value.clone();

        let first_byte = m_value.remove(0);

        if first_byte != Self::FIRST_BYTE {
            return Err(decoding_error(DecodingError {
                from: value,
                to: "NamedValue".to_string(),
                cause: DecodingErrors::FirstByteError(
                    "NamedValue".to_string(),
                    Self::FIRST_BYTE,
                    first_byte,
                ),
            }));
        };

        let records = slice_records(m_value.clone());

        if records.len() > 1 {
            return Err(decoding_error(DecodingError::new(
                value,
                "NamedValue",
                DecodingErrors::TooMuch(
                    "Records".to_string(),
                    1,
                    records.len().try_into().unwrap(),
                ),
            )));
        };

        let named_value_record = match records.first() {
            Some(v_arr) => v_arr,
            None => {
                return Err(decoding_error(DecodingError {
                    from: value.clone(),
                    to: "NamedValue".to_string(),
                    cause: DecodingErrors::NotEnough(
                        "Bytes".to_string(),
                        4,
                        value.len().try_into().unwrap(),
                    ),
                }))
            }
        };

        let values = split_values(named_value_record.clone());

        let name = match values.first() {
            Some(v) => crate::types::String::decode(v.to_vec())?,
            None => {
                return Err(decoding_error(DecodingError::new(
                    value,
                    "NamedValue",
                    DecodingErrors::NotEnough(
                        "Values".to_string(),
                        2,
                        values.len().try_into().unwrap(),
                    ),
                )))
            }
        };

        let value = match values.get(1) {
            Some(v) => infer_buffer(v.to_vec())?,
            None => {
                return Err(decoding_error(DecodingError::new(
                    value,
                    "NamedValue",
                    DecodingErrors::NotEnough(
                        "Values".to_string(),
                        2,
                        values.len().try_into().unwrap(),
                    ),
                )))
            }
        };

        Ok((name, value))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commom::{
            delimiters::{BUFFER_END, END_RECORD, START_RECORD},
            CoprotoType, ValueOrBuffer,
        },
        types::{NamedValue, SupportedTypes},
    };

    #[test]
    fn test_encoding_decoding() {
        let types_to_test = [
            SupportedTypes::BigInt(123456789),
            SupportedTypes::Boolean(false),
            SupportedTypes::Double(1.2345),
            SupportedTypes::Integer(123),
            SupportedTypes::Null(None),
            SupportedTypes::String("Hello, fellow rustacean!".to_string()),
        ];

        for tp in types_to_test.iter() {
            let encoding = NamedValue::new(ValueOrBuffer::Value((
                tp.get_name().to_string(),
                tp.clone(),
            )));

            let buff = encoding.buff.unwrap();
            let decoding = NamedValue::new(ValueOrBuffer::Buffer(buff));

            assert_eq!(decoding.value_of.unwrap(), encoding.value_of.unwrap());
        }
    }

    #[test]
    fn wrong_buffer() {
        let buff = vec![b'?', START_RECORD, 0, END_RECORD, BUFFER_END];

        let wrong = NamedValue::new(ValueOrBuffer::Buffer(buff));

        match wrong.value_of {
            Ok(_) => false,
            Err(e) => match e {
                crate::commom::errors::TypeError::Encoding(_) => false,
                crate::commom::errors::TypeError::Decoding(decoding_error) => {
                    matches!(
                        decoding_error.cause,
                        crate::commom::errors::DecodingErrors::FirstByteError(_, _, _)
                    )
                }
            },
        };
    }
}
