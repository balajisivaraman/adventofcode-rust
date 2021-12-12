#![feature(iter_advance_by)]

use anyhow::Result;
use std::env;

mod utils;
mod year2021;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let year = args.get(1).unwrap();
    let day = args.get(2).unwrap();
    match (year.as_str(), day.as_str()) {
        ("2021", "day01") => {
            let result = year2021::day01a(utils::read_lines_as_i32_vector("input/day01.txt")?);
            println!("Year 2021, Day 01, Part 1: {}", result);
        }
        (_, _) => println!("Unknown input"),
    }
    Ok(())
}
