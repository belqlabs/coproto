use crate::types::{Array, BigInt, Boolean, Command, Double, Integer, NamedValue, Null, Table};

use super::CoprotoType;

pub fn is_known_first_byte(byte: u8) -> bool {
    match byte {
        BigInt::FIRST_BYTE => true,
        Boolean::FIRST_BYTE => true,
        Double::FIRST_BYTE => true,
        Integer::FIRST_BYTE => true,
        Null::FIRST_BYTE => true,
        crate::types::String::FIRST_BYTE => true,
        Array::FIRST_BYTE => true,
        Command::FIRST_BYTE => true,
        NamedValue::FIRST_BYTE => true,
        Table::FIRST_BYTE => true,
        _ => false,
    }
}
