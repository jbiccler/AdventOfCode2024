use advent_of_code2024::ReturnType;
use regex::Regex;
use std::error::Error;

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let data = parse(contents);
    // Set grid size, different for test/example case
    let (xmax, ymax) = if cfg!(test) { (11, 7) } else { (101, 103) };
    // First part
    // Move each robot 100 positions
    let mut data_shifted = Vec::with_capacity(data.len());
    for r in data.iter() {
        data_shifted.push(calc_position(r, 100, xmax, ymax));
    }
    let quads = quadrant_count(&data_shifted, xmax, ymax);
    let part1 = quads.iter().product();

    // Second part
    // Find solution with least variance in x and y positions
    let (mut min_var, mut part2) = (f64::MAX, 0);
    let mut x_shifted = vec![0; data.len()];
    let mut y_shifted = vec![0; data.len()];
    for i in 0..(xmax * ymax) {
        // Move robots
        for (j, r) in data.iter().enumerate() {
            let (xnew, ynew) = calc_position(r, i, xmax, ymax);
            x_shifted[j] = xnew;
            y_shifted[j] = ynew;
        }
        // Calculate variances of the positions
        let (xvar, yvar) = (variance(&x_shifted), variance(&y_shifted));
        let var = xvar + yvar;
        if var < min_var {
            min_var = var;
            part2 = i;
        }
    }
    Ok((
        ReturnType::LongUnsignedInteger(part1),
        ReturnType::LongUnsignedInteger(part2 as u64),
    ))
}

fn mean(x: &[i64]) -> f64 {
    let n = x.len() as f64;
    x.iter().sum::<i64>() as f64 / n
}
fn variance(x: &[i64]) -> f64 {
    let n = x.len() as f64;
    let mean = mean(x);
    x.iter().map(|y| (*y as f64 - mean).powi(2)).sum::<f64>() / n
}

fn calc_position(r: &Robot, n: usize, xmax: usize, ymax: usize) -> (i64, i64) {
    let x_new = (r.x + r.vx * (n as i64))
        .checked_rem_euclid(xmax as i64)
        .unwrap();
    let y_new = (r.y + r.vy * (n as i64))
        .checked_rem_euclid(ymax as i64)
        .unwrap();

    (x_new, y_new)
}

fn quadrant_count(data: &[(i64, i64)], xmax: usize, ymax: usize) -> [u64; 4] {
    let (xmax, ymax) = (xmax as i64, ymax as i64);
    let mut quad: [u64; 4] = [0, 0, 0, 0];
    let (hx, hy) = (xmax / 2, ymax / 2);
    for d in data {
        if d.0 == hx || d.1 == hy {
            continue;
        }
        if d.0 < hx {
            if d.1 < hy {
                quad[0] += 1;
            } else {
                quad[1] += 1;
            }
        } else if d.1 < hy {
            quad[2] += 1;
        } else {
            quad[3] += 1;
        }
    }
    quad
}

fn parse(contents: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    let mut result = vec![];

    for m in re.captures_iter(contents) {
        let a = Robot {
            x: m["px"].parse::<i64>().unwrap(),
            y: m["py"].parse::<i64>().unwrap(),
            vx: m["vx"].parse::<i64>().unwrap(),
            vy: m["vy"].parse::<i64>().unwrap(),
        };
        result.push(a);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::LongUnsignedInteger(d1), ReturnType::LongUnsignedInteger(d2)) =
            (part1, part2)
        {
            assert_eq!(d1, 12);
            // assert_eq!(d2, 0);
        } else {
            panic!();
        }
    }
}
