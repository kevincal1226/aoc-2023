use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day4_part1() -> u64 {
    let reader = BufReader::new(File::open("day-4.txt").unwrap());
    let mut input: Vec<Vec<Vec<u64>>> = Vec::new();
    reader.lines().for_each(|line| {
        let unwrapped = line.unwrap();
        let numbers: Vec<&str> = unwrapped.split(':').collect();
        let split_numbers: Vec<&str> = numbers[1].split('|').collect();

        input.push(
            split_numbers
                .iter()
                .map(|n| {
                    n.split_whitespace()
                        .map(|f| f.parse::<u64>().unwrap())
                        .collect()
                })
                .collect(),
        );
    });
    input
        .iter()
        .map(|v| 2u64.pow(v[1].iter().map(|e| v[0].contains(e) as u32).sum::<u32>() - 1))
        .sum()
}

pub fn day4_part2() -> u64 {
    let m: Vec<i32> = vec![1, 2, 3, 4];
    m.iter()
        .map(|n| {
            let x = 10;
            n * x
        })
        .sum::<i32>();
    0
}
