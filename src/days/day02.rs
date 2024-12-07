use advent_of_code2024::ReturnType;
use std::error::Error;

pub fn check_safe(line: &[i32]) -> bool {
    // Determine increasing or decreasing based on first and second element
    let increasing = &line[0] > line.get(1).unwrap();
    for i in 0..(line.len() - 1) {
        let diff = line.get(i).unwrap().abs_diff(line[i + 1]);
        if (increasing && (line[i] <= line[i + 1]))
            || (!increasing && (line[i] >= line[i + 1]))
            || (diff < 1 || diff > 3)
        {
            return false;
        }
    }
    true
}

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    // Part 1
    let data = parse(contents);
    let mut count1: u32 = 0;

    for line in data.iter() {
        if check_safe(line) {
            count1 += 1;
        }
    }
    println!("First part: {}", count1);

    // Part 2
    let mut count2: u32 = 0;
    for line in data.iter() {
        if check_safe(line) {
            count2 += 1;
        } else {
            // Pop indexes and check if safe without
            for i in 0..line.len() {
                let mut cpy = line.clone();
                cpy.remove(i);
                if check_safe(&cpy) {
                    count2 += 1;
                    break;
                }
            }
        }
    }

    println!("Second part: {}", count2);
    Ok((
        ReturnType::UnsignedInteger(count1),
        ReturnType::UnsignedInteger(count2),
    ))
}

fn parse(contents: &str) -> Vec<Vec<i32>> {
    contents
        .lines()
        .filter_map(|line| {
            let nums: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().expect("Failed to parse number."))
                .collect();
            if nums.len() >= 2 {
                Some(nums)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::UnsignedInteger(d1), ReturnType::UnsignedInteger(d2)) = (part1, part2) {
            assert_eq!(d1, 2);
            assert_eq!(d2, 4);
        } else {
            panic!();
        }
    }
}
