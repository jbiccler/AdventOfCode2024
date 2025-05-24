use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ReturnType {
    Integer(i32),
    Long(i64),
    UnsignedInteger(u32),
    LongUnsignedInteger(u64),
    LongSingedInteger(i64),
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
            Self::LongSingedInteger(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::Text(val) => write!(f, "{}", val),
            Self::Boolean(val) => write!(f, "{}", val),
        }
    }
}

pub fn matrix_parse_whitespace<T>(contents: &str) -> Result<Vec<Vec<T>>, <T as FromStr>::Err>
where
    T: FromStr,
{
    let mut res: Vec<Vec<T>> = Vec::with_capacity(contents.trim().lines().count());
    for line in contents.trim().lines() {
        let mut row: Vec<T> = Vec::new();
        for c in line.split_whitespace() {
            match c.parse::<T>() {
                Ok(parsed) => row.push(parsed),
                Err(err) => return Err(err),
            }
        }
        res.push(row);
    }
    Ok(res)
}

pub fn matrix_parse<T>(contents: &str) -> Result<Vec<Vec<T>>, <T as FromStr>::Err>
where
    T: FromStr,
{
    let mut res: Vec<Vec<T>> = Vec::with_capacity(contents.trim().lines().count());
    for line in contents.trim().lines() {
        let mut row: Vec<T> = Vec::new();
        for c in line.chars() {
            match c.to_string().parse::<T>() {
                Ok(parsed) => row.push(parsed),
                Err(err) => return Err(err),
            }
        }
        res.push(row);
    }
    Ok(res)
}
