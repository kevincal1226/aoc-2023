use std::io::BufRead;

pub fn part_1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("day-8.txt").unwrap());
    let mut iter = reader.lines();
    let mut edges: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let instructions: Vec<char> = iter.next().unwrap().unwrap().chars().collect();
    iter.next();
    iter.map(|i| i.unwrap()).for_each(|s| {
        let (k, e) = s.split_once('=').unwrap();
        let (e1, e2) = e.split_once(',').unwrap();
        edges
            .entry(k.trim().to_owned())
            .or_default()
            .extend(vec![e1.trim().replace("(", ""), e2.trim().replace(")", "")]);
    });

    let mut curr: String = "AAA".to_owned();
    let mut i = 0;

    while curr != "ZZZ" {
        match instructions[i % instructions.len()] {
            'L' => {
                curr = edges.get(&curr).unwrap()[0].clone();
            }
            'R' => {
                curr = edges.get(&curr).unwrap()[1].clone();
            }
            _ => {
                panic!("Not L or R");
            }
        }
        i += 1;
    }

    i
}

pub fn part_2() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("day-8.txt").unwrap());
    let mut iter = reader.lines();
    let mut edges: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let instructions: Vec<char> = iter.next().unwrap().unwrap().chars().collect();
    iter.next();
    iter.map(|i| i.unwrap()).for_each(|s| {
        let (k, e) = s.split_once('=').unwrap();
        let (e1, e2) = e.split_once(',').unwrap();
        edges
            .entry(k.trim().to_owned())
            .or_default()
            .extend(vec![e1.trim().replace("(", ""), e2.trim().replace(")", "")]);
    });

    let mut curr_nodes: Vec<String> = edges
        .keys()
        .map(|k| k.clone())
        .filter(|k| k.ends_with("A"))
        .collect();

    let mut distances: Vec<usize> = Vec::new();

    println!("{:?}", curr_nodes);

    curr_nodes.iter().for_each(|n| {
        let mut i = 0;
        let mut node = n.clone();
        while !node.ends_with("Z") {
            match instructions[i % instructions.len()] {
                'L' => {
                    node = edges.get(&node).unwrap()[0].clone();
                }
                'R' => {
                    node = edges.get(&node).unwrap()[1].clone();
                }
                _ => {
                    panic!("Not L or R");
                }
            }
            i += 1;
        }
        distances.push(i);
    });

    println!("{:?}", distances);

    distances
        .iter()
        .fold(1, |acc, x| num::integer::lcm(acc, *x))
}
