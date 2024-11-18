use std::io::{self, BufRead};

fn get_score(cards: &String) -> usize {
    let mut frequencies: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    cards
        .chars()
        .for_each(|c| match frequencies.contains_key(&c) {
            true => {
                *frequencies.get_mut(&c).unwrap() += 1;
            }
            false => {
                frequencies.insert(c, 1);
            }
        });
    let mut inverse_freqs: std::collections::HashMap<usize, std::collections::HashSet<char>> =
        std::collections::HashMap::new();
    frequencies
        .iter()
        .for_each(|(k, v)| match inverse_freqs.contains_key(v) {
            true => {
                (*inverse_freqs.get_mut(v).unwrap()).insert(*k);
            }
            false => {
                inverse_freqs.insert(*v, std::collections::HashSet::from([*k]));
            }
        });

    if inverse_freqs.contains_key(&5usize) {
        6
    } else if inverse_freqs.contains_key(&4usize) {
        5
    } else if inverse_freqs.contains_key(&3usize) && inverse_freqs.contains_key(&2usize) {
        4
    } else if inverse_freqs.contains_key(&3usize) {
        3
    } else if inverse_freqs.contains_key(&2usize) {
        if inverse_freqs.get(&2usize).unwrap().len() >= 2 {
            2
        } else {
            1
        }
    } else {
        0
    }
}

pub fn part_1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("day-7.txt").unwrap());
    let mut input: Vec<(String, usize)> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let pair: Vec<&str> = line.split_whitespace().collect();
            (
                pair[0]
                    .replace("T", "a")
                    .replace("J", "b")
                    .replace("Q", "c")
                    .replace("K", "d")
                    .replace("A", "e")
                    .to_owned(),
                pair[1].parse::<usize>().unwrap(),
            )
        })
        .collect();

    input.sort_by(|lhs, rhs| {
        if get_score(&lhs.0) == get_score(&rhs.0) {
            lhs.0.cmp(&rhs.0)
        } else {
            get_score(&lhs.0).cmp(&get_score(&rhs.0))
        }
    });

    println!("{:#?}", input);
    let mut rank = 0;

    input
        .iter()
        .map(|(_, val)| {
            rank += 1;
            val * rank
        })
        .sum()
}

fn get_score_joker(cards: &String) -> usize {
    let mut frequencies: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    let mut num_jokers = 0;
    cards.chars().for_each(|c| {
        if c == '1' {
            num_jokers += 1;
            return;
        }
        match frequencies.contains_key(&c) {
            true => {
                *frequencies.get_mut(&c).unwrap() += 1;
            }
            false => {
                frequencies.insert(c, 1);
            }
        }
    });
    let mut inverse_freqs: std::collections::HashMap<usize, std::collections::HashSet<char>> =
        std::collections::HashMap::new();
    frequencies
        .iter()
        .for_each(|(k, v)| match inverse_freqs.contains_key(v) {
            true => {
                (*inverse_freqs.get_mut(v).unwrap()).insert(*k);
            }
            false => {
                inverse_freqs.insert(*v, std::collections::HashSet::from([*k]));
            }
        });

    if inverse_freqs.contains_key(&5usize) {
        6
    } else if inverse_freqs.contains_key(&4usize) {
        5 + num_jokers
    } else if inverse_freqs.contains_key(&3usize) && inverse_freqs.contains_key(&2usize) {
        4 + num_jokers
    } else if inverse_freqs.contains_key(&3usize) {
        if num_jokers == 0 {
            3
        } else {
            4 + num_jokers
        }
    } else if inverse_freqs.contains_key(&2usize) {
        if inverse_freqs.get(&2usize).unwrap().len() >= 2 {
            if num_jokers == 0 {
                2
            } else {
                3 + num_jokers
            }
        } else {
            if num_jokers == 0 {
                1
            } else if num_jokers == 1 {
                3
            } else if num_jokers == 2 {
                5
            } else {
                6
            }
        }
    } else {
        if num_jokers == 0 {
            0
        } else if num_jokers == 1 {
            1
        } else if num_jokers == 2 {
            3
        } else if num_jokers == 3 {
            5
        } else {
            6
        }
    }
}

pub fn part_2() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("day-7.txt").unwrap());
    let mut input: Vec<(String, usize)> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let pair: Vec<&str> = line.split_whitespace().collect();
            (
                pair[0]
                    .replace("T", "a")
                    .replace("J", "1")
                    .replace("Q", "c")
                    .replace("K", "d")
                    .replace("A", "e")
                    .to_owned(),
                pair[1].parse::<usize>().unwrap(),
            )
        })
        .collect();

    input.sort_by(|lhs, rhs| {
        if get_score_joker(&lhs.0) == get_score_joker(&rhs.0) {
            lhs.0.cmp(&rhs.0)
        } else {
            get_score_joker(&lhs.0).cmp(&get_score_joker(&rhs.0))
        }
    });

    println!("{:#?}", input);
    let mut rank = 0;

    input
        .iter()
        .map(|(_, val)| {
            rank += 1;
            val * rank
        })
        .sum()
}
