use advent_of_code2024::ReturnType;
use std::error::Error;

pub fn solve(path: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    Ok((
        ReturnType::UnsignedInteger(0),
        ReturnType::UnsignedInteger(0),
    ))
}
