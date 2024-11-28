use std::collections::HashMap;
use std::io::BufRead;

pub fn part_1() -> usize {
    std::io::BufReader::new(std::fs::File::open("day-15.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_terminator(',')
        .map(|s| {
            s.chars()
                .fold(0, |acc, ele| ((acc + ele as usize) * 17) % 256)
        })
        .sum()
}

pub fn part_2() -> usize {
    let mut boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();
    std::io::BufReader::new(std::fs::File::open("day-15.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_terminator(',')
        .for_each(|s| {
            let box_index = s
                .chars()
                .filter(|c| c.is_alphabetic())
                .fold(0, |acc, ele| ((acc + ele as usize) * 17) % 256);
            let key: String = s.chars().filter(|c| c.is_alphabetic()).collect();
            let mut op = s.chars().filter(|c| !c.is_alphabetic());
            let curr_box: &mut Vec<(String, usize)> = boxes.entry(box_index).or_default();
            let index_to_edit = curr_box.iter().position(|e| e.0 == key);
            match op.next().unwrap() {
                '-' => {
                    if let Some(index_to_edit) = index_to_edit {
                        curr_box.remove(index_to_edit);
                    }
                }
                '=' => {
                    if let Some(index_to_edit) = index_to_edit {
                        curr_box[index_to_edit].1 =
                            op.next().unwrap().to_digit(10).unwrap() as usize;
                    } else {
                        curr_box.push((key, op.next().unwrap().to_digit(10).unwrap() as usize));
                    }
                }
                _ => panic!("expected - or ="),
            };
        });
    boxes
        .iter()
        .map(|(box_index, v)| {
            v.iter().enumerate().fold(0, |acc, (slot, (_, focal_len))| {
                acc + (box_index + 1) * (slot + 1) * focal_len
            })
        })
        .sum()
}
