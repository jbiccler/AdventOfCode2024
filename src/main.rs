#![allow(dead_code, unused_variables)]
use advent_of_code2024::ReturnType;
use std::env;
use std::error::Error;
use std::fs::{self};

pub mod days;
use days::{day01, day02, day03, day04, day05, day06, day07, day08};

fn get_day_fn(day: u32) -> impl Fn(&str) -> Result<(ReturnType, ReturnType), Box<dyn Error>> {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        6 => day06::solve,
        7 => day07::solve,
        8 => day08::solve,
        _ => unimplemented!("Unsuppted day entered."),
    }
}

fn construct_path(day: u32) -> String {
    format!(
        "{}/input/{:02}.txt",
        env::current_dir().unwrap().display(),
        day
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let day = args
        .get(1)
        .expect("No day provided as argument.")
        .parse::<u32>()
        .expect("Day can't be parsed to integer.");

    let path = construct_path(day);
    dbg!(&path);
    let contents = fs::read_to_string(path)?;
    let f = get_day_fn(day);

    let res = f(&contents)?;
    Ok(())
}
