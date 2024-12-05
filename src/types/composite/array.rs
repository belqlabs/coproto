use crate::{
    commom::{
        delimiters::{BUFFER_END, END_RECORD, START_RECORD, VALUE_DELIMITER},
        errors::{decoding_error, DecodingError, DecodingErrors, TypeResult},
        join_parts, slice_records, BuffPart, CoprotoType, Uint8Buff, ValueOrBuffer,
    },
    types::{infer_buffer, primitive, BigInt, Boolean, Double, Integer, Null, SupportedTypes},
};

#[derive(Debug)]
pub struct Array {
    pub first_byte: u8,
    pub modifier_byte: Option<u8>,
    pub modifier_char: Option<char>,
    pub first_char: char,
    pub value_of: TypeResult<Vec<SupportedTypes>>,
    pub buff: TypeResult<Uint8Buff>,
}

impl CoprotoType<Vec<SupportedTypes>> for Array {
    const FIRST_BYTE: u8 = b'[';

    fn new(value: ValueOrBuffer<Vec<SupportedTypes>>) -> Self {
        match value {
            ValueOrBuffer::Value(value) => Self {
                first_byte: Self::FIRST_BYTE,
                modifier_byte: None,
                modifier_char: None,
                first_char: '[',
                value_of: Ok(value.clone()),
                buff: Self::encode(value),
            },
            ValueOrBuffer::Buffer(vec) => Self {
                first_byte: Self::FIRST_BYTE,
                modifier_byte: None,
                modifier_char: None,
                first_char: '[',
                value_of: Self::decode(vec.clone()),
                buff: Ok(vec),
            },
        }
    }

    fn encode(values: Vec<SupportedTypes>) -> TypeResult<Uint8Buff> {
        let mut parts: Vec<BuffPart> = vec![BuffPart::Val(b'['), BuffPart::Val(START_RECORD)];

        for value in values.iter() {
            let encoded_value = match value {
                SupportedTypes::BigInt(bi) => {
                    let mut bi_encoded = BigInt::encode(*bi)?;
                    bi_encoded.pop();
                    BuffPart::Arr(bi_encoded)
                }
                SupportedTypes::Boolean(bol) => {
                    let mut bol_encoded = Boolean::encode(*bol)?;
                    bol_encoded.pop();
                    BuffPart::Arr(bol_encoded)
                }
                SupportedTypes::Double(db) => {
                    let mut db_encoded = Double::encode(*db)?;
                    db_encoded.pop();
                    BuffPart::Arr(db_encoded)
                }
                SupportedTypes::Integer(int) => {
                    let mut int_encoded = Integer::encode(*int)?;
                    int_encoded.pop();
                    BuffPart::Arr(int_encoded)
                }
                SupportedTypes::Null(null_val) => {
                    let mut null_encoded = Null::encode(*null_val)?;
                    null_encoded.pop();
                    BuffPart::Arr(null_encoded)
                }
                SupportedTypes::String(string) => {
                    let mut string_encoded = primitive::String::encode(string.clone())?;
                    string_encoded.pop();
                    BuffPart::Arr(string_encoded)
                }
            };

            parts.push(BuffPart::Val(START_RECORD));
            parts.push(encoded_value);
            parts.push(BuffPart::Val(END_RECORD));
            parts.push(BuffPart::Val(VALUE_DELIMITER));
        }

        parts.push(BuffPart::Val(END_RECORD));
        parts.push(BuffPart::Val(BUFFER_END));

        Ok(join_parts(parts))
    }

    fn decode(value: Uint8Buff) -> TypeResult<Vec<SupportedTypes>> {
        let mut array: Vec<SupportedTypes> = vec![];

        let mut m_value = value.clone();

        let first_byte = m_value.remove(0);

        if first_byte != Self::FIRST_BYTE {
            return Err(decoding_error(DecodingError {
                from: value,
                to: "Array".to_string(),
                cause: DecodingErrors::FirstByteError(
                    "Array".to_string(),
                    Self::FIRST_BYTE,
                    first_byte,
                ),
            }));
        };

        let records = slice_records(m_value.clone());

        for record in records.iter() {
            if record.is_empty() {
                continue;
            }

            array.push(infer_buffer(record.to_vec())?);
        }

        Ok(array)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commom::{
            delimiters::{BUFFER_END, END_RECORD, START_RECORD},
            CoprotoType, ValueOrBuffer,
        },
        types::{Array, SupportedTypes},
    };

    #[test]
    fn test_filled_encoding_decoding() {
        let vec_to_test = vec![
            SupportedTypes::BigInt(123456789),
            SupportedTypes::Boolean(false),
            SupportedTypes::Double(1.2345),
            SupportedTypes::Integer(123),
            SupportedTypes::Null(None),
            SupportedTypes::String("Hello, fellow rustacean!".to_string()),
        ];

        let filled_encoding = Array::new(ValueOrBuffer::Value(vec_to_test.clone()));

        let filled_buff = filled_encoding.buff.unwrap();
        let filled_decoding = Array::new(ValueOrBuffer::Buffer(filled_buff));

        assert_eq!(
            filled_decoding.value_of.unwrap(),
            filled_encoding.value_of.unwrap()
        );
    }

    #[test]
    fn test_empty_encoding_decoding() {
        let empty_encoding = Array::new(ValueOrBuffer::Value(vec![]));
        let empty_buff = empty_encoding.buff.unwrap();
        let empty_decoding = Array::new(ValueOrBuffer::Buffer(empty_buff));

        assert_eq!(
            empty_decoding.value_of.unwrap(),
            empty_encoding.value_of.unwrap()
        );
    }

    #[test]
    fn wrong_buffer() {
        let buff = vec![b'?', START_RECORD, 0, END_RECORD, BUFFER_END];

        let wrong = Array::new(ValueOrBuffer::Buffer(buff));

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
