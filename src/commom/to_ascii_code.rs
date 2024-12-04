use crate::commom::errors::DecodingResult;

pub fn to_ascii_code(c: char) -> DecodingResult<u8> {
    match c {
        ':' => Ok(b':'),
        ';' => Ok(b';'),
        '+' => Ok(b'+'),
        '-' => Ok(b'-'),
        _ => todo!(),
    }
}
