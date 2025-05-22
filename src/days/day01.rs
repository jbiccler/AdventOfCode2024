use advent_of_code2024::ReturnType;
use std::collections::HashMap;
use std::error::Error;

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let pairs = parse(contents);

    // First part
    // Split vector in right and left column
    let mut first: Vec<u32> = vec![];
    let mut second: Vec<u32> = vec![];

    for (f, s) in pairs.iter() {
        first.push(f.to_owned());
        second.push(s.to_owned());
    }
    // Sort
    first.sort_unstable();
    second.sort_unstable();

    assert_eq!(first.len(), second.len());

    // Calculate absolute distance
    let mut sum1: u32 = 0;
    for i in 0..first.len() {
        let diff = &first[i].abs_diff(second[i]);
        sum1 += diff;
    }

    // Second part
    // Construct frequency map
    let mut map: HashMap<u32, u32> = HashMap::new();
    for key in second.into_iter() {
        *map.entry(key).or_insert(0) += 1;
    }
    let sum2 = first.iter().map(|x| x * map.get(x).unwrap_or(&0)).sum();

    Ok((
        ReturnType::UnsignedInteger(sum1),
        ReturnType::UnsignedInteger(sum2),
    ))
}

fn parse(buf: &str) -> Vec<(u32, u32)> {
    buf.lines()
        .filter_map(|line| {
            let mut nums = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<u32>().expect("Failed to parse a number."));
            match (nums.next(), nums.next()) {
                (Some(first), Some(second)) => Some((first, second)),
                _ => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::UnsignedInteger(d1), ReturnType::UnsignedInteger(d2)) = (part1, part2) {
            assert_eq!(d1, 11);
            assert_eq!(d2, 31);
        } else {
            panic!();
        }
    }
}
