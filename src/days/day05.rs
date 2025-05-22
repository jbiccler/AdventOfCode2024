use advent_of_code2024::ReturnType;
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let (s1, s2) = contents.split_once("\n\n").unwrap();
    let mut ord: HashMap<u32, HashSet<u32>> = HashMap::new();
    // create hashmap with rules
    for line in s1.lines() {
        let (x, y) = line.trim().split_once("|").unwrap();
        let x = x.parse::<u32>().unwrap();
        let y = y.parse::<u32>().unwrap();
        ord.entry(x).or_default().insert(y);
    }
    // create vector of page sets
    let pages: Vec<Vec<u32>> = s2
        .trim()
        .lines()
        .map(|l| l.split(",").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();

    // fill in missing page values in hashmap of rules
    for p in pages.iter() {
        for i in p {
            if !ord.contains_key(i) {
                ord.insert(i.to_owned(), HashSet::new());
            }
        }
    }

    // check sorting
    let (mut sum1, mut sum2) = (0, 0);
    for mut p in pages {
        if p.is_sorted_by(|a, b| ord[a].contains(b)) {
            // dbg!(&p);
            sum1 += p[p.len() / 2];
        } else {
            p.sort_by(|a, b| ord[a].contains(b).cmp(&true));
            sum2 += p[p.len() / 2];
        }
    }

    Ok((
        ReturnType::UnsignedInteger(sum1),
        ReturnType::UnsignedInteger(sum2),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::UnsignedInteger(d1), ReturnType::UnsignedInteger(d2)) = (part1, part2) {
            assert_eq!(d1, 143);
            assert_eq!(d2, 123);
        } else {
            panic!();
        }
    }
}
