use advent_of_code2024::ReturnType;
use regex::Regex;
use std::error::Error;

const EPSILON: f64 = f64::EPSILON * 1000000000.;

#[derive(Debug)]
struct Button {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Prize {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Solution {
    a: u64,
    b: u64,
    cost: u64,
}

pub fn solve(contents: &str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    let data = parse(contents);

    let mut part1 = 0;
    let mut part2 = 0;

    for (a, b, p) in data.iter() {
        // part 1
        let sol = calc_solution(a, b, p);
        if let Some((a_presses, b_presses)) = sol {
            let sol = Solution {
                a: a_presses,
                b: b_presses,
                cost: a_presses * 3 + b_presses,
            };
            // part 1 limits nr of iterations to 100
            if a_presses <= 100 && b_presses <= 100 {
                part1 += sol.cost;
            }
        }
        // part 2
        let sol = calc_solution(
            a,
            b,
            &Prize {
                x: p.x + 10000000000000,
                y: p.y + 10000000000000,
            },
        );
        if let Some((a_presses, b_presses)) = sol {
            let sol = Solution {
                a: a_presses,
                b: b_presses,
                cost: a_presses * 3 + b_presses,
            };
            part2 += sol.cost;
        }
    }

    Ok((
        ReturnType::LongUnsignedInteger(part1),
        ReturnType::LongUnsignedInteger(part2),
    ))
}

fn is_whole(f: f64, eps: f64) -> bool {
    (f - f.round()).abs() < eps
}

fn calc_solution(a: &Button, b: &Button, p: &Prize) -> Option<(u64, u64)> {
    // Solution is a set of 2 equations with 2 unknowns, number of A and B presses
    // -> should have unique solution
    let (ax, ay) = (a.x as f64, a.y as f64);
    let (bx, by) = (b.x as f64, b.y as f64);
    let (px, py) = (p.x as f64, p.y as f64);

    let b_denom = ax * by - bx * ay;
    let a_denom = ax;

    // division by zero
    if b_denom.abs() < EPSILON || a_denom < EPSILON {
        return None;
    }

    let b_presses: f64 = (ax * py - px * ay) / b_denom;
    let a_presses: f64 = (px - b_presses * bx) / a_denom;

    // negative solutions not allowed
    if b_presses < -EPSILON || a_presses < -EPSILON {
        return None;
    }
    if is_whole(a_presses, EPSILON) && is_whole(b_presses, EPSILON) {
        Some((a_presses.round() as u64, b_presses.round() as u64))
    } else {
        None
    }
}

fn parse(contents: &str) -> Vec<(Button, Button, Prize)> {
    let re = Regex::new(r"Button A: X\+(?<xa>\d+), Y\+(?<ya>\d+)\nButton B: X\+(?<xb>\d+), Y\+(?<yb>\d+)\nPrize: X=(?<xp>\d+), Y=(?<yp>\d+)").unwrap();
    let mut result = vec![];

    for m in re.captures_iter(contents) {
        let a = Button {
            x: m["xa"].parse::<u64>().unwrap(),
            y: m["ya"].parse::<u64>().unwrap(),
        };
        let b = Button {
            x: m["xb"].parse::<u64>().unwrap(),
            y: m["yb"].parse::<u64>().unwrap(),
        };
        let p = Prize {
            x: m["xp"].parse::<u64>().unwrap(),
            y: m["yp"].parse::<u64>().unwrap(),
        };
        result.push((a, b, p));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_solver() {
        let (part1, part2) = solve(TEST_STRING).unwrap();
        if let (ReturnType::LongUnsignedInteger(d1), ReturnType::LongUnsignedInteger(d2)) =
            (part1, part2)
        {
            assert_eq!(d1, 480);
            assert_eq!(d2, 875318608908);
        } else {
            panic!();
        }
    }
}
