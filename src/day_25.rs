use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

pub fn day25_part1() -> u64 {
    let file = File::open("day-25.txt").expect("file");
    let reader = io::BufReader::new(file);
    let mut adjlist: HashMap<String, HashSet<String>> = HashMap::new();
    reader.lines().for_each(|line| {
        let uline = line.unwrap();
        let key = uline[0..3].to_owned();
        uline[5..].split(' ').map(|s| s.to_owned()).for_each(|s| {
            adjlist.entry(s.clone()).or_default().insert(key.clone());
            adjlist.entry(key.clone()).or_default().insert(s.clone());
        });
    });
    let reps: HashMap<String, String> = adjlist.keys().map(|k| (k.clone(), k.clone())).collect();

    println!("{:#?}", adjlist);
    println!("{:#?}", reps);
    0
}
