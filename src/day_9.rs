use std::io::{self, BufRead};

pub fn part_1() -> i64 {
    let reader = std::io::BufReader::new(std::fs::File::open("day-9.txt").unwrap());
    let input: Vec<Vec<i64>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let sw: Vec<&str> = line.split_whitespace().collect();
            sw.iter().map(|s| s.parse::<i64>().unwrap()).collect()
        })
        .collect();

    input
        .iter()
        .map(|v| {
            let mut simulation: Vec<Vec<i64>> = vec![v.clone()];
            while !simulation.last().unwrap().windows(2).all(|w| w[0] == w[1]) {
                simulation.push(
                    simulation
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect(),
                );
            }
            for i in (0..simulation.len() - 1).rev() {
                let h = *simulation[i].last_mut().unwrap();
                let k = *simulation[i + 1].last_mut().unwrap();
                simulation[i].push(h + k);
            }

            *simulation[0].last().unwrap()
        })
        .sum()
}

pub fn part_2() -> i64 {
    let reader = std::io::BufReader::new(std::fs::File::open("day-9.txt").unwrap());
    let input: Vec<Vec<i64>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let sw: Vec<&str> = line.split_whitespace().collect();
            sw.iter().map(|s| s.parse::<i64>().unwrap()).collect()
        })
        .collect();

    input
        .iter()
        .map(|v| {
            let mut simulation: Vec<Vec<i64>> = vec![v.clone()];
            while !simulation.last().unwrap().windows(2).all(|w| w[0] == w[1]) {
                simulation.push(
                    simulation
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect(),
                );
            }
            for i in (0..simulation.len() - 1).rev() {
                let h = simulation[i][0];
                let k = simulation[i + 1][0];
                simulation[i].insert(0, h - k);
            }

            simulation[0][0]
        })
        .sum()
}
