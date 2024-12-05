use super::{delimiters::VALUE_DELIMITER, Uint8Buff};

pub fn split_values(buff: Uint8Buff) -> Vec<Uint8Buff> {
    let mut values: Vec<Uint8Buff> = vec![];

    let mut value: Uint8Buff = vec![];

    for byte in buff.iter() {
        if *byte == VALUE_DELIMITER {
            values.extend_from_slice(&[value.clone()]);
            value = vec![];
            continue;
        }

        value.push(*byte);
    }

    values
}
