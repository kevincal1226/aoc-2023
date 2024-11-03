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

pub fn day4_part1_cursed() -> u64 {
    let input: Vec<Vec<Vec<u64>>> = BufReader::new(File::open("day-4.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|unwrapped| unwrapped.split(':').map(|s| s.to_string()).collect())
        .map(|nums: Vec<String>| nums[1].split('|').map(|s| s.to_string()).collect())
        .map(|split_nums: Vec<String>| {
            split_nums
                .iter()
                .map(|n| {
                    n.split_whitespace()
                        .map(|f| f.parse::<u64>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    input
        .iter()
        .map(|v| 2u64.pow(v[1].iter().map(|e| v[0].contains(e) as u32).sum::<u32>() - 1))
        .sum()
}

pub fn day4_part2() -> u64 {
    let input: Vec<Vec<Vec<u64>>> = BufReader::new(File::open("day-4.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|unwrapped| unwrapped.split(':').map(|s| s.to_string()).collect())
        .map(|nums: Vec<String>| nums[1].split('|').map(|s| s.to_string()).collect())
        .map(|split_nums: Vec<String>| {
            split_nums
                .iter()
                .map(|n| {
                    n.split_whitespace()
                        .map(|f| f.parse::<u64>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut card_count: Vec<u64> = vec![1; input.len()];
    for i in 0..input.len() {
        let end: usize = input[i][1]
            .iter()
            .map(|f| input[i][0].contains(f) as usize)
            .sum();

        for j in i + 1..i + end + 1 {
            card_count[j] += card_count[i];
        }
    }
    println!("{:?}", card_count);
    card_count.iter().sum()
}
