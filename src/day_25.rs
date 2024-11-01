use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

fn set_find(vertex: &String, reps: &mut HashMap<String, String>) -> String {
    let mut start = vertex.clone();
    while reps[vertex] != *vertex {
        let temp = reps.entry(start.clone());
        reps.insert(start, *vertex);
    }
    "".to_owned()
}

fn set_union(vertex_a: &String, vertex_b: &String, reps: &mut HashMap<String, String>) {
    reps.insert(set_find(vertex_b, reps), set_find(vertex_a, reps));
}

fn count_comps(edges: &Vec<(String, String)>, reps: &mut HashMap<String, String>) -> usize {
    for (i, j) in edges {
        set_union(i, j, reps);
    }
    reps.clone()
        .iter()
        .filter(|(rep, _)| set_find(rep, reps) == **rep)
        .count()
}

pub fn day25_part1() -> u64 {
    let file = File::open("sample.txt").expect("file");
    let reader = io::BufReader::new(file);
    //let mut adjlist: HashMap<String, BTreeSet<String>> = HashMap::new();
    //reader.lines().for_each(|line| {
    //    let uline = line.unwrap();
    //    let key = uline[0..3].to_owned();
    //    uline[5..].split(' ').map(|s| s.to_owned()).for_each(|s| {
    //        adjlist.entry(s.clone()).or_default().insert(key.clone());
    //        adjlist.entry(key.clone()).or_default().insert(s.clone());
    //    });
    //});
    //let reps: HashMap<String, String> = adjlist.keys().map(|k| (k.clone(), k.clone())).collect();
    let mut edges: Vec<(String, String)> = Vec::new();
    let mut reps: HashMap<String, String> = HashMap::new();
    reader.lines().for_each(|line| {
        let uline = line.unwrap();
        let first = uline[0..3].to_owned();
        uline[5..].split(' ').map(|s| s.to_owned()).for_each(|s| {
            edges.push((first.clone(), s.clone()));
            reps.insert(first.clone(), first.clone());
            reps.insert(s.clone(), s.clone());
        });
    });

    println!("{:#?}", edges);
    println!("{:#?}", reps);

    0
}
