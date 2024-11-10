use std::io::{self, BufRead};

pub fn day21_part1() -> u64 {
    let reader = std::io::BufReader::new(std::fs::File::open("sample.txt").unwrap());
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|l| l.chars().collect())
        .collect();

    let mut start_row = 0;
    let mut start_col = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 'S' {
                start_row = i;
                start_col = j;
                break;
            }
        }
    }
    grid.iter().for_each(|g| {
        println!("{:?}", g);
    });
    println!("{} {}", start_row, start_col);
    let mut dq: std::collections::VecDeque<i32> = std::collections::VecDeque::new();
    let mut coords: std::collections::HashSet<i32> = std::collections::HashSet::new();
    0
}

pub fn day21_part2() -> u64 {
    0
}
