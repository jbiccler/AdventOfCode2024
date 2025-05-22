use advent_of_code2024::ReturnType;
use std::{collections::HashMap, error::Error};

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let data = parse(contents);
    let mut map: HashMap<(u64, usize), u64> = HashMap::new();

    // Part 1
    let mut part1 = 0;
    for &d in data.iter() {
        let c = count(d, 25, &mut map);
        part1 += c;
    }
    // Part 2
    let mut part2 = 0;
    for &d in data.iter() {
        let c = count(d, 75, &mut map);
        part2 += c;
    }

    Ok((
        ReturnType::LongUnsignedInteger(part1),
        ReturnType::LongUnsignedInteger(part2),
    ))
}

fn parse(contents: &str) -> Vec<u64> {
    contents
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn count(x: u64, n: usize, map: &mut HashMap<(u64, usize), u64>) -> u64 {
    if n == 0 {
        return 1;
    }
    if let Some(&c) = map.get(&(x, n)) {
        // return cached count
        return c;
    }
    let c: u64;
    if x == 0 {
        c = count(1, n - 1, map);
    } else {
        // length of the digit
        let l = x.ilog10() + 1;
        if l % 2 != 0 {
            c = count(x * 2024, n - 1, map);
        } else {
            let left = x / 10u64.pow(l / 2);
            let right = x % 10u64.pow(l / 2);
            c = count(left, n - 1, map) + count(right, n - 1, map);
        }
    }
    map.insert((x, n), c);
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "125 17";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::LongUnsignedInteger(d1), ReturnType::LongUnsignedInteger(d2)) =
            (part1, part2)
        {
            // assert_eq!(d1, 6);
            // assert_eq!(d2, 0);
        } else {
            panic!();
        }
    }
}
