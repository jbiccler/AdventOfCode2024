#[derive(Debug, PartialEq)]
pub enum ReturnType {
    Integer(i32),
    Long(i64),
    UnsignedInteger(u32),
    Float(f64),
    Text(String),
    Boolean(bool),
}
