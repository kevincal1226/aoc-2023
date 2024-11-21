use std::io::BufRead;

fn check_if_valid(springs: &[char], groups: &[usize]) -> usize {
    let mut curr_groups: Vec<usize> = Vec::new();
    let mut curr_group_size = 0;
    springs.iter().for_each(|s| {
        if *s == '.' {
            if curr_group_size != 0 {
                curr_groups.push(curr_group_size);
            }
            curr_group_size = 0;
        } else {
            curr_group_size += 1;
        }
    });
    if curr_group_size != 0 {
        curr_groups.push(curr_group_size);
    }
    (curr_groups == *groups) as usize
}

fn check_perm(springs: &mut [char], groups: &[usize], index: usize) -> usize {
    if index == springs.len() {
        return check_if_valid(springs, groups);
    }
    if springs[index] != '?' {
        return check_perm(springs, groups, index + 1);
    }
    springs[index] = '.';
    let with_period = check_perm(springs, groups, index + 1);
    springs[index] = '#';
    let with_pound = check_perm(springs, groups, index + 1);
    springs[index] = '?';
    with_period + with_pound
}

pub fn part_1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("day-12.txt").unwrap());

    let mut input: Vec<(Vec<char>, Vec<usize>)> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();
            let nums: Vec<&str> = v[1].split_terminator(",").collect();
            (
                v[0].chars().collect(),
                nums.iter().map(|s| s.parse::<usize>().unwrap()).collect(),
            )
        })
        .collect();

    input
        .iter_mut()
        .map(|(springs, groups)| check_perm(springs, groups, 0))
        .sum()
}

pub fn part_2() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("day-12.txt").unwrap());

    let mut input: Vec<(Vec<char>, Vec<usize>)> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();
            let nums: Vec<&str> = v[1].split_terminator(",").collect();
            (
                v[0].chars().collect(),
                nums.iter().map(|s| s.parse::<usize>().unwrap()).collect(),
            )
        })
        .collect();

    input.iter_mut().for_each(|(springs, groups)| {
        let original_springs = springs.clone();
        let original_groups = groups.clone();
        (0..4).for_each(|_| {
            springs.push('?');
            springs.extend(original_springs.clone());
            groups.extend(original_groups.clone());
        });
    });

    input
        .iter_mut()
        .map(|(springs, groups)| check_perm(springs, groups, 0))
        .sum()
}
