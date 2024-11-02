use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::net::ToSocketAddrs;

fn set_find(vertex: &mut String, reps: &mut HashMap<String, String>) -> String {
    let mut start = vertex.clone();
    while reps[vertex] != *vertex {
        *vertex = reps[vertex].clone();
    }

    while reps[&start] != *vertex {
        let temp = reps[&start].clone();
        let res = (*vertex).clone();
        reps.entry(start).and_modify(|s| *s = res);
        start = temp;
    }

    (*vertex).clone()
}

fn set_union(vertex_a: &mut String, vertex_b: &mut String, reps: &mut HashMap<String, String>) {
    let key = set_find(vertex_b, reps);
    let value = set_find(vertex_a, reps);
    reps.entry(key).and_modify(|s| *s = value);
}

fn count_comps(edges: &HashSet<(String, String)>, reps: &mut HashMap<String, String>) -> usize {
    for (i, j) in edges {
        set_union(&mut i.clone(), &mut j.clone(), reps);
    }
    reps.clone()
        .iter()
        .filter(|(rep, _)| set_find(&mut (**rep).clone(), reps) == **rep)
        .count()
}

pub fn day25_part1() -> u64 {
    let file = File::open("day-25.txt").expect("file");
    let reader = io::BufReader::new(file);
    let mut edges: HashSet<(String, String)> = HashSet::new();
    let mut reps: HashMap<String, String> = HashMap::new();
    reader.lines().for_each(|line| {
        let uline = line.unwrap();
        let first = uline[0..3].to_owned();
        uline[5..].split(' ').map(|s| s.to_owned()).for_each(|s| {
            edges.insert((first.clone(), s.clone()));
            reps.insert(first.clone(), first.clone());
            reps.insert(s.clone(), s.clone());
        });
    });

    let cloned_edges = edges.clone();
    println!("{}", edges.len());
    let mut found = false;

    cloned_edges.iter().for_each(|e| {
        if found {
            return;
        }
        edges.remove(e);
        cloned_edges.iter().for_each(|f| {
            if f == e || found {
                return;
            }
            edges.remove(f);
            cloned_edges.iter().for_each(|g| {
                if g == f || g == e || found {
                    return;
                }
                edges.remove(g);
                let mut temp_reps = reps.clone();
                if count_comps(&edges, &mut temp_reps) == 2 {
                    println!("Found by removing edges {:?} {:?} {:?}", e, f, g);
                    found = true;
                }
                if found {
                    return;
                }
                edges.insert(g.clone());
            });
            if found {
                return;
            }
            edges.insert(f.clone());
        });
        if found {
            return;
        }
        edges.insert(e.clone());
    });

    println!("{}", count_comps(&edges, &mut reps));

    let final_reps: HashMap<String, u64> = reps
        .iter()
        .map(|p| (p.1.clone(), reps.iter().map(|f| (f.1 == p.1) as u64).sum()))
        .collect();

    println!("{:?}", final_reps);

    final_reps.iter().map(|p| p.1).product()
}
