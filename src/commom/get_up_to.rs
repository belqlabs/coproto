use super::Uint8Buff;

pub fn get_up_to(buffer: Uint8Buff, target: u8) -> Option<(usize, Uint8Buff)> {
    for (i, byte) in buffer.iter().enumerate() {
        if *byte == target {
            return Some((i, buffer[0..i].to_vec()));
        };
    }

    None
}
