use crate::{
    commom::{
        delimiters::{BUFFER_END, END_RECORD, START_RECORD, VALUE_DELIMITER},
        errors::{decoding_error, DecodingError, DecodingErrors, TypeResult},
        join_parts, slice_records, split_values, BuffPart, CoprotoType, Uint8Buff, ValueOrBuffer,
    },
    types::{infer_buffer, SupportedTypes},
};

#[derive(Debug)]
pub struct Command {
    pub first_byte: u8,
    pub modifier_byte: Option<u8>,
    pub modifier_char: Option<char>,
    pub first_char: char,
    pub value_of: TypeResult<(String, Vec<String>)>,
    pub buff: TypeResult<Uint8Buff>,
}

impl CoprotoType<(String, Vec<String>)> for Command {
    const FIRST_BYTE: u8 = b'$';

    fn new(value: ValueOrBuffer<(String, Vec<String>)>) -> Self {
        match value {
            ValueOrBuffer::Value(v) => Self {
                first_byte: Self::FIRST_BYTE,
                modifier_byte: None,
                modifier_char: None,
                first_char: '$',
                value_of: Ok(v.clone()),
                buff: Self::encode(v),
            },
            ValueOrBuffer::Buffer(vec) => Self {
                first_byte: Self::FIRST_BYTE,
                modifier_byte: None,
                modifier_char: None,
                first_char: '$',
                value_of: Self::decode(vec.clone()),
                buff: Ok(vec),
            },
        }
    }

    fn encode(value: (String, Vec<String>)) -> TypeResult<Uint8Buff> {
        let mut parts: Vec<BuffPart> =
            vec![BuffPart::Val(Self::FIRST_BYTE), BuffPart::Val(START_RECORD)];

        let mut command_name = crate::types::String::encode(value.0)?;

        command_name.pop();

        parts.push(BuffPart::Arr(command_name));

        parts.push(BuffPart::Val(VALUE_DELIMITER));

        for str in value.1.iter() {
            let mut encoded_string = crate::types::String::encode(str.clone())?;

            encoded_string.pop();

            parts.push(BuffPart::Arr(encoded_string));
            parts.push(BuffPart::Val(VALUE_DELIMITER));
        }

        parts.push(BuffPart::Val(END_RECORD));
        parts.push(BuffPart::Val(BUFFER_END));

        Ok(join_parts(parts))
    }

    fn decode(value: Uint8Buff) -> TypeResult<(String, Vec<String>)> {
        let mut m_value = value.clone();

        let first_byte = m_value.remove(0);

        if first_byte != Self::FIRST_BYTE {
            return Err(decoding_error(DecodingError {
                from: value,
                to: "Command".to_string(),
                cause: DecodingErrors::FirstByteError(
                    "Command".to_string(),
                    Self::FIRST_BYTE,
                    first_byte,
                ),
            }));
        };

        let records = slice_records(m_value.clone());

        if records.len() > 1 {
            return Err(decoding_error(DecodingError::new(
                value,
                "Command",
                DecodingErrors::TooMuch(
                    "Records".to_string(),
                    1,
                    records.len().try_into().unwrap(),
                ),
            )));
        };

        let command_record = match records.first() {
            Some(v_arr) => v_arr,
            None => {
                return Err(decoding_error(DecodingError {
                    from: value.clone(),
                    to: "Command".to_string(),
                    cause: DecodingErrors::NotEnough(
                        "Bytes".to_string(),
                        4,
                        value.len().try_into().unwrap(),
                    ),
                }))
            }
        };

        let values = split_values(command_record.clone());

        let name = match values.first() {
            Some(v) => crate::types::String::decode(v.to_vec())?,
            None => {
                return Err(decoding_error(DecodingError::new(
                    value,
                    "Command",
                    DecodingErrors::NotEnough(
                        "Values".to_string(),
                        2,
                        values.len().try_into().unwrap(),
                    ),
                )))
            }
        };

        let args_slice = (values[1..]).to_vec();

        let mut coproto_args: Vec<String> = vec![];

        for arg_str in args_slice.iter() {
            let infered = infer_buffer(arg_str.to_vec())?;

            match infered {
                SupportedTypes::String(str) => coproto_args.push(str.clone()),
                _ => {
                    return Err(decoding_error(DecodingError::new(
                        value,
                        "Command",
                        DecodingErrors::InvalidTypeInCompositeType(
                            infered.get_name().to_string(),
                            "String".to_string(),
                        ),
                    )))
                }
            }
        }

        Ok((name, coproto_args))
    }
}

#[cfg(test)]
mod tests {

    use crate::commom::{
        delimiters::{BUFFER_END, END_RECORD, START_RECORD},
        CoprotoType, ValueOrBuffer,
    };

    #[test]
    fn test_encoding_decoding() {
        let encoding = crate::types::Command::new(ValueOrBuffer::Value((
            "OK".to_string(),
            vec![
                "Be".to_string(),
                "happy".to_string(),
                ".".to_string(),
                "Its".to_string(),
                "all".to_string(),
                "ours".to_string(),
            ],
        )));

        let buff = encoding.buff.unwrap();

        let decoding = crate::types::Command::new(ValueOrBuffer::Buffer(buff));

        assert_eq!(decoding.value_of.unwrap(), encoding.value_of.unwrap());
    }

    #[test]
    fn wrong_buffer() {
        let buff = vec![b'?', START_RECORD, 0, END_RECORD, BUFFER_END];

        let wrong = crate::types::Command::new(ValueOrBuffer::Buffer(buff));

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
