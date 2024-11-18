use std::io::{self, BufRead};

#[derive(std::hash::Hash, std::cmp::Eq, std::cmp::PartialEq)]
struct PointAndSteps {
    pub row: i128,
    pub col: i128,
    pub steps: usize,
}

#[derive(std::hash::Hash, std::cmp::Eq, std::cmp::PartialEq)]
struct Point {
    pub row: i128,
    pub col: i128,
}

impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row
            .cmp(&other.row)
            .then_with(|| self.col.cmp(&other.col))
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day21_part1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("day-21.txt").unwrap());
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|l| l.chars().collect())
        .collect();

    let mut steps: Vec<Vec<usize>> = vec![vec![69; grid[0].len()]; grid.len()];

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
    let mut dq: std::collections::VecDeque<PointAndSteps> = std::collections::VecDeque::new();
    dq.push_front(PointAndSteps {
        row: start_row as i128,
        col: start_col as i128,
        steps: 0,
    });

    while !dq.is_empty() {
        let temp = dq.pop_back().unwrap();
        if temp.steps > 64
            || grid[temp.row as usize][temp.col as usize] == '#'
            || steps[temp.row as usize][temp.col as usize] <= temp.steps
        {
            continue;
        }
        steps[temp.row as usize][temp.col as usize] = temp.steps;

        if temp.row != 0 {
            dq.push_back(PointAndSteps {
                row: temp.row - 1,
                col: temp.col,
                steps: temp.steps + 1,
            });
        }

        if temp.row as usize != grid.len() - 1 {
            dq.push_back(PointAndSteps {
                row: temp.row + 1,
                col: temp.col,
                steps: temp.steps + 1,
            });
        }

        if temp.col != 0 {
            dq.push_back(PointAndSteps {
                row: temp.row,
                col: temp.col - 1,
                steps: temp.steps + 1,
            });
        }

        if temp.col as usize != grid[0].len() - 1 {
            dq.push_back(PointAndSteps {
                row: temp.row,
                col: temp.col + 1,
                steps: temp.steps + 1,
            });
        }
    }
    steps
        .iter()
        .map(|v| v.iter().filter(|e| **e % 2 == 0).count())
        .sum()
}

pub fn day21_part2() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("sample.txt").unwrap());
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|l| l.chars().collect())
        .collect();

    let mut steps: std::collections::HashMap<i128, std::collections::HashMap<i128, usize>> =
        std::collections::HashMap::new();

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
    let mut dq: std::collections::VecDeque<PointAndSteps> = std::collections::VecDeque::new();
    dq.push_front(PointAndSteps {
        row: start_row as i128,
        col: start_col as i128,
        steps: 0,
    });

    while !dq.is_empty() {
        let temp = dq.pop_back().unwrap();
        if temp.steps > 1000
            || grid[temp.row.rem_euclid(grid.len() as i128) as usize]
                [temp.col.rem_euclid(grid[0].len() as i128) as usize]
                == '#'
            || (steps.contains_key(&temp.row)
                && steps[&temp.row].contains_key(&temp.col)
                && steps[&temp.row][&temp.col] <= temp.steps)
        {
            continue;
        }

        steps
            .entry(temp.row)
            .or_default()
            .insert(temp.col, temp.steps);

        dq.push_back(PointAndSteps {
            row: temp.row - 1,
            col: temp.col,
            steps: temp.steps + 1,
        });

        dq.push_back(PointAndSteps {
            row: temp.row + 1,
            col: temp.col,
            steps: temp.steps + 1,
        });

        dq.push_back(PointAndSteps {
            row: temp.row,
            col: temp.col - 1,
            steps: temp.steps + 1,
        });

        dq.push_back(PointAndSteps {
            row: temp.row,
            col: temp.col + 1,
            steps: temp.steps + 1,
        });
    }
    steps
        .iter()
        .map(|v| v.1.iter().filter(|e| e.1 % 2 == 0).count())
        .sum()
}
