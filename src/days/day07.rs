use advent_of_code2024::ReturnType;
use std::error::Error;

#[derive(Copy, Clone)]
enum Operators {
    Add,
    Mult,
    Concat,
}

fn verify(nums: &[i64], ops: &Vec<Operators>) -> bool {
    if nums.len() == 2 {
        return nums[0] == nums[1];
    }
    let (total, a, b) = (nums[0], nums[1], nums[2]);
    // if this path is already larger than the total return false because only strictly positive operations
    if a > total {
        return false;
    }
    for op in ops.iter() {
        match op {
            Operators::Add => {
                let mut tmp = vec![total, a + b];
                tmp.extend(nums[3..].iter());
                if verify(&tmp, ops) {
                    return true;
                }
            }
            Operators::Mult => {
                let mut tmp = vec![total, a * b];
                tmp.extend(nums[3..].iter());
                if verify(&tmp, ops) {
                    return true;
                }
            }
            Operators::Concat => {
                let c = format!("{}{}", a, b).parse::<i64>().unwrap();
                let mut tmp = vec![total, c];
                tmp.extend(nums[3..].iter());
                if verify(&tmp, ops) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let data = parse(contents.trim());

    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in data {
        let total = line[0];
        if verify(&line, &vec![Operators::Add, Operators::Mult]) {
            sum1 += total;
        }
        if verify(
            &line,
            &vec![Operators::Add, Operators::Mult, Operators::Concat],
        ) {
            sum2 += total;
        }
    }

    Ok((ReturnType::Long(sum1), ReturnType::Long(sum2)))
}

fn parse(contents: &str) -> Vec<Vec<i64>> {
    let mut res = vec![];
    for l in contents.lines() {
        let mut tmp: Vec<i64> = vec![];
        let (first, rest) = l.split_once(": ").unwrap();
        tmp.push(first.parse::<i64>().unwrap());
        for r in rest.split_whitespace() {
            tmp.push(r.parse::<i64>().unwrap());
        }
        res.push(tmp);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::Long(d1), ReturnType::Long(d2)) = (part1, part2) {
            assert_eq!(d1, 3749);
            assert_eq!(d2, 11387);
        } else {
            panic!();
        }
    }
}
