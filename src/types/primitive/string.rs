use crate::commom::{
    delimiters::{BUFFER_END, END_RECORD, START_RECORD},
    errors::{decoding_error, DecodingError, DecodingErrors, TypeResult},
    join_parts, slice_records, BuffPart, CoprotoType, Uint8Buff,
};

#[derive(Debug)]
pub struct String {
    pub first_byte: u8,
    pub modifier_byte: Option<u8>,
    pub modifier_char: Option<char>,
    pub first_char: char,
    pub value_of: TypeResult<std::string::String>,
    pub buff: TypeResult<Uint8Buff>,
}

impl CoprotoType<std::string::String> for String {
    fn new(value: crate::commom::ValueOrBuffer<std::string::String>) -> Self {
        match value {
            crate::commom::ValueOrBuffer::Value(v) => Self {
                first_byte: b'-',
                modifier_byte: None,
                modifier_char: None,
                first_char: '-',
                value_of: Ok(v.clone()),
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

    fn encode(value: std::string::String) -> TypeResult<Uint8Buff> {
        let value_buff = value.into_bytes();

        let parts: Vec<BuffPart> = vec![
            BuffPart::Val(b'+'),
            BuffPart::Val(START_RECORD),
            BuffPart::Arr(value_buff),
            BuffPart::Val(END_RECORD),
            BuffPart::Val(BUFFER_END),
        ];

        Ok(join_parts(parts))
    }

    fn decode(value: Uint8Buff) -> TypeResult<std::string::String> {
        let mut m_value = value.clone();

        let first_byte = m_value.remove(0);

        if first_byte != b'+' {
            return Err(decoding_error(DecodingError {
                from: value,
                to: "String".to_string(),
                cause: DecodingErrors::FirstByteError("String".to_string(), b'+', first_byte),
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
            Some(buff) => match std::string::String::from_utf8(buff.to_vec()) {
                Ok(string) => Ok(string),
                Err(e) => Err(decoding_error(DecodingError::new(
                    value,
                    "String",
                    DecodingErrors::InternalError(Box::new(e)),
                ))),
            },
            None => Err(decoding_error(DecodingError::new(
                value,
                "String",
                DecodingErrors::NotEnough("Records".to_string(), 1, 0),
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
        types::String,
    };

    #[test]
    fn test_encoding_decoding() {
        let string_encoding =
            String::new(ValueOrBuffer::Value("Hello, fellow rustacean!".to_string()));
        let string_buff = string_encoding.buff.unwrap();
        let string_decoding = String::new(ValueOrBuffer::Buffer(string_buff));

        assert_eq!(
            string_decoding.value_of.unwrap(),
            string_encoding.value_of.unwrap()
        );
    }

    #[test]
    fn wrong_buffer() {
        let buff = vec![b'?', START_RECORD, 0, END_RECORD, BUFFER_END];

        let wrong = String::new(ValueOrBuffer::Buffer(buff));

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
