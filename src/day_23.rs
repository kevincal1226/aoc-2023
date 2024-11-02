use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn dfs(grid: &Vec<Vec<char>>, dp: &mut Vec<Vec<u64>>, discovered: &mut HashSet<(usize, usize)>) {}

pub fn day23_part1() -> u64 {
    let file = File::open("day-23.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|f| f.unwrap().chars().collect())
        .collect();

    grid.insert(0, vec!['#'; grid[0].len()]);
    grid.push(vec!['#'; grid[0].len()]);

    let mut dp: Vec<Vec<u64>> = vec![vec![0; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '#' {
                let mut disc: HashSet<(usize, usize)> = HashSet::new();
                dfs(&grid, &mut dp, &mut disc);
            }
        }
    }

    dp[dp.len() - 2][dp[0].len() - 2]
}
