use nalgebra::base;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Hail {
    px: i128,
    py: i128,
    pz: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}

const MIN_XY: f64 = 200000000000000.0;
const MAX_XY: f64 = 400000000000000.0;

fn in_range(p1: f64, v1: f64, t1: f64, p2: f64, v2: f64, t2: f64, dependent: bool) -> bool {
    if dependent {
        (MIN_XY..=MAX_XY).contains(&p1) || (p1 + v1 - MIN_XY).abs() < (p1 - MIN_XY).abs()
    } else {
        (MIN_XY..=MAX_XY).contains(&(p1 + v1 * t1)) && (MIN_XY..=MAX_XY).contains(&(p2 + v2 * t2))
    }
}

fn rref(a: &mut [[f64; 2]; 2], b_col: &mut [f64; 2]) -> (f64, f64, bool, bool) {
    a[0][1] /= a[0][0];
    b_col[0] /= a[0][0];
    let fac = a[1][0] / a[0][0];
    a[1][0] -= a[0][0] * fac;
    a[1][1] -= a[0][1] * fac;
    b_col[1] -= b_col[0] * fac;
    if a[1][1] as i128 == 0 {
        if b_col[1] as i128 != 0 {
            (-1.0, -1.0, false, false)
        } else {
            (0.0, 0.0, true, true)
        }
    } else {
        b_col[1] /= a[1][1];
        a[1][1] /= a[1][1];
        let fac_2 = a[0][1] / a[1][1];
        b_col[0] -= b_col[1] * fac_2;
        a[0][1] = 0.0;
        (
            b_col[0],
            b_col[1],
            b_col[0] >= 0.0 && b_col[1] >= 0.0,
            false,
        )
    }
}

fn calc_time(a: &mut [[f64; 2]; 2], b_col: &mut [f64; 2]) -> (f64, f64, bool, bool) {
    let det_a = (a[0][0] * a[1][1] - (a[0][1] * a[1][0])) as i128;
    if det_a == 0 {
        rref(a, b_col)
    } else {
        let t1: f64 = (b_col[0] * a[1][1] - (a[0][1] * b_col[1])) / det_a as f64;
        let t2: f64 = (a[0][0] * b_col[1] - (b_col[0] * a[1][0])) / det_a as f64;
        (t1, t2, t1 >= 0.0 && t2 >= 0.0, false)
    }
}

fn has_xy_intersection(x1: &Hail, x2: &Hail) -> bool {
    let mut a_matrix = [[x1.vx as f64, -x2.vx as f64], [x1.vy as f64, -x2.vy as f64]];
    let mut b_col = [x2.px as f64 - x1.px as f64, x2.py as f64 - x1.py as f64];
    let (tx1, tx2, valid_time, t2_free) = calc_time(&mut a_matrix, &mut b_col);
    valid_time
        && in_range(
            x1.px as f64,
            x1.vx as f64,
            tx1,
            x2.px as f64,
            x2.vx as f64,
            tx2,
            t2_free,
        )
        && in_range(
            x1.py as f64,
            x1.vy as f64,
            tx1,
            x2.py as f64,
            x2.vy as f64,
            tx2,
            t2_free,
        )
}

fn parse_from_str(line: &str) -> Hail {
    let strs: Vec<&str> = line.split(", ").flat_map(|l| l.split(" @ ")).collect();
    Hail {
        px: strs[0].parse::<i128>().unwrap(),
        py: strs[1].parse::<i128>().unwrap(),
        pz: strs[2].parse::<i128>().unwrap(),
        vx: strs[3].parse::<i128>().unwrap(),
        vy: strs[4].parse::<i128>().unwrap(),
        vz: strs[5].parse::<i128>().unwrap(),
    }
}

pub fn day24_part1() -> i128 {
    let file = File::open("day-24.txt").expect("E");
    let reader = io::BufReader::new(file);
    let hailstones: Vec<Hail> = reader
        .lines()
        .map(|line| parse_from_str(line.unwrap().as_str()))
        .collect();

    let mut num_intersections: i128 = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            num_intersections += has_xy_intersection(&hailstones[i], &hailstones[j]) as i128;
        }
    }

    num_intersections
}

pub fn day24_part2() -> i128 {
    let file = File::open("day-24.txt").expect("file");
    let reader = io::BufReader::new(file);
    let hailstones: Vec<Hail> = reader
        .lines()
        .map(|line| parse_from_str(line.unwrap().as_str()))
        .collect();

    let position_0 = nalgebra::Matrix3x1::new(hailstones[0].px, hailstones[0].py, hailstones[0].pz);

    let position_1 = nalgebra::Matrix3x1::new(hailstones[1].px, hailstones[1].py, hailstones[1].pz);
    let position_2 = nalgebra::Matrix3x1::new(hailstones[2].px, hailstones[2].py, hailstones[2].pz);

    let velocity_0 = nalgebra::Matrix3x1::new(hailstones[0].vx, hailstones[0].vy, hailstones[0].vz);
    let velocity_1 = nalgebra::Matrix3x1::new(hailstones[1].vx, hailstones[1].vy, hailstones[1].vz);
    let velocity_2 = nalgebra::Matrix3x1::new(hailstones[2].vx, hailstones[2].vy, hailstones[2].vz);

    let p1_col_mtx = position_1 - position_0;
    let p2_col_mtx = position_2 - position_0;
    let v1_col_mtx = velocity_1 - velocity_0;
    let v2_col_mtx = velocity_2 - velocity_0;

    let t1 = -((p1_col_mtx.cross(&p2_col_mtx)).dot(&v2_col_mtx))
        / ((v1_col_mtx.cross(&p2_col_mtx)).dot(&v2_col_mtx));

    let t2 = -((p1_col_mtx.cross(&p2_col_mtx)).dot(&v1_col_mtx))
        / ((p1_col_mtx.cross(&v2_col_mtx)).dot(&v1_col_mtx));

    let c1 = position_1 + velocity_1 * t1;
    let c2 = position_2 + velocity_2 * t2;
    let v = (c2 - c1) / (t2 - t1);
    let p = c1 - v * t1;
    println!("{}", p);
    p.sum()
}
