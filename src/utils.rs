use anyhow::Result;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_lines_as_i32_vector(path: &str) -> Result<Vec<i32>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
}

pub fn read_lines_as_str_vector(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>())
}
