use super::{
    delimiters::{END_RECORD, START_RECORD, VALUE_DELIMITER},
    Uint8Buff,
};

pub fn slice_records(buff: Uint8Buff) -> Vec<Uint8Buff> {
    let mut records: Vec<Uint8Buff> = vec![];

    let mut record: Uint8Buff = vec![];

    for byte in buff.iter() {
        if *byte == START_RECORD {
            continue;
        }

        if (*byte == END_RECORD || *byte == VALUE_DELIMITER) && !record.is_empty() {
            records.extend_from_slice(&[record.clone()]);
            record = vec![];
        }

        record.push(*byte);
    }

    return records;
}
