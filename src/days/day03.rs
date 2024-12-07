use advent_of_code2024::ReturnType;
use regex::Regex;
use std::error::Error;

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    // part 1
    let pattern1 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let sum1: u32 = pattern1
        .captures_iter(contents.trim())
        .map(|cap| {
            cap.get(1).unwrap().as_str().parse::<u32>().unwrap()
                * cap.get(2).unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum();

    println!("First part: {}", sum1);

    // part 2
    let pattern2 = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))")?;
    let mut flag = true; // track do or don't state
    let mut sum2 = 0;
    for cap in pattern2.captures_iter(&contents) {
        match cap.get(0).unwrap().as_str() {
            "don't()" => flag = false,
            "do()" => flag = true,
            _ => {
                if flag {
                    sum2 += cap.get(2).unwrap().as_str().parse::<u32>().unwrap()
                        * cap.get(3).unwrap().as_str().parse::<u32>().unwrap();
                }
            }
        }
    }
    println!("Second part: {}", sum2);
    Ok((
        ReturnType::UnsignedInteger(sum1),
        ReturnType::UnsignedInteger(sum2),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::UnsignedInteger(d1), ReturnType::UnsignedInteger(d2)) = (part1, part2) {
            assert_eq!(d1, 161);
            assert_eq!(d2, 48);
        } else {
            panic!();
        }
    }
}
