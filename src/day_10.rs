use std::io::{self, BufRead};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    dist: usize,
}

pub fn part_1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("day-10.txt").unwrap());

    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| line.chars().collect())
        .collect();

    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);

    grid.iter_mut().for_each(|v| {
        v.insert(0, '.');
        v.push('.');
    });

    let mut start_row = 0;
    let mut start_col = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start_row = i;
                start_col = j;
                break;
            }
        }
    }

    let mut dfs: std::collections::VecDeque<Point> = std::collections::VecDeque::new();

    dfs.push_back(Point {
        x: start_row,
        y: start_col,
        dist: 0,
    });

    println!("Start: {} {}", start_row, start_col);

    let mut discovered: std::collections::HashSet<(usize, usize)> =
        std::collections::HashSet::new();

    let mut p: Point;

    loop {
        p = dfs.pop_back().unwrap();
        if p.dist > 2 && grid[p.x][p.y] == 'S' {
            break;
        }

        if !discovered.insert((p.x, p.y)) {
            continue;
        }

        match grid[p.x][p.y] {
            'S' => {
                if ['|', '7', 'F'].contains(&grid[start_row - 1][start_col]) {
                    dfs.push_back(Point {
                        x: start_row - 1,
                        y: start_col,
                        dist: 1,
                    });
                }

                if ['|', 'J', 'L'].contains(&grid[start_row + 1][start_col]) {
                    dfs.push_back(Point {
                        x: start_row + 1,
                        y: start_col,
                        dist: 1,
                    });
                }
                if ['-', 'L', 'F'].contains(&grid[start_row][start_col - 1]) {
                    dfs.push_back(Point {
                        x: start_row,
                        y: start_col - 1,
                        dist: 1,
                    });
                }
                if ['-', '7', 'J'].contains(&grid[start_row][start_col + 1]) {
                    dfs.push_back(Point {
                        x: start_row,
                        y: start_col + 1,
                        dist: 1,
                    });
                }
            }
            '.' => continue,
            '|' => dfs.extend([
                Point {
                    x: p.x - 1,
                    y: p.y,
                    dist: p.dist + 1,
                },
                Point {
                    x: p.x + 1,
                    y: p.y,
                    dist: p.dist + 1,
                },
            ]),
            '-' => dfs.extend([
                Point {
                    x: p.x,
                    y: p.y - 1,
                    dist: p.dist + 1,
                },
                Point {
                    x: p.x,
                    y: p.y + 1,
                    dist: p.dist + 1,
                },
            ]),
            'L' => dfs.extend([
                Point {
                    x: p.x - 1,
                    y: p.y,
                    dist: p.dist + 1,
                },
                Point {
                    x: p.x,
                    y: p.y + 1,
                    dist: p.dist + 1,
                },
            ]),
            'J' => dfs.extend([
                Point {
                    x: p.x - 1,
                    y: p.y,
                    dist: p.dist + 1,
                },
                Point {
                    x: p.x,
                    y: p.y - 1,
                    dist: p.dist + 1,
                },
            ]),
            '7' => dfs.extend([
                Point {
                    x: p.x + 1,
                    y: p.y,
                    dist: p.dist + 1,
                },
                Point {
                    x: p.x,
                    y: p.y - 1,
                    dist: p.dist + 1,
                },
            ]),
            'F' => dfs.extend([
                Point {
                    x: p.x + 1,
                    y: p.y,
                    dist: p.dist + 1,
                },
                Point {
                    x: p.x,
                    y: p.y + 1,
                    dist: p.dist + 1,
                },
            ]),
            _ => panic!("Unrecognized character"),
        }
    }

    p.dist / 2
}

pub fn part_2() -> usize {
    0
}
