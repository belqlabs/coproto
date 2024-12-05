use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq)]
pub enum SupportedTypes {
    BigInt(i64),
    Boolean(bool),
    Double(f64),
    Integer(i32),
    Null(Option<()>),
    String(std::string::String),
}

impl SupportedTypes {
    pub fn get_name(&self) -> &str {
        match self {
            SupportedTypes::BigInt(_) => "BigInt",
            SupportedTypes::Boolean(_) => "Boolean",
            SupportedTypes::Double(_) => "Double",
            SupportedTypes::Integer(_) => "Integer",
            SupportedTypes::Null(_) => "Null",
            SupportedTypes::String(_) => "String",
        }
    }
}

impl Debug for SupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedTypes::BigInt(v) => write!(f, "{}_BigInt", v),
            SupportedTypes::Boolean(v) => write!(f, "{}_Boolean", v),
            SupportedTypes::Double(v) => write!(f, "{}_Double", v),
            SupportedTypes::Integer(v) => write!(f, "{}_Integer", v),
            SupportedTypes::Null(v) => write!(f, "{:?}_Null", v),
            SupportedTypes::String(v) => write!(f, "\"{}\"_String", v),
        }
    }
}

impl Display for SupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedTypes::BigInt(v) => write!(f, "{}_BigInt", v),
            SupportedTypes::Boolean(v) => write!(f, "{}_Boolean", v),
            SupportedTypes::Double(v) => write!(f, "{}_Double", v),
            SupportedTypes::Integer(v) => write!(f, "{}_Integer", v),
            SupportedTypes::Null(v) => write!(f, "{:?}_Null", v),
            SupportedTypes::String(v) => write!(f, "{}_String", v),
        }
    }
}
