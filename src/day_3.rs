use std::fs::File;
use std::io::{self, BufRead};

const DIRECTIONS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn is_part_number(input: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    let mut part_number = false;
    DIRECTIONS.iter().for_each(|dir| {
        part_number = part_number
            | (!input[(i + dir.0) as usize][(j + dir.1) as usize].is_ascii_digit()
                && input[(i + dir.0) as usize][(j + dir.1) as usize] != '.')
    });
    part_number
}

fn edit_string(s: String) -> String {
    let mut n = s.to_owned();
    n.insert(0, '.');
    n + "."
}

fn get_value(input: &mut Vec<Vec<char>>, i: usize, mut j: usize) -> (u64, bool) {
    let mut res = 0;
    let mut part_number = false;
    while input[i][j].is_ascii_digit() {
        j -= 1;
    }
    j += 1;
    while input[i][j].is_ascii_digit() {
        part_number |= is_part_number(&input, i as i32, j as i32);
        res = res * 10 + input[i][j].to_digit(10).expect("int");
        input[i][j] = '.';
        j += 1;
    }
    (res as u64, part_number)
}

fn get_gear_ratio(input: &Vec<Vec<char>>, i: i32, j: i32) -> u64 {
    let mut count = 0;
    let mut result: u64 = 1;

    let mut grid = input.clone();
    DIRECTIONS.iter().for_each(|direction| {
        if grid[(i + direction.0) as usize][(j + direction.1) as usize].is_ascii_digit() {
            let (temp, is_part) = get_value(
                &mut grid,
                (i + direction.0) as usize,
                (j + direction.1) as usize,
            );

            if is_part {
                result *= temp;
                count += 1;
            }
        }
    });

    if count == 2 {
        result
    } else {
        0
    }
}

pub fn day3_part1() -> u32 {
    let file = File::open("day-3.txt").expect("E");
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    let mut input: Vec<Vec<char>> = reader
        .lines()
        .map(|l| edit_string(l.expect("str")))
        .map(|s| s.chars().collect())
        .collect();

    let period_vec = vec!['.'; input[0].len()];
    input.insert(0, period_vec.clone());
    input.push(period_vec.clone());
    let mut i = 0;
    while i < input.len() {
        let mut j = 0;
        while j < input[0].len() {
            let mut add = false;
            let mut curr = 0;
            while j < input[0].len() && input[i][j].is_ascii_digit() {
                add = add || is_part_number(&input, i as i32, j as i32);
                curr = curr * 10 + input[i][j].to_digit(10).expect("int");
                j += 1;
            }
            if add {
                sum += curr;
            }
            j += 1
        }
        i += 1
    }
    sum
}

pub fn day3_part2() -> u64 {
    let file = File::open("day-3.txt").expect("E");
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    let mut input: Vec<Vec<char>> = reader
        .lines()
        .map(|l| edit_string(l.expect("str")))
        .map(|s| s.chars().collect())
        .collect();

    let period_vec = vec!['.'; input[0].len()];
    input.insert(0, period_vec.clone());
    input.push(period_vec.clone());
    let mut i = 0;
    while i < input.len() {
        let mut j = 0;
        while j < input[0].len() {
            if input[i][j] == '*' {
                sum += get_gear_ratio(&input, i as i32, j as i32);
            }
            j += 1
        }
        i += 1
    }
    sum
}
