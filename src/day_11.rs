use std::io::BufRead;

pub fn part_1() -> i64 {
    let reader = std::io::BufReader::new(std::fs::File::open("day-11.txt").unwrap());
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.chars().collect())
        .collect();

    let planets: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(row, v)| -> _ {
            v.iter()
                .enumerate()
                .filter(|e| *e.1 == '#')
                .map(move |e| (row, e.0))
        })
        .collect();

    let rows_to_double: std::collections::BTreeSet<usize> = grid
        .iter()
        .enumerate()
        .filter(|s| !s.1.contains(&'#'))
        .map(|s| s.0)
        .collect();

    let cols_to_double: std::collections::BTreeSet<usize> = (0..grid[0].len())
        .filter(|c| grid.iter().all(|v| v[*c] != '#'))
        .collect();

    planets
        .iter()
        .enumerate()
        .map(|(i, (row, col))| {
            (i + 1..planets.len())
                .map(|j| planets[j])
                .map(|(other_row, other_col)| {
                    num::abs(other_row as i64 - *row as i64)
                        + num::abs(other_col as i64 - *col as i64)
                        + rows_to_double
                            .range(std::cmp::min(row, &other_row)..std::cmp::max(row, &other_row))
                            .count() as i64
                        + cols_to_double
                            .range(std::cmp::min(col, &other_col)..std::cmp::max(col, &other_col))
                            .count() as i64
                })
                .sum::<i64>()
        })
        .sum()
}

pub fn part_2() -> i64 {
    let reader = std::io::BufReader::new(std::fs::File::open("day-11.txt").unwrap());
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.chars().collect())
        .collect();

    let planets: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(row, v)| -> _ {
            v.iter()
                .enumerate()
                .filter(|e| *e.1 == '#')
                .map(move |e| (row, e.0))
        })
        .collect();

    let rows_to_double: std::collections::BTreeSet<usize> = grid
        .iter()
        .enumerate()
        .filter(|s| !s.1.contains(&'#'))
        .map(|s| s.0)
        .collect();

    let cols_to_double: std::collections::BTreeSet<usize> = (0..grid[0].len())
        .filter(|c| grid.iter().all(|v| v[*c] != '#'))
        .collect();

    planets
        .iter()
        .enumerate()
        .map(|(i, (row, col))| {
            (i + 1..planets.len())
                .map(|j| planets[j])
                .map(|(other_row, other_col)| {
                    num::abs(other_row as i64 - *row as i64)
                        + num::abs(other_col as i64 - *col as i64)
                        + (rows_to_double
                            .range(std::cmp::min(row, &other_row)..std::cmp::max(row, &other_row))
                            .count() as i64
                            * 999999)
                        + (cols_to_double
                            .range(std::cmp::min(col, &other_col)..std::cmp::max(col, &other_col))
                            .count() as i64
                            * 999999)
                })
                .sum::<i64>()
        })
        .sum()
}
