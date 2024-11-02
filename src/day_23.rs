use std::fs::File;
use std::io::{self, BufRead};

pub fn day23_part1() -> u64 {
    let file = File::open("day-23.txt").unwrap();
    let reader = io::BufReader::new(file);
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|f| f.unwrap().chars().collect())
        .collect();
    0
}
