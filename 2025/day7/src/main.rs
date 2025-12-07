use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    let mut beams = HashMap::new();
    let mut rows = Vec::new();
    for line in io::stdin().lines() {
        let mut splitters = HashSet::new();
        for (i, c) in line.unwrap().chars().enumerate() {
            match c {
                '.' => (),
                'S' => {
                    beams.insert(i, 1 as u64);
                }
                '^' => {
                    splitters.insert(i);
                }
                _ => panic!("Invalid character"),
            }
        }
        if !splitters.is_empty() {
            rows.push(splitters)
        }
    }

    let mut total_splits = 0;
    for splitters in rows.iter() {
        let mut new_beams = HashMap::new();
        for (&b, &num) in beams.iter() {
            if splitters.contains(&b) {
                total_splits += 1;
                *new_beams.entry(b - 1).or_insert(0) += num;
                *new_beams.entry(b + 1).or_insert(0) += num;
            } else {
                *new_beams.entry(b).or_insert(0) += num;
            }
        }
        beams = new_beams;
    }
    println!("A total_splits: {total_splits}");

    let total_paths = beams.values().sum::<u64>();
    println!("B total_paths: {total_paths}");
}
