use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

pub fn day2() -> u64 {
    let file = File::open("day-2.txt").expect("E");
    let reader = io::BufReader::new(file);
    let mut sum: u64 = 0;
    for line in reader.lines() {
        let e = line.expect("str");
        let mut iter = e.split_whitespace();
        let _ = iter.next();
        let _ = iter.next();
        let mut num_red: u64 = 0;
        let mut num_green: u64 = 0;
        let mut num_blue: u64 = 0;

        while let Some(amount) = iter.next() {
            let color = iter.next().expect("not none");
            let val = amount.parse::<u64>().unwrap();
            if color[0..1] == *"r" {
                num_red = cmp::max(num_red, val);
            } else if color[0..1] == *"g" {
                num_green = cmp::max(num_green, val);
            } else if color[0..1] == *"b" {
                num_blue = cmp::max(num_blue, val);
            }
        }
        sum += num_red * num_green * num_blue;
    }
    sum
}
