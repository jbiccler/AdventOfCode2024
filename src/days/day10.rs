use advent_of_code2024::ReturnType;
use std::{collections::HashSet, error::Error};

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let data = parse(contents);
    let zeros = zero_indices(&data);
    let n_zeros = zeros.len();

    let mut part1 = 0;
    let mut part2 = 0;

    for &z in zeros.iter() {
        let (x, y) = z;
        let c = eval_trail(&data, x, y);
        let s: HashSet<(usize, usize)> = HashSet::from_iter(c.clone());

        part1 += s.len();
        part2 += c.len();
    }

    Ok((
        ReturnType::UnsignedInteger(part1 as u32),
        ReturnType::UnsignedInteger(part2 as u32),
    ))
}

fn parse(contents: &str) -> Vec<Vec<u32>> {
    let mut data: Vec<Vec<u32>> = vec![];
    for line in contents.lines() {
        let line = line.trim();
        let mut row = vec![];
        for c in line.chars() {
            let x = c.to_digit(10).unwrap();
            row.push(x);
        }
        data.push(row);
    }
    data
}

fn zero_indices(data: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut zeros: Vec<(usize, usize)> = vec![];
    for (i, row) in data.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            if val == 0 {
                zeros.push((i, j));
            }
        }
    }
    zeros
}

fn get_neighbors(x: usize, y: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    for (d1, d2) in DIRS.iter() {
        let x_pot = x as i32 + d1;
        let y_pot = y as i32 + d2;

        if x_pot >= 0 && x_pot <= (n as i32 - 1) && y_pot >= 0 && y_pot <= (m as i32 - 1) {
            neighbors.push((x_pot as usize, y_pot as usize))
        }
    }
    neighbors
}

fn eval_trail(data: &Vec<Vec<u32>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let (n, m) = (data.len(), data[0].len());
    let cur = data.get(x).unwrap().get(y).unwrap().clone();
    let mut c: Vec<(usize, usize)> = vec![];

    if cur == 9 {
        // found a peak, return coordinates
        c.push((x, y));
    } else {
        let neighbors = get_neighbors(x, y, n, m);
        if neighbors.len() == 0 {
            return c;
        } else {
            for (nx, ny) in neighbors.iter() {
                if data[*nx][*ny] == cur + 1 {
                    // evaluate neighboring square with height increase 1
                    let r = eval_trail(data, *nx, *ny);
                    if r.len() != 0 {
                        for &rval in r.iter() {
                            c.push(rval);
                        }
                    }
                }
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

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
