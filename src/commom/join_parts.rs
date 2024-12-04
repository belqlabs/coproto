use super::Uint8Buff;

pub enum BuffPart {
    Arr(Vec<u8>),
    Val(u8),
}

pub fn join_parts(parts: Vec<BuffPart>) -> Uint8Buff {
    let mut joined: Uint8Buff = vec![];

    for part in parts.iter() {
        match part {
            BuffPart::Arr(arr_part) => {
                joined.extend_from_slice(&**arr_part);
            }
            BuffPart::Val(v) => {
                joined.push(*v);
            }
        }
    }

    joined
}
