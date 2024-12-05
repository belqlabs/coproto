use crate::commom::{
    delimiters::{BUFFER_END, END_RECORD, START_RECORD},
    errors::{
        decoding_error, encoding_error, DecodingError, DecodingErrors, EncodingError,
        EncodingErrors, TypeResult,
    },
    join_parts,
    modifiers::{MINUS, PLUS},
    slice_records, to_ascii_code, BuffPart, CoprotoType, Uint8Buff, ValueOrBuffer,
};

#[derive(Debug)]
pub struct BigInt {
    pub first_byte: u8,
    pub modifier_byte: Option<u8>,
    pub modifier_char: Option<char>,
    pub first_char: char,
    pub value_of: TypeResult<i64>,
    pub buff: TypeResult<Uint8Buff>,
}

impl CoprotoType<i64> for BigInt {
    const FIRST_BYTE: u8 = b'(';
    fn new(value: ValueOrBuffer<i64>) -> Self {
        match value {
            ValueOrBuffer::Value(value) => Self {
                first_byte: Self::FIRST_BYTE,
                first_char: '(',
                modifier_byte: Some(if value < 0 { b'-' } else { b'+' }),
                modifier_char: Some(if value < 0 { '-' } else { '+' }),
                value_of: Ok(value),
                buff: Self::encode(value),
            },
            ValueOrBuffer::Buffer(vec) => {
                let value = Self::decode(vec.clone());
                let modifier_char = match value {
                    Ok(value) => Some(if value < 0 { '-' } else { '+' }),
                    Err(_) => None,
                };

                let modifier_byte = match modifier_char {
                    Some(char) => match to_ascii_code(char) {
                        Ok(c) => Some(c),
                        Err(_) => todo!(),
                    },
                    None => None,
                };
                Self {
                    first_byte: b'(',
                    first_char: '(',
                    modifier_char,
                    modifier_byte,
                    value_of: value,
                    buff: Ok(vec.clone()),
                }
            }
        }
    }

    fn encode(value: i64) -> TypeResult<Uint8Buff> {
        let signal: u8 = if value < 0 { MINUS } else { PLUS };

        let mut abs_value: i64 = value.abs();

        let mut digits_arr: Vec<u8> = vec![];

        let mut digit: u8 = match (abs_value % 10).try_into() {
            Ok(digit) => digit,
            Err(e) => {
                return Err(encoding_error(EncodingError {
                    from: value.to_string(),
                    to: "u8 digit".to_string(),
                    origin: EncodingErrors::InternalError(Box::new(e)),
                }))
            }
        };

        while abs_value != 0 {
            digits_arr.insert(0, digit);

            abs_value /= 10;

            digit = match (abs_value % 10).try_into() {
                Ok(digit) => digit,
                Err(e) => {
                    return Err(encoding_error(EncodingError {
                        from: value.to_string(),
                        to: "u8 digit".to_string(),
                        origin: EncodingErrors::InternalError(Box::new(e)),
                    }))
                }
            };
        }

        let parts: Vec<BuffPart> = vec![
            BuffPart::Val(b'('),
            BuffPart::Val(signal),
            BuffPart::Val(START_RECORD),
            BuffPart::Arr(digits_arr),
            BuffPart::Val(END_RECORD),
            BuffPart::Val(BUFFER_END),
        ];

        Ok(join_parts(parts))
    }

    fn decode(value: Uint8Buff) -> TypeResult<i64> {
        let mut m_value = value.clone();

        let first_byte = m_value.remove(0);

        if first_byte != b'(' {
            return Err(decoding_error(DecodingError {
                from: value,
                to: "BigInt".to_string(),
                cause: DecodingErrors::FirstByteError("BigInt".to_string(), b'(', first_byte),
            }));
        };

        let signal = match m_value.first() {
            Some(b) => match b {
                b'-' => {
                    m_value.remove(0);
                    -1
                }
                b'+' => {
                    m_value.remove(0);
                    1
                }
                _ => 1,
            },
            None => {
                return Err(decoding_error(DecodingError {
                    from: value.clone(),
                    to: "BigInt".to_string(),
                    cause: DecodingErrors::NotEnough(
                        "Bytes".to_string(),
                        4,
                        value.len().try_into().unwrap(),
                    ),
                }))
            }
        };

        let records = slice_records(m_value.clone());

        if records.len() > 1 {
            return Err(decoding_error(DecodingError {
                from: value,
                to: "BigInt".to_string(),
                cause: DecodingErrors::TooMuch(
                    "Records".to_string(),
                    1,
                    records.len().try_into().unwrap(),
                ),
            }));
        };

        let digits: Vec<u8> = match records.first() {
            Some(entries) => entries.to_vec(),
            None => {
                return Err(decoding_error(DecodingError {
                    from: value.clone(),
                    to: "BigInt".to_string(),
                    cause: DecodingErrors::NotEnough(
                        "Bytes".to_string(),
                        4,
                        value.len().try_into().unwrap(),
                    ),
                }))
            }
        };

        let mut number: i64 = 0;

        let digits_count: i64 = match digits.len().try_into() {
            Ok(digits_count) => digits_count,
            Err(_) => {
                return Err(decoding_error(DecodingError {
                    cause: DecodingErrors::SizeConversionError(
                        digits.len().to_string(),
                        "i64".to_string(),
                    ),
                    from: value,
                    to: "BigInt".to_string(),
                }))
            }
        };

        for i in 0..digits_count {
            let index: usize = match (i).try_into() {
                Ok(index) => index,
                Err(_) => {
                    return Err(decoding_error(DecodingError {
                        from: value,
                        to: "BigInt".to_string(),
                        cause: DecodingErrors::SizeConversionError(
                            i.to_string(),
                            "usize".to_string(),
                        ),
                    }))
                }
            };

            let position: u32 = match (digits_count - i - 1).try_into() {
                Ok(position) => position,
                Err(_) => {
                    return Err(decoding_error(DecodingError {
                        from: value,
                        to: "BigInt".to_string(),
                        cause: DecodingErrors::SizeConversionError(
                            i.to_string(),
                            "usize".to_string(),
                        ),
                    }))
                }
            };

            let d_64: i64 = (digits[index]).into();

            let part = d_64 * (10i64.pow(position));

            number += part;
        }

        number *= signal;

        Ok(number)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commom::{
            delimiters::{BUFFER_END, END_RECORD, START_RECORD},
            CoprotoType, ValueOrBuffer,
        },
        types::BigInt,
    };

    #[test]
    fn test_positive() {
        let positive_encoding = BigInt::new(ValueOrBuffer::Value(1000));

        let positive_encoding_buff = positive_encoding.buff.unwrap();

        let positive_decoding = BigInt::new(ValueOrBuffer::Buffer(positive_encoding_buff.clone()));

        let positive_decoding_buff = positive_decoding.buff.unwrap();

        assert_eq!(
            positive_encoding_buff,
            vec![b'(', b'+', START_RECORD, 1, 0, 0, 0, END_RECORD, BUFFER_END]
        );

        assert_eq!(positive_encoding.value_of.unwrap(), 1000);

        assert_eq!(
            positive_decoding_buff,
            vec![b'(', b'+', START_RECORD, 1, 0, 0, 0, END_RECORD, BUFFER_END]
        );

        assert_eq!(positive_decoding.value_of.unwrap(), 1000);
    }

    #[test]
    fn no_signal() {
        let positive_decoding = BigInt::new(ValueOrBuffer::Buffer(vec![
            b'(',
            START_RECORD,
            1,
            0,
            0,
            0,
            END_RECORD,
            BUFFER_END,
        ]));
        assert_eq!(positive_decoding.value_of.unwrap(), 1000);
    }

    #[test]
    fn test_negative() {
        let negative_encoding = BigInt::new(ValueOrBuffer::Value(-1000));

        let negative_encoding_buff = negative_encoding.buff.unwrap();

        let negative_decoding = BigInt::new(ValueOrBuffer::Buffer(negative_encoding_buff.clone()));

        let negative_decoding_buff = negative_decoding.buff.unwrap();

        assert_eq!(
            negative_encoding_buff,
            vec![b'(', b'-', START_RECORD, 1, 0, 0, 0, END_RECORD, BUFFER_END]
        );

        assert_eq!(negative_encoding.value_of.unwrap(), -1000);

        assert_eq!(
            negative_decoding_buff,
            vec![b'(', b'-', START_RECORD, 1, 0, 0, 0, END_RECORD, BUFFER_END]
        );

        assert_eq!(negative_decoding.value_of.unwrap(), -1000);
    }

    #[test]
    fn test_encoding_decoding() {
        let positive_encoding = BigInt::new(ValueOrBuffer::Value(1000));
        let positive_buff = positive_encoding.buff.unwrap();
        let positive_decoding = BigInt::new(ValueOrBuffer::Buffer(positive_buff));

        assert_eq!(
            positive_decoding.value_of.unwrap(),
            positive_encoding.value_of.unwrap()
        );

        let negative_encoding = BigInt::new(ValueOrBuffer::Value(-1000));
        let negative_buff = negative_encoding.buff.unwrap();
        let negative_decoding = BigInt::new(ValueOrBuffer::Buffer(negative_buff));

        assert_eq!(
            negative_decoding.value_of.unwrap(),
            negative_encoding.value_of.unwrap()
        );
    }

    #[test]
    fn wrong_buffer() {
        let buff = vec![b'?', START_RECORD, 1, 0, 0, 0, END_RECORD, BUFFER_END];

        let wrong = BigInt::new(ValueOrBuffer::Buffer(buff));

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
