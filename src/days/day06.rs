use advent_of_code2024::ReturnType;
use std::collections::HashMap;
use std::error::Error;

struct Circular {
    data: Vec<Vec<i32>>,
    index: usize,
}

impl Circular {
    fn new(data: Vec<Vec<i32>>) -> Self {
        Circular { data, index: 0 }
    }
    fn next(&mut self) -> Vec<i32> {
        self.index = (self.index + 1) % self.data.len();
        self.data[self.index].clone()
    }
    fn current(&self) -> Vec<i32> {
        self.data[self.index].clone()
    }
    fn get_index(&self) -> usize {
        self.index
    }
}

fn places_visited(data: &Vec<Vec<u32>>, start: (usize, usize)) -> (Vec<Vec<u32>>, i32) {
    let (n, m) = (data.len(), data[0].len());
    let mut pos: Vec<Vec<u32>> = vec![vec![0; m]; n];

    let mut current = start.to_owned();
    // directions
    let d: Vec<Vec<i32>> = vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1]];
    let mut dirs: Circular = Circular::new(d);
    let mut cdir = dirs.current();

    let mut sum = 0;
    // for part2
    // populate map of obstuctions
    let mut obs_map: HashMap<(usize, usize), [usize; 4]> = HashMap::new();
    for r in 0..n {
        for c in 0..n {
            if data[r][c] == 0 {
                obs_map.insert((r, c), [0, 0, 0, 0]);
            }
        }
    }

    loop {
        // update position
        if pos[current.0][current.1] == 0 {
            sum += 1;
        }
        pos[current.0][current.1] = 1;
        // check next position
        let next = (current.0 as i32 + cdir[0], current.1 as i32 + cdir[1]);
        if (next.0 < 0) || (next.0 >= n as i32) || (next.1 < 0) || (next.1 >= m as i32) {
            // out of bounds
            break;
        } else if data[next.0 as usize][next.1 as usize] == 0 {
            // obstacle hit
            let obs_tracker = obs_map
                .get_mut(&(next.0 as usize, next.1 as usize))
                .unwrap();
            if obs_tracker[dirs.get_index()] > 0 {
                // we are in a loop
                return (pos, -1);
            } else {
                obs_tracker[dirs.get_index()] += 1;
            }
            // need to change direction
            cdir = dirs.next();
            continue;
        } else {
            // valid move
            current = (next.0 as usize, next.1 as usize);
        }
    }
    (pos, sum)
}

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let (mut data, start) = parse(contents);
    let (n, m) = (data.len(), data[0].len());

    let (pos, sum1) = places_visited(&data, start);

    println!("First part: {}", sum1);

    // Second part
    // Only place obstructions on visited places and track if any obstruction is being hit from
    // the same direction multiple times, if so you are in a loop

    let mut obs: Vec<(usize, usize)> = vec![];
    for r in 0..n {
        for c in 0..m {
            if (pos[r][c] == 1) && ((r, c) != start) {
                obs.push((r, c));
            }
        }
    }
    let mut sum2 = 0;
    for (x, y) in obs {
        // Create a copy of the map and place the obstuction
        let prev_data_value = data[x][y];
        data[x][y] = 0;
        let (_, count2) = places_visited(&data, start);
        if count2 < 0 {
            sum2 += 1
        }
        data[x][y] = prev_data_value;
    }
    println!("Second part: {}", sum2);

    Ok((
        ReturnType::UnsignedInteger(sum1 as u32),
        ReturnType::UnsignedInteger(sum2),
    ))
}

fn parse(contents: &str) -> (Vec<Vec<u32>>, (usize, usize)) {
    let mut parsed: Vec<Vec<u32>> = contents
        .split_whitespace()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => 0,
                    '.' => 1,
                    '^' => 2,
                    _ => unimplemented!(),
                })
                .collect()
        })
        .collect();
    let (n, m) = (parsed.len(), parsed[1].len());
    let mut start = (0, 0);
    for r in 0..n {
        for c in 0..m {
            if parsed[r][c] == 2 {
                parsed[r][c] = 1;
                start = (r, c);
                dbg!(&start);
            }
        }
    }
    (parsed, start)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::UnsignedInteger(d1), ReturnType::UnsignedInteger(d2)) = (part1, part2) {
            assert_eq!(d1, 41);
            assert_eq!(d2, 6);
        } else {
            panic!();
        }
    }
}
