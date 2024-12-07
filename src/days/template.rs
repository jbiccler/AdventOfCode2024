use advent_of_code2024::ReturnType;
use std::error::Error;

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    Ok((
        ReturnType::UnsignedInteger(0),
        ReturnType::UnsignedInteger(0),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::UnsignedInteger(d1), ReturnType::UnsignedInteger(d2)) = (part1, part2) {
            assert_eq!(d1, 0);
            assert_eq!(d2, 0);
        } else {
            panic!();
        }
    }
}
