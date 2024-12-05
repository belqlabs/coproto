use crate::commom::{
    delimiters::{BUFFER_END, END_RECORD, START_RECORD},
    errors::{
        decoding_error, encoding_error, DecodingError, DecodingErrors, EncodingError, TypeResult,
    },
    join_parts, slice_records, BuffPart, CoprotoType, Uint8Buff,
};

#[derive(Debug)]
pub struct Null {
    pub first_byte: u8,
    pub modifier_byte: Option<u8>,
    pub modifier_char: Option<char>,
    pub first_char: char,
    pub value_of: TypeResult<Option<()>>,
    pub buff: TypeResult<Uint8Buff>,
}

impl CoprotoType<Option<()>> for Null {
    const FIRST_BYTE: u8 = b'-';
    fn new(value: crate::commom::ValueOrBuffer<Option<()>>) -> Self {
        match value {
            crate::commom::ValueOrBuffer::Value(v) => Self {
                first_byte: b'-',
                modifier_byte: None,
                modifier_char: None,
                first_char: '-',
                value_of: Ok(v),
                buff: Self::encode(v),
            },
            crate::commom::ValueOrBuffer::Buffer(vec) => Self {
                first_byte: b'-',
                modifier_byte: None,
                modifier_char: None,
                first_char: '-',
                value_of: Self::decode(vec.clone()),
                buff: Ok(vec),
            },
        }
    }

    fn encode(value: Option<()>) -> TypeResult<Uint8Buff> {
        if value.is_some() {
            return Err(encoding_error(EncodingError::new(
                "Some(())",
                "Null",
                encoding_error::EncodingErrors::InvalidValue(
                    "Value cannot be Some(()), only None".to_string(),
                ),
            )));
        }

        let parts: Vec<BuffPart> = vec![
            BuffPart::Val(b'-'),
            BuffPart::Val(START_RECORD),
            BuffPart::Val(END_RECORD),
            BuffPart::Val(BUFFER_END),
        ];

        Ok(join_parts(parts))
    }

    fn decode(value: Uint8Buff) -> TypeResult<Option<()>> {
        let mut m_value = value.clone();

        let first_byte = m_value.remove(0);

        if first_byte != b'-' {
            return Err(decoding_error(DecodingError {
                from: value,
                to: "Boolean".to_string(),
                cause: DecodingErrors::FirstByteError("Boolean".to_string(), b'-', first_byte),
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

        match records.first() {
            Some(v) => match v.is_empty() {
                true => Ok(None),
                false => Err(decoding_error(DecodingError::new(
                    value.clone(),
                    "Null",
                    DecodingErrors::TooMuch("Bytes".to_string(), 0, v.len().try_into().unwrap()),
                ))),
            },
            None => Ok(None),
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
        types::Null,
    };

    #[test]
    fn test_encoding_decoding() {
        let null_encoding = Null::new(ValueOrBuffer::Value(None));
        let null_buff = null_encoding.buff.unwrap();
        let null_decoding = Null::new(ValueOrBuffer::Buffer(null_buff));

        assert_eq!(
            null_decoding.value_of.unwrap(),
            null_encoding.value_of.unwrap()
        );
    }

    #[test]
    fn test_invalid_byte() {
        let parts: Vec<BuffPart> = vec![
            BuffPart::Val(b'('),
            BuffPart::Val(START_RECORD),
            BuffPart::Val(2),
            BuffPart::Val(END_RECORD),
            BuffPart::Val(BUFFER_END),
        ];

        let invalid_byte = Null::new(ValueOrBuffer::Buffer(join_parts(parts)));

        match invalid_byte.value_of {
            Ok(_) => false,
            Err(e) => match e {
                crate::commom::errors::TypeError::Encoding(_) => false,
                crate::commom::errors::TypeError::Decoding(decoding_error) => {
                    matches!(
                        decoding_error.cause,
                        crate::commom::errors::DecodingErrors::TooMuch(_, _, _)
                    )
                }
            },
        };
    }

    #[test]
    fn wrong_buffer() {
        let buff = vec![b'?', START_RECORD, 0, END_RECORD, BUFFER_END];

        let wrong = Null::new(ValueOrBuffer::Buffer(buff));

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
