use advent_of_code2024::ReturnType;
use std::collections::VecDeque;
use std::error::Error;
use std::{collections::HashMap, collections::HashSet};

use advent_of_code2024::matrix_parse;

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let data: Vec<Vec<char>> = matrix_parse(contents)?;
    let chars: HashSet<char> = HashSet::from_iter(contents.chars().filter(|c| !c.is_whitespace()));
    let mut region_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for c in chars {
        region_map.insert(c, region_indices(&data, c));
    }
    let (n, m) = (data.len(), data[0].len());
    let regions = get_regions(&region_map, n, m);
    let mut neighbors: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    for i in 0..n {
        for j in 0..m {
            neighbors.insert((i, j), equal_neighbors(&data, i, j, n, m));
        }
    }

    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    for ind in regions {
        let count = ind.len() as u64;
        let mut p: usize = 0;
        let s = calc_sides(&ind);
        for (i, j) in ind {
            let neighs = neighbors.get(&(i, j)).unwrap();
            let nr_neighbors_non_diagonal = neighs.len();
            p += 4 - nr_neighbors_non_diagonal;
        }
        part1 += count * p as u64;
        part2 += count * s;
    }

    Ok((
        ReturnType::LongUnsignedInteger(part1),
        ReturnType::LongUnsignedInteger(part2),
    ))
}

fn calc_sides(region: &[(usize, usize)]) -> u64 {
    // Note that a polygon with N corners has N sides.
    // We want to find the corners, so index +- 0.5
    // to avoid having to do float comparison we can just multiply indices by 10 and add 5 instead
    // to reach a corner.
    let region_scaled: Vec<(isize, isize)> = region
        .iter()
        .map(|(x, y)| (*x as isize * 10, *y as isize * 10))
        .collect();
    // Generate all unique corners
    let mut corner_candidates: HashSet<(isize, isize)> = HashSet::new();
    for (cx, cy) in &region_scaled {
        corner_candidates.insert((cx + 5, cy + 5));
        corner_candidates.insert((cx + 5, cy - 5));
        corner_candidates.insert((cx - 5, cy - 5));
        corner_candidates.insert((cx - 5, cy + 5));
    }

    // For each corner, offset again and check how many are present in the region
    // I.e. how many of these neighbors do we have, as this determines the type of corner.

    let mut corners: u64 = 0;
    for (cx, cy) in &corner_candidates {
        // offset back to neighbor locations
        let mut surrounding: [(isize, isize); 4] = [(0, 0); 4];
        surrounding[0] = (cx + 5, cy + 5);
        surrounding[1] = (cx + 5, cy - 5);
        surrounding[2] = (cx - 5, cy - 5);
        surrounding[3] = (cx - 5, cy + 5);
        // count the number present in region
        let count: Vec<u8> = surrounding
            .iter()
            .map(|(sx, sy)| (region_scaled.contains(&(*sx, *sy))) as u8)
            .collect();
        let hits = count.iter().sum();
        corners += match hits {
            1 => 1,
            2 => {
                if (count[0] == 1 && count[2] == 1) || (count[1] == 1 && count[3] == 1) {
                    // on diagonal -> 2 corners
                    //  x
                    //  x
                    //xxx
                    //   xxx
                    //   x
                    //   x
                    2
                } else {
                    // adjacent -> no corner
                    0
                }
            }
            3 => 1,
            _ => 0,
        };
    }
    corners
}

fn region_indices(data: &[Vec<char>], c: char) -> Vec<(usize, usize)> {
    data.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, &val)| val == c)
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn equal_neighbors(
    data: &[Vec<char>],
    x: usize,
    y: usize,
    n: usize,
    m: usize,
) -> HashSet<(usize, usize)> {
    assert!(m > 0 && n > 0);
    assert!(x < m && y < n);

    let mut neighbors: HashSet<(usize, usize)> = HashSet::new();
    let target = data[x][y];

    for (d1, d2) in DIRS.iter() {
        let x_pot = x.saturating_add_signed(*d1 as isize);
        let y_pot = y.saturating_add_signed(*d2 as isize);

        if x_pot < n && y_pot < m && data[x_pot][y_pot] == target {
            let edge_of_matrix: bool = x_pot == x && y_pot == y;
            if !edge_of_matrix {
                neighbors.insert((x_pot, y_pot));
            }
        }
    }
    neighbors
}

fn get_regions(
    map: &HashMap<char, Vec<(usize, usize)>>,
    n: usize,
    m: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut regions = vec![];
    let (n, m) = (n as i32, m as i32);
    for (c, ind) in map {
        let mut q: Vec<(usize, usize)> = ind.clone();
        while !q.is_empty() {
            let mut tmp: Vec<(usize, usize)> = vec![];
            // reset next values to check
            let mut next = VecDeque::new();
            next.push_back(q.pop().unwrap());

            while !next.is_empty() {
                let cur = next.pop_front().unwrap();
                tmp.push(cur);
                let (x, y) = cur;
                q.retain(|(i, j)| (*i, *j) != cur);

                for (d1, d2) in DIRS {
                    let x_pot = x as i32 + d1;
                    let y_pot = y as i32 + d2;

                    // not out of bounds
                    if x_pot >= 0 && x_pot < n && y_pot >= 0 && y_pot < m {
                        let x_pot = x_pot as usize;
                        let y_pot = y_pot as usize;
                        if q.contains(&(x_pot, y_pot)) && !next.contains(&(x_pot, y_pot)) {
                            next.push_back((x_pot, y_pot));
                        }
                    }
                }
            }
            regions.push(tmp);
        }
    }

    regions
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::LongUnsignedInteger(d1), ReturnType::LongUnsignedInteger(d2)) =
            (part1, part2)
        {
            assert_eq!(d1, 1930);
            assert_eq!(d2, 1206);
        } else {
            panic!();
        }
    }
}
