use advent_of_code2024::ReturnType;
use std::error::Error;

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let (mut cs, filled, empty) = parse(contents);
    let mut i = filled.len() - 1;
    let stop = filled.len();
    for e in empty {
        if e >= stop {
            break;
        }
        cs[e] = cs[filled[i]];
        cs[filled[i]] = -1;
        i -= 1;
    }

    let mut sum1 = 0;
    for (i, &x) in cs.iter().enumerate() {
        if x == -1 {
            continue;
        }
        sum1 += (i as u64) * (x as u64);
    }

    Ok((
        ReturnType::LongUnsignedInteger(sum1),
        ReturnType::UnsignedInteger(0),
    ))
}

fn parse(contents: &str) -> (Vec<i32>, Vec<usize>, Vec<usize>) {
    let mut cs: Vec<i32> = vec![];
    let mut file = true;
    let mut i = 0;
    for c in contents.trim().chars() {
        let n = c.to_string().parse::<usize>().unwrap();
        if file {
            for _ in 0..n {
                cs.push(i);
            }
            i += 1;
            file = false;
        } else {
            for _ in 0..n {
                cs.push(-1);
            }
            file = true;
        }
    }
    // create vectors containing indices with full and empty spots
    let mut filled: Vec<usize> = vec![];
    let mut empty: Vec<usize> = vec![];
    for (i, c) in cs.iter().enumerate() {
        match c {
            -1 => empty.push(i),
            _ => filled.push(i),
        }
    }
    (cs, filled, empty)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "2333133121414131402";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::UnsignedInteger(d1), ReturnType::UnsignedInteger(d2)) = (part1, part2) {
            assert_eq!(d1, 1928);
            assert_eq!(d2, 0);
        } else {
            panic!();
        }
    }
}
