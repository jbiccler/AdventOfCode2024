#[derive(Debug, PartialEq)]
pub enum ReturnType {
    Integer(i32),
    UnsignedInteger(u32),
    Float(f64),
    Text(String),
    Boolean(bool),
}
