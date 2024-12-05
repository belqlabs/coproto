use crate::commom::{
    delimiters::{BUFFER_END, END_RECORD, START_RECORD},
    errors::{decoding_error, DecodingError, DecodingErrors, TypeResult},
    join_parts, slice_records, BuffPart, CoprotoType, Uint8Buff,
};

#[derive(Debug)]
pub struct Boolean {
    pub first_byte: u8,
    pub modifier_byte: Option<u8>,
    pub modifier_char: Option<char>,
    pub first_char: char,
    pub value_of: TypeResult<bool>,
    pub buff: TypeResult<Uint8Buff>,
}

impl CoprotoType<bool> for Boolean {
    const FIRST_BYTE: u8 = b'#';
    fn new(value: crate::commom::ValueOrBuffer<bool>) -> Self {
        match value {
            crate::commom::ValueOrBuffer::Value(v) => Self {
                first_byte: b'#',
                modifier_byte: None,
                modifier_char: None,
                first_char: '#',
                value_of: Ok(v),
                buff: Self::encode(v),
            },
            crate::commom::ValueOrBuffer::Buffer(vec) => Self {
                first_byte: b'#',
                modifier_byte: None,
                modifier_char: None,
                first_char: '#',
                value_of: Self::decode(vec.clone()),
                buff: Ok(vec),
            },
        }
    }

    fn encode(value: bool) -> TypeResult<Uint8Buff> {
        let val: u8 = if value { 1 } else { 0 };

        let parts: Vec<BuffPart> = vec![
            BuffPart::Val(b'#'),
            BuffPart::Val(START_RECORD),
            BuffPart::Val(val),
            BuffPart::Val(END_RECORD),
            BuffPart::Val(BUFFER_END),
        ];

        Ok(join_parts(parts))
    }

    fn decode(value: Uint8Buff) -> TypeResult<bool> {
        let mut m_value = value.clone();

        let first_byte = m_value.remove(0);

        if first_byte != b'#' {
            return Err(decoding_error(DecodingError {
                from: value,
                to: "Boolean".to_string(),
                cause: DecodingErrors::FirstByteError("Boolean".to_string(), b'#', first_byte),
            }));
        };

        let records = slice_records(m_value.clone());

        if records.len() > 1 {
            return Err(decoding_error(DecodingError::new(
                value,
                "Double",
                DecodingErrors::TooMuch(
                    "Records".to_string(),
                    1,
                    records.len().try_into().unwrap(),
                ),
            )));
        };

        let val = match records.first() {
            Some(v_arr) => match v_arr.first() {
                Some(v) => v,
                None => {
                    return Err(decoding_error(DecodingError {
                        from: value.clone(),
                        to: "Boolean".to_string(),
                        cause: DecodingErrors::NotEnough(
                            "Bytes".to_string(),
                            4,
                            value.len().try_into().unwrap(),
                        ),
                    }))
                }
            },
            None => {
                return Err(decoding_error(DecodingError {
                    from: value.clone(),
                    to: "Boolean".to_string(),
                    cause: DecodingErrors::NotEnough(
                        "Bytes".to_string(),
                        4,
                        value.len().try_into().unwrap(),
                    ),
                }))
            }
        };

        match val {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(decoding_error(DecodingError::new(
                value,
                "Double",
                DecodingErrors::InvalidByte(*val, 0, vec![1, 0]),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commom::{
            delimiters::{BUFFER_END, END_RECORD, START_RECORD},
            join_parts, BuffPart, CoprotoType, ValueOrBuffer,
        },
        types::Boolean,
    };

    #[test]
    fn test_encoding_decoding() {
        let true_encoding = Boolean::new(ValueOrBuffer::Value(true));
        let true_buff = true_encoding.buff.unwrap();
        let true_decoding = Boolean::new(ValueOrBuffer::Buffer(true_buff));

        assert_eq!(
            true_decoding.value_of.unwrap(),
            true_encoding.value_of.unwrap()
        );

        let false_encoding = Boolean::new(ValueOrBuffer::Value(false));
        let false_buff = false_encoding.buff.unwrap();
        let false_decoding = Boolean::new(ValueOrBuffer::Buffer(false_buff));

        assert_eq!(
            false_decoding.value_of.unwrap(),
            false_encoding.value_of.unwrap()
        );
    }

    #[test]
    fn test_invalid_byte() {
        let parts: Vec<BuffPart> = vec![
            BuffPart::Val(b'#'),
            BuffPart::Val(START_RECORD),
            BuffPart::Val(2),
            BuffPart::Val(END_RECORD),
            BuffPart::Val(BUFFER_END),
        ];

        let invalid_byte = Boolean::new(ValueOrBuffer::Buffer(join_parts(parts)));

        match invalid_byte.value_of {
            Ok(_) => false,
            Err(e) => match e {
                crate::commom::errors::TypeError::Encoding(_) => false,
                crate::commom::errors::TypeError::Decoding(decoding_error) => {
                    matches!(
                        decoding_error.cause,
                        crate::commom::errors::DecodingErrors::InvalidByte(_, _, _)
                    )
                }
            },
        };
    }

    #[test]
    fn wrong_buffer() {
        let buff = vec![b'?', START_RECORD, 0, END_RECORD, BUFFER_END];

        let wrong = Boolean::new(ValueOrBuffer::Buffer(buff));

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
