use advent_of_code2024::ReturnType;
use std::error::Error;

fn search_count(lines: &Vec<&str>, search: &str) -> u32 {
    let n = lines[0].len();
    let m = lines.len();
    let nr = search.len();

    let mut count: u32 = 0;
    let rev: String = search.chars().rev().collect();
    let first_char = search.chars().next().unwrap();
    let last_char = rev.chars().next().unwrap();

    // horizontal
    for line in lines.iter() {
        for i in 0..=(n - nr) {
            let end = i + nr - 1;
            let s = &line[i..=end];
            if (s == search) || (s == rev) {
                count += 1;
            }
        }
    }

    // vertical
    for i in 0..=(m - nr) {
        for j in 0..n {
            // construct slice
            let mut s = String::from("");
            let first = lines.get(i).unwrap().chars().nth(j).unwrap();
            if (first != first_char) && (first != last_char) {
                continue;
            }

            for q in 0..nr {
                s.push(lines.get(i + q).unwrap().chars().nth(j).unwrap());
            }
            if (s == search) || (s == rev) {
                count += 1;
            }
        }
    }

    // main diagonal
    for i in 0..=(m - nr) {
        for j in 0..=(n - nr) {
            // construct slice
            let mut s = String::from("");
            let first = lines.get(i).unwrap().chars().nth(j).unwrap();
            if (first != first_char) && (first != last_char) {
                continue;
            }
            for q in 0..nr {
                s.push(lines.get(i + q).unwrap().chars().nth(j + q).unwrap());
            }
            if (s == search) || (s == rev) {
                count += 1;
            }
        }
    }
    // off diagonal
    for i in 0..=(m - nr) {
        for j in (nr - 1)..n {
            // construct slice
            let mut s = String::from("");
            let first = lines.get(i).unwrap().chars().nth(j).unwrap();
            if (first != first_char) && (first != last_char) {
                continue;
            }
            for q in 0..nr {
                s.push(lines.get(i + q).unwrap().chars().nth(j - q).unwrap());
            }
            if (s == search) || (s == rev) {
                count += 1;
            }
        }
    }
    count
}

fn mas_cross_count(lines: &Vec<&str>) -> u32 {
    let n = lines[0].len();
    let m = lines.len();
    let mut count: u32 = 0;

    // Find A an construct square around it
    for i in 1..(m - 1) {
        for j in 1..(n - 1) {
            let centre = lines.get(i).unwrap().chars().nth(j).unwrap();
            match centre {
                'A' => {
                    // construct slice of corner points of square
                    let mut s = String::from("");
                    s.push(lines.get(i - 1).unwrap().chars().nth(j - 1).unwrap());
                    s.push(lines.get(i - 1).unwrap().chars().nth(j + 1).unwrap());
                    s.push(lines.get(i + 1).unwrap().chars().nth(j - 1).unwrap());
                    s.push(lines.get(i + 1).unwrap().chars().nth(j + 1).unwrap());

                    match &s[..] {
                        "MSMS" | "MMSS" | "SSMM" | "SMSM" => count += 1,
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }
    }
    count
}

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    const SEARCH_STRING: &str = "XMAS";

    let lines = parse(contents);
    let sum1 = search_count(&lines, SEARCH_STRING);
    println!("First part: {}", sum1);

    let sum2 = mas_cross_count(&lines);
    println!("Second part: {}", sum2);
    Ok((
        ReturnType::UnsignedInteger(sum1),
        ReturnType::UnsignedInteger(sum2),
    ))
}

fn parse(contents: &str) -> Vec<&str> {
    contents
        .split_ascii_whitespace()
        .filter(|m| !m.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::UnsignedInteger(d1), ReturnType::UnsignedInteger(d2)) = (part1, part2) {
            assert_eq!(d1, 18);
            assert_eq!(d2, 9);
        } else {
            panic!();
        }
    }
}
