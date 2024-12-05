use crate::{
    commom::{
        delimiters::{BUFFER_END, END_RECORD, START_RECORD, VALUE_DELIMITER},
        errors::{
            decoding_error, encoding_error, DecodingError, DecodingErrors, EncodingError,
            EncodingErrors, TypeResult,
        },
        join_parts, slice_records, split_values, BuffPart, CoprotoType, Uint8Buff, ValueOrBuffer,
    },
    types::{infer_buffer, primitive, BigInt, Boolean, Double, Integer, Null, SupportedTypes},
};

#[derive(Debug)]
pub struct Table {
    pub first_byte: u8,
    pub modifier_byte: Option<u8>,
    pub modifier_char: Option<char>,
    pub first_char: char,
    pub value_of: TypeResult<(Vec<String>, Vec<Vec<SupportedTypes>>)>,
    pub buff: TypeResult<Uint8Buff>,
}

impl CoprotoType<(Vec<String>, Vec<Vec<SupportedTypes>>)> for Table {
    const FIRST_BYTE: u8 = b'{';

    fn new(value: ValueOrBuffer<(Vec<String>, Vec<Vec<SupportedTypes>>)>) -> Self {
        todo!()
    }

    fn encode(value: (Vec<String>, Vec<Vec<SupportedTypes>>)) -> TypeResult<Uint8Buff> {
        let mut parts: Vec<BuffPart> =
            vec![BuffPart::Val(Self::FIRST_BYTE), BuffPart::Val(START_RECORD)];

        let headers = value.0;

        if headers.is_empty() {
            return Err(encoding_error(EncodingError::new(
                "[]",
                "Table",
                EncodingErrors::InvalidValue("Table headers cannot be empty".to_string()),
            )));
        }

        let rows = value.1;

        for header in headers.iter() {
            let mut encoded_str = crate::types::String::encode(header.to_string())?;
            encoded_str.pop();
            parts.push(BuffPart::Val(START_RECORD));
            parts.push(BuffPart::Arr(encoded_str));
            parts.push(BuffPart::Val(END_RECORD));
        }

        parts.push(BuffPart::Val(END_RECORD));
        parts.push(BuffPart::Val(VALUE_DELIMITER));

        parts.push(BuffPart::Val(START_RECORD));
        parts.push(BuffPart::Arr(Null::encode(None)?));
        parts.push(BuffPart::Val(END_RECORD));

        for row in rows.iter() {
            if row.len() != headers.len() {
                return Err(encoding_error(EncodingError::new(
                    "(Vec<String>, Vec<Vec<SupportedTypes>>)",
                    "Table",
                    EncodingErrors::DecodedWouldOverflow,
                )));
            }

            parts.push(BuffPart::Val(START_RECORD));

            for row_data in row.iter() {
                parts.push(BuffPart::Val(START_RECORD));
                match row_data {
                    SupportedTypes::BigInt(bi) => {
                        let mut encoded = BigInt::encode(*bi)?;
                        encoded.pop();
                        parts.push(BuffPart::Arr(encoded));
                    }
                    SupportedTypes::Boolean(bol) => {
                        let mut encoded = Boolean::encode(*bol)?;
                        encoded.pop();
                        parts.push(BuffPart::Arr(encoded));
                    }
                    SupportedTypes::Double(db) => {
                        let mut encoded = Double::encode(*db)?;
                        encoded.pop();
                        parts.push(BuffPart::Arr(encoded));
                    }
                    SupportedTypes::Integer(int) => {
                        let mut encoded = Integer::encode(*int)?;
                        encoded.pop();
                        parts.push(BuffPart::Arr(encoded));
                    }
                    SupportedTypes::Null(null) => {
                        let mut encoded = Null::encode(*null)?;
                        encoded.pop();
                        parts.push(BuffPart::Arr(encoded));
                    }
                    SupportedTypes::String(str) => {
                        let mut encoded = crate::types::String::encode(str.to_string())?;
                        encoded.pop();
                        parts.push(BuffPart::Arr(encoded));
                    }
                }
                parts.push(BuffPart::Val(END_RECORD));
            }

            parts.push(BuffPart::Val(END_RECORD));
            parts.push(BuffPart::Val(VALUE_DELIMITER));
        }

        Ok(join_parts(parts))
    }

    fn decode(value: Uint8Buff) -> TypeResult<(Vec<String>, Vec<Vec<SupportedTypes>>)> {
        let mut m_value = value.clone();

        let first_byte = m_value.remove(0);

        if first_byte != Self::FIRST_BYTE {
            return Err(decoding_error(DecodingError {
                from: value,
                to: "Table".to_string(),
                cause: DecodingErrors::FirstByteError(
                    "Table".to_string(),
                    Self::FIRST_BYTE,
                    first_byte,
                ),
            }));
        };

        let records = slice_records(m_value);

        let mut headers: Vec<String> = vec![];

        let mut rows: Vec<Vec<SupportedTypes>> = vec![];

        let mut found_null = false;

        let mut headers_len: usize = 0;

        let mut row: Vec<SupportedTypes> = vec![];

        // This is shitty code
        fn insert_where_it_belongs(
            val: SupportedTypes,
            original_value: Uint8Buff,
            f_n: &mut bool,
            r: &mut Vec<SupportedTypes>,
            h: &mut Vec<String>,
        ) -> TypeResult<()> {
            match val {
                SupportedTypes::Null(_) => {
                    if *f_n == false {
                        *f_n = true;
                    } else {
                        r.push(val);
                    }
                    Ok(())
                }
                SupportedTypes::String(ref str) => {
                    if *f_n == false {
                        h.push(str.to_string());
                    } else {
                        r.push(val);
                    }
                    Ok(())
                }
                _ => {
                    if *f_n {
                        r.push(val);
                        Ok(())
                    } else {
                        Err(decoding_error(DecodingError::new(
                            original_value.clone(),
                            "Table",
                            DecodingErrors::InvalidTypeInCompositeType(
                                "String or Null".to_string(),
                                val.get_name().to_string(),
                            ),
                        )))
                    }
                }
            }
        }

        for record in records.iter() {
            let infered = infer_buffer(record.to_vec())?;

            insert_where_it_belongs(
                infered,
                value.clone(),
                &mut found_null,
                &mut row,
                &mut headers,
            )?;

            if found_null && headers_len == 0 {
                headers_len = headers.len();
                continue;
            }

            if row.len() == headers_len && headers_len != 0 {
                rows.extend_from_slice(&[row.to_vec()]);
                row = vec![];
            }
        }

        if !row.is_empty() {
            return Err(decoding_error(DecodingError::new(
                value,
                "Table",
                DecodingErrors::CantFitValues(format!(
                    "The table has {} values. But a row was found to have only {} values.",
                    headers_len,
                    row.len()
                )),
            )));
        }

        println!("Headers:\n{:?}", headers);
        println!("Rows:");
        for r in rows.iter() {
            println!("{:?}", r);
        }

        Ok((headers, rows))
    }
}

#[cfg(test)]
mod tests {
    use crate::{commom::CoprotoType, types::SupportedTypes};

    use super::Table;

    #[test]
    fn encoding_decoding() {
        let original_table = (
            vec!["Teste1".to_string(), "Teste2".to_string()],
            vec![vec![
                SupportedTypes::Boolean(false),
                SupportedTypes::Integer(10),
            ]],
        );

        let encoded = Table::encode(original_table.clone()).unwrap();

        println!("Encoded:\n{:?}", encoded);

        let decoded = Table::decode(encoded).unwrap();

        assert_eq!(original_table, decoded)
    }
}
