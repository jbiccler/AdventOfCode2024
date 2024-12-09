use advent_of_code2024::ReturnType;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> (i32, i32) {
        ((self.x - other.x), (self.y - other.y))
    }
    fn offset(&self, x: i32, y: i32) -> Self {
        Point {
            x: (self.x + x),
            y: (self.y + y),
        }
    }
    fn in_grid(&self, n: usize, m: usize) -> bool {
        !((self.x < 0) || (self.y < 0) || (self.x >= n as i32) || (self.y >= m as i32))
    }
}

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let ((n, m), map) = parse(contents);
    // Track unique antinode positions
    let mut set1: HashSet<Point> = HashSet::new();
    let mut set2: HashSet<Point> = HashSet::new();
    for val in map.values() {
        for first in val.iter() {
            for second in val.iter() {
                if first != second {
                    let (dx, dy) = first.distance(&second);
                    let mut counter = 0;
                    let mut anti = first.offset(dx * counter, dy * counter);
                    while anti.in_grid(n, m) {
                        set2.insert(anti.clone());
                        if counter == 1 {
                            set1.insert(anti);
                        }
                        counter += 1;
                        anti = first.offset(dx * counter, dy * counter);
                    }
                }
            }
        }
    }

    let count1 = set1.len() as u32;
    let count2 = set2.len() as u32;
    println!("First part: {}", count1);
    println!("Second part: {}", count2);

    Ok((
        ReturnType::UnsignedInteger(count1),
        ReturnType::UnsignedInteger(count2),
    ))
}

fn parse(contents: &str) -> ((usize, usize), HashMap<char, Vec<Point>>) {
    let mut map = HashMap::new();
    let mut n = 0;
    let mut m = 0;
    for (i, line) in contents.trim().lines().enumerate() {
        // track dimensions
        if i == 0 {
            m = line.trim().len();
        }
        n += 1;

        // parse
        for (j, c) in line.trim().chars().enumerate() {
            if c != '.' {
                let p = Point {
                    x: i as i32,
                    y: j as i32,
                };
                map.entry(c).or_insert_with(Vec::new).push(p);
            }
        }
    }
    ((n, m), map)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::UnsignedInteger(d1), ReturnType::UnsignedInteger(d2)) = (part1, part2) {
            assert_eq!(d1, 14);
            assert_eq!(d2, 34);
        } else {
            panic!();
        }
    }
}
