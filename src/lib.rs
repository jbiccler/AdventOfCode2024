use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ReturnType {
    Integer(i32),
    Long(i64),
    UnsignedInteger(u32),
    LongUnsignedInteger(u64),
    Float(f64),
    Text(String),
    Boolean(bool),
}

impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(val) => write!(f, "{}", val),
            Self::Long(val) => write!(f, "{}", val),
            Self::UnsignedInteger(val) => write!(f, "{}", val),
            Self::LongUnsignedInteger(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::Text(val) => write!(f, "{}", val),
            Self::Boolean(val) => write!(f, "{}", val),
        }
    }
}
