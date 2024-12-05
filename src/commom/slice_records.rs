use super::{
    delimiters::{END_RECORD, START_RECORD, VALUE_DELIMITER},
    is_known_first_byte, Uint8Buff,
};

fn trim_record_delimiters(buff: &mut Uint8Buff) -> Option<&mut Uint8Buff> {
    let first_byte = buff.remove(0);

    let last_byte = match buff.pop() {
        Some(lb) => lb,
        None => return None,
    };

    if first_byte != START_RECORD || last_byte != END_RECORD {
        return None;
    };

    Some(buff)
}

fn inspect_record(buff: &mut Uint8Buff) -> Option<Uint8Buff> {
    if buff.is_empty() {
        return None;
    }

    trim_record_delimiters(buff);

    let next_byte = match buff.first() {
        Some(nb) => nb,
        None => return None,
    };

    match is_known_first_byte(*next_byte) {
        true => Some(buff.to_vec()),
        false => None,
    }
}

fn slicer_must_inspect_records(buff: &Uint8Buff) -> bool {
    buff.contains(&VALUE_DELIMITER)
}

pub fn slice_records(buff: Uint8Buff) -> Vec<Uint8Buff> {
    let should_inspect = slicer_must_inspect_records(&buff);

    let mut records: Vec<Uint8Buff> = vec![];

    for (idx, byte) in buff.iter().enumerate() {
        if *byte == START_RECORD {
            let mut should_ignore_next = false;

            for (n_idx, n_byte) in buff[idx + 1..buff.len()].iter().enumerate() {
                if *n_byte == START_RECORD {
                    should_ignore_next = true;
                    continue;
                }

                if *n_byte == END_RECORD {
                    if !should_ignore_next {
                        let mut found_record = buff[idx..=n_idx + idx + 1].to_vec();

                        if should_inspect {
                            if let Some(b) = inspect_record(&mut found_record) {
                                records.push(b)
                            }

                            break;
                        }

                        if let Some(fr) = trim_record_delimiters(&mut found_record) {
                            records.push(fr.to_vec())
                        };

                        break;
                    }

                    should_ignore_next = false;
                }
            }

            continue;
        }
    }

    records
}

#[cfg(test)]
mod tests {
    use crate::commom::{
        delimiters::{BUFFER_END, END_RECORD, START_RECORD, VALUE_DELIMITER},
        slice_records, Uint8Buff,
    };

    #[test]
    fn primitive_types_buffer() {
        let vec_to_test = vec![START_RECORD, 1, 2, 3, 4, 5, 6, 7, 8, 9, END_RECORD];

        let vec_to_be: Uint8Buff = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let sliced = slice_records(vec_to_test);

        println!("Sliced:\n{:?}", sliced);

        assert_eq!(sliced[0], vec_to_be)
    }

    #[test]
    fn composite_types_buffer() {
        let vec_to_test = vec![
            START_RECORD,
            40,
            43,
            START_RECORD,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            END_RECORD,
            END_RECORD,
            VALUE_DELIMITER,
            START_RECORD,
            35,
            START_RECORD,
            0,
            END_RECORD,
            END_RECORD,
            VALUE_DELIMITER,
            START_RECORD,
            59,
            43,
            START_RECORD,
            1,
            46,
            2,
            3,
            4,
            5,
            END_RECORD,
            END_RECORD,
            VALUE_DELIMITER,
            START_RECORD,
            58,
            43,
            START_RECORD,
            1,
            2,
            3,
            END_RECORD,
            END_RECORD,
            VALUE_DELIMITER,
            START_RECORD,
            45,
            START_RECORD,
            END_RECORD,
            END_RECORD,
            VALUE_DELIMITER,
            START_RECORD,
            43,
            START_RECORD,
            72,
            101,
            108,
            108,
            111,
            44,
            32,
            102,
            101,
            108,
            108,
            111,
            119,
            32,
            114,
            117,
            115,
            116,
            97,
            99,
            101,
            97,
            110,
            33,
            END_RECORD,
            END_RECORD,
            VALUE_DELIMITER,
            BUFFER_END,
        ];

        let vec_to_be: Vec<Uint8Buff> = vec![
            [40, 43, START_RECORD, 1, 2, 3, 4, 5, 6, 7, 8, 9, END_RECORD].to_vec(),
            [35, START_RECORD, 0, END_RECORD].to_vec(),
            [59, 43, START_RECORD, 1, 46, 2, 3, 4, 5, END_RECORD].to_vec(),
            [58, 43, START_RECORD, 1, 2, 3, END_RECORD].to_vec(),
            [45, START_RECORD, END_RECORD].to_vec(),
            [
                43,
                START_RECORD,
                72,
                101,
                108,
                108,
                111,
                44,
                32,
                102,
                101,
                108,
                108,
                111,
                119,
                32,
                114,
                117,
                115,
                116,
                97,
                99,
                101,
                97,
                110,
                33,
                END_RECORD,
            ]
            .to_vec(),
        ];

        let sliced = slice_records(vec_to_test);

        println!("Sliced:\n{:?}", sliced);

        for (idx, _el) in sliced.iter().enumerate() {
            println!("Testing case {}", idx);
            assert_eq!(sliced[idx], vec_to_be[idx])
        }
    }
}
