use std::io::{self, BufRead};

pub fn part_1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("day-6.txt").unwrap());
    let mut iter = reader.lines();
    let times: Vec<usize> = iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    let distances: Vec<usize> = iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let ways: Vec<usize> = times
        .iter()
        .zip(distances.iter())
        .map(|(time, dist)| {
            let mut ways = 0;
            for curr_time in 0..*time {
                if curr_time * (time - curr_time) > *dist {
                    ways += 1;
                }
            }
            ways
        })
        .collect();
    println!("{:?} {:?} {:?}", times, distances, ways);
    ways.iter().product()
}
pub fn part_2() -> u64 {
    let reader = std::io::BufReader::new(std::fs::File::open("day-6.txt").unwrap());
    let mut iter = reader.lines();
    let time: usize = iter
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|s| s.to_digit(10).unwrap())
        .fold(0, |acc, x| acc * 10 + x as usize);
    let distance: usize = iter
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|s| s.to_digit(10).unwrap())
        .fold(0, |acc, x| acc * 10 + x as usize);
    println!("{} {}", time, distance);
    let mut count = 0;

    for i in 0..time {
        if i * (time - i) > distance {
            count += 1;
        }
    }

    count
}
