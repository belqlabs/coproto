use crate::commom::{
    delimiters::{BUFFER_END, END_RECORD, START_RECORD},
    errors::{
        decoding_error, encoding_error, DecodingError, DecodingErrors, EncodingError,
        EncodingErrors, TypeError, TypeResult,
    },
    join_parts, slice_records, to_ascii_code, BuffPart, CoprotoType, Uint8Buff,
};

#[derive(Debug)]
pub struct Double {
    pub first_byte: u8,
    pub modifier_byte: Option<u8>,
    pub modifier_char: Option<char>,
    pub first_char: char,
    pub value_of: TypeResult<f64>,
    pub buff: TypeResult<Uint8Buff>,
}

impl CoprotoType<f64> for Double {
    fn new(value: crate::commom::ValueOrBuffer<f64>) -> Self {
        match value {
            crate::commom::ValueOrBuffer::Value(value) => Self {
                first_byte: b';',
                modifier_byte: Some(if value < 0f64 { b'-' } else { b'+' }),
                modifier_char: Some(if value < 0f64 { '-' } else { '+' }),
                first_char: ';',
                value_of: Ok(value),
                buff: Self::encode(value),
            },
            crate::commom::ValueOrBuffer::Buffer(vec) => {
                let value = Self::decode(vec.clone());
                let modifier_char = match value {
                    Ok(value) => Some(if value < 0f64 { '-' } else { '+' }),
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
                    first_byte: b';',
                    first_char: ';',
                    modifier_char,
                    modifier_byte,
                    value_of: value,
                    buff: Ok(vec.clone()),
                }
            }
        }
    }

    fn encode(value: f64) -> TypeResult<Uint8Buff> {
        let signal = if value > 0f64 { b'+' } else { b'-' };

        let binding = value.abs().to_string();

        let mut str_values = binding.split(".");

        let mut integer_part: i32 = match str_values.next() {
            Some(int_str) => match int_str.parse() {
                Ok(int) => int,
                Err(e) => {
                    return Err(encoding_error(EncodingError::new(
                        "String",
                        "Integer 32",
                        EncodingErrors::InternalError(Box::new(e)),
                    )))
                }
            },
            None => {
                return Err(encoding_error(EncodingError::new(
                    "Double",
                    "Integer Part",
                    EncodingErrors::InvalidValue(format!("Expected: double, received: {}", value)),
                )))
            }
        };

        let mut decimal_part: i32 = match str_values.next() {
            Some(dec_str) => match dec_str.parse() {
                Ok(dec) => dec,
                Err(e) => {
                    return Err(encoding_error(EncodingError::new(
                        "String",
                        "Integer 32",
                        EncodingErrors::InternalError(Box::new(e)),
                    )))
                }
            },
            None => {
                return Err(encoding_error(EncodingError::new(
                    "Double",
                    "Decimal Part",
                    EncodingErrors::InvalidValue(format!("Expected: double, received: {}", value)),
                )))
            }
        };

        let mut integer_buff: Vec<u8> = vec![];

        let mut integer_digit: u8 = match (integer_part % 10).try_into() {
            Ok(digit) => digit,
            Err(e) => {
                return Err(encoding_error(EncodingError {
                    from: value.to_string(),
                    to: "u8 digit".to_string(),
                    origin: EncodingErrors::InternalError(Box::new(e)),
                }))
            }
        };

        while integer_part != 0 {
            integer_buff.insert(0, integer_digit);

            integer_part /= 10;

            integer_digit = match (integer_part % 10).try_into() {
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

        let mut decimal_buff: Vec<u8> = vec![];

        let mut decimal_digit: u8 = match (decimal_part % 10).try_into() {
            Ok(digit) => digit,
            Err(e) => {
                return Err(encoding_error(EncodingError {
                    from: value.to_string(),
                    to: "u8 digit".to_string(),
                    origin: EncodingErrors::InternalError(Box::new(e)),
                }))
            }
        };

        while decimal_part != 0 {
            decimal_buff.insert(0, decimal_digit);

            decimal_part /= 10;

            decimal_digit = match (decimal_part % 10).try_into() {
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
            BuffPart::Val(b';'),
            BuffPart::Val(signal),
            BuffPart::Val(START_RECORD),
            BuffPart::Arr(integer_buff),
            BuffPart::Val(b'.'),
            BuffPart::Arr(decimal_buff),
            BuffPart::Val(END_RECORD),
            BuffPart::Val(BUFFER_END),
        ];

        Ok(join_parts(parts))
    }

    fn decode(value: Uint8Buff) -> TypeResult<f64> {
        let mut m_value = value.clone();

        let first_byte = m_value.remove(0);

        if first_byte != b';' {
            return Err(decoding_error(DecodingError::new(
                value,
                "Double",
                decoding_error::DecodingErrors::FirstByteError(
                    "Double".to_string(),
                    b';',
                    first_byte,
                ),
            )));
        };

        let signal = match m_value.first() {
            Some(b) => match b {
                b'-' => {
                    m_value.remove(0);
                    -1f64
                }
                b'+' => {
                    m_value.remove(0);
                    1f64
                }
                _ => 1f64,
            },
            None => {
                return Err(decoding_error(DecodingError {
                    from: value.clone(),
                    to: "Integer".to_string(),
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
                to: "Double".to_string(),
                cause: DecodingErrors::TooMuch(
                    "Records".to_string(),
                    1,
                    records.len().try_into().unwrap(),
                ),
            }));
        }

        let digits: Vec<u8> = match records.first() {
            Some(entries) => entries.to_vec(),
            None => {
                return Err(decoding_error(DecodingError {
                    from: value.clone(),
                    to: "Integer".to_string(),
                    cause: DecodingErrors::NotEnough(
                        "Bytes".to_string(),
                        4,
                        value.len().try_into().unwrap(),
                    ),
                }))
            }
        };

        let mut number = String::new();

        let digits_count: i64 = match digits.len().try_into() {
            Ok(digits_count) => digits_count,
            Err(_) => {
                return Err(decoding_error(DecodingError {
                    cause: DecodingErrors::SizeConversionError(
                        digits.len().to_string(),
                        "i64".to_string(),
                    ),
                    from: value,
                    to: "Integer".to_string(),
                }))
            }
        };

        for i in 0..digits_count {
            let index: usize = match (i).try_into() {
                Ok(index) => index,
                Err(_) => {
                    return Err(decoding_error(DecodingError {
                        from: value,
                        to: "Integer".to_string(),
                        cause: DecodingErrors::SizeConversionError(
                            i.to_string(),
                            "usize".to_string(),
                        ),
                    }))
                }
            };

            if digits[index] != b'.' {
                number += (digits[index]).to_string().as_str()
            } else {
                number += "."
            };
        }

        let abs_value: f64 = match number.parse() {
            Ok(value) => value,
            Err(e) => {
                return Err(decoding_error(DecodingError::new(
                    value,
                    "Double",
                    DecodingErrors::InternalError(Box::new(e)),
                )))
            }
        };

        Ok(abs_value * signal)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commom::{
            delimiters::{BUFFER_END, END_RECORD, START_RECORD},
            CoprotoType, ValueOrBuffer,
        },
        types::Double,
    };

    #[test]
    fn test_positive() {
        let positive_encoding = Double::new(ValueOrBuffer::Value(123.456));

        let positive_encoding_buff = positive_encoding.buff.unwrap();

        let positive_decoding = Double::new(ValueOrBuffer::Buffer(positive_encoding_buff.clone()));

        let positive_decoding_buff = positive_decoding.buff.unwrap();

        assert_eq!(
            positive_encoding_buff,
            vec![
                b';',
                b'+',
                START_RECORD,
                1,
                2,
                3,
                b'.',
                4,
                5,
                6,
                END_RECORD,
                BUFFER_END
            ]
        );

        assert_eq!(positive_encoding.value_of.unwrap(), 123.456);

        assert_eq!(
            positive_decoding_buff,
            vec![
                b';',
                b'+',
                START_RECORD,
                1,
                2,
                3,
                b'.',
                4,
                5,
                6,
                END_RECORD,
                BUFFER_END
            ]
        );

        assert_eq!(positive_decoding.value_of.unwrap(), 123.456);
    }

    #[test]
    fn no_signal() {
        let positive_decoding = Double::new(ValueOrBuffer::Buffer(vec![
            b';',
            START_RECORD,
            1,
            2,
            3,
            b'.',
            4,
            5,
            6,
            END_RECORD,
            BUFFER_END,
        ]));
        assert_eq!(positive_decoding.value_of.unwrap(), 123.456);
    }

    #[test]
    fn test_negative() {
        let negative_encoding = Double::new(ValueOrBuffer::Value(-123.456));

        let negative_encoding_buff = negative_encoding.buff.unwrap();

        let negative_decoding = Double::new(ValueOrBuffer::Buffer(negative_encoding_buff.clone()));

        let negative_decoding_buff = negative_decoding.buff.unwrap();

        assert_eq!(
            negative_encoding_buff,
            vec![
                b';',
                b'-',
                START_RECORD,
                1,
                2,
                3,
                b'.',
                4,
                5,
                6,
                END_RECORD,
                BUFFER_END
            ]
        );

        assert_eq!(negative_encoding.value_of.unwrap(), -123.456);

        assert_eq!(
            negative_decoding_buff,
            vec![
                b';',
                b'-',
                START_RECORD,
                1,
                2,
                3,
                b'.',
                4,
                5,
                6,
                END_RECORD,
                BUFFER_END
            ]
        );

        assert_eq!(negative_decoding.value_of.unwrap(), -123.456);
    }

    #[test]
    fn test_encoding_decoding() {
        let positive_encoding = Double::new(ValueOrBuffer::Value(123.456));
        let positive_buff = positive_encoding.buff.unwrap();
        let positive_decoding = Double::new(ValueOrBuffer::Buffer(positive_buff));

        assert_eq!(
            positive_decoding.value_of.unwrap(),
            positive_encoding.value_of.unwrap()
        );

        let negative_encoding = Double::new(ValueOrBuffer::Value(-123.456));
        let negative_buff = negative_encoding.buff.unwrap();
        let negative_decoding = Double::new(ValueOrBuffer::Buffer(negative_buff));

        assert_eq!(
            negative_decoding.value_of.unwrap(),
            negative_encoding.value_of.unwrap()
        );
    }

    #[test]
    fn wrong_buffer() {
        let buff = vec![
            b'?',
            START_RECORD,
            1,
            2,
            3,
            b'.',
            4,
            5,
            6,
            END_RECORD,
            BUFFER_END,
        ];

        let wrong = Double::new(ValueOrBuffer::Buffer(buff));

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
