use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn dfs_1(
    grid: &Vec<Vec<char>>,
    discovered: &mut HashSet<(usize, usize)>,
    curr: u64,
    best: &mut u64,
    row: usize,
    col: usize,
) {
    if row == grid[0].len() - 2 && col == grid[0].len() - 2 {
        *best = max(*best, curr);
        return;
    }

    if grid[row][col] == '#' || !discovered.insert((row, col)) {
        return;
    }

    if grid[row][col] == 'v' {
        dfs_1(grid, &mut discovered.clone(), curr + 1, best, row + 1, col);
    } else if grid[row][col] == '^' {
        dfs_1(grid, &mut discovered.clone(), curr + 1, best, row - 1, col);
    } else if grid[row][col] == '<' {
        dfs_1(grid, &mut discovered.clone(), curr + 1, best, row, col - 1);
    } else if grid[row][col] == '>' {
        dfs_1(grid, &mut discovered.clone(), curr + 1, best, row, col + 1);
    } else {
        dfs_1(grid, &mut discovered.clone(), curr + 1, best, row + 1, col);
        dfs_1(grid, &mut discovered.clone(), curr + 1, best, row - 1, col);
        dfs_1(grid, &mut discovered.clone(), curr + 1, best, row, col + 1);
        dfs_1(grid, &mut discovered.clone(), curr + 1, best, row, col - 1);
    }
}

fn dfs_2(
    grid: &Vec<Vec<bool>>,
    discovered: &mut HashSet<(usize, usize)>,
    curr: u64,
    best: &mut u64,
    row: usize,
    col: usize,
) {
    if row == grid[0].len() - 2 && col == grid[0].len() - 2 {
        *best = max(*best, curr);
        return;
    }

    if !grid[row][col] || !discovered.insert((row, col)) {
        return;
    }
    dfs_2(grid, discovered, curr + 1, best, row + 1, col);
    dfs_2(grid, discovered, curr + 1, best, row - 1, col);
    dfs_2(grid, discovered, curr + 1, best, row, col + 1);
    dfs_2(grid, discovered, curr + 1, best, row, col - 1);
}

pub fn day23_part1() -> u64 {
    let file = File::open("day-23.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|f| f.unwrap().chars().collect())
        .collect();

    grid.insert(0, vec!['#'; grid[0].len()]);
    grid.push(vec!['#'; grid[0].len()]);

    let mut disc: HashSet<(usize, usize)> = HashSet::new();
    let mut num_found_down: u64 = 0;
    dfs_1(&grid, &mut disc, 0, &mut num_found_down, 1, 1);

    num_found_down
}

#[derive(Clone)]
struct Vertex {
    id: usize,
    edge_weight: usize,
}

impl Vertex {
    pub fn new(id_in: usize, edge_in: usize) -> Vertex {
        Vertex {
            id: id_in,
            edge_weight: edge_in,
        }
    }
}

fn get_num_connections(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    grid[row - 1][col] as usize
        + grid[row + 1][col] as usize
        + grid[row][col - 1] as usize
        + grid[row][col + 1] as usize
}

fn merge_edges(grid: &Vec<Vec<bool>>, adjlist: &mut Vec<Vec<Vertex>>, row: usize, col: usize) {}

pub fn day23_part2() -> u64 {
    let file = File::open("day-23.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut grid: Vec<Vec<bool>> = reader
        .lines()
        .map(|f| f.unwrap().chars().map(|e| e != '#').collect())
        .collect();

    grid.insert(0, vec![false; grid[0].len()]);
    grid.push(vec![false; grid[0].len()]);

    let mut adjlist: Vec<Vec<Vertex>> = vec![Vec::new(); grid.len() * grid[0].len()];

    let mut disc: HashSet<(usize, usize)> = HashSet::new();
    let mut num_found_down: u64 = 0;
    dfs_2(&grid, &mut disc, 0, &mut num_found_down, 1, 1);

    num_found_down
}
