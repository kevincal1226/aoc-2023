use phf::phf_map;
use std::fs::File;
use std::io::{self, BufRead};

const ENGLISH_NUMBERS: phf::Map<&str, i32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9
};

fn helper(str: &str) -> i32 {
    for (key, value) in &ENGLISH_NUMBERS {
        if str.starts_with(key) || str.starts_with(&value.to_string()) {
            return *value;
        }
    }
    -1
}

pub fn day1() -> i32 {
    let file = File::open("day-1.txt").expect("E");
    let reader = io::BufReader::new(file);
    let mut sum: i32 = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut first = -1;
        let mut last = -1;
        for i in 0..line.len() {
            let temp = helper(&line[i..]);
            if temp != -1 {
                last = temp;
            }
            if first == -1 {
                first = last;
            }
        }
        sum += last + (first << 3) + (first << 1);
    }
    sum
}
