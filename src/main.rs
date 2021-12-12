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
            println!(
                "Year 2021, Day 01, Part 1: {}",
                year2021::day01a(utils::read_lines_as_i32_vector("input/day01.txt")?)
            );
            println!(
                "Year 2021, Day 01, Part 2: {}",
                year2021::day01b(utils::read_lines_as_i32_vector("input/day01.txt")?)
            );
        }
        ("2021", "day02") => {
            println!(
                "Year 2021, Day 02, Part 1: {}",
                year2021::day02a(utils::read_lines_as_str_vector("input/day02.txt")?)
            );
        }
        (_, _) => println!("Unknown input"),
    }
    Ok(())
}
