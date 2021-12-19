#![feature(iter_advance_by)]
#![feature(concat_idents)]
#![feature(option_get_or_insert_default)]

use anyhow::Result;
use paste::paste;
use std::env;

mod utils;
mod year2021;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let year = args.get(1).unwrap();
    let day = args.get(2).unwrap();
    match (year.as_str(), day.as_str()) {
        ("2021", "day01") => {
            advent_run!(2021, 1, utils::read_lines_as_i32_vector);
        }
        ("2021", "day02") => {
            advent_run!(2021, 2, utils::read_lines_as_str_vector);
        }
        ("2021", "day03") => {
            advent_run!(2021, 3, utils::read_lines_as_str_vector);
        }
        ("2021", "day04") => {
            advent_run!(2021, 4, utils::read_lines_as_str_vector);
        }
        ("2021", "day05") => {
            advent_run!(2021, 5, utils::read_lines_as_str_vector);
        }
        (_, _) => println!("Unknown input"),
    }

    Ok(())
}

#[macro_export]
macro_rules! advent_run {
    ($year:expr, $day:expr, $utils:path) => {
        paste! {
            let part1 = crate::[<year $year>]::[<day0 $day a>]($utils(format!("input/{}/day0{}.txt", $year, $day).as_str())?);
            let part2 = crate::[<year $year>]::[<day0 $day b>]($utils(format!("input/{}/day0{}.txt", $year, $day).as_str())?);
            println!(
                "Year {}, Day 0{}, Part 1: {}", $year, $day, part1
            );
            println!(
                "Year {}, Day 0{}, Part 2: {}", $year, $day, part2
            );
        }
    };
}
