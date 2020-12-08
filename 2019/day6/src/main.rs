use std::collections::HashMap;
use std::io::{self, BufRead};

#[rustfmt::skip]
fn p1_solve(orbits: &HashMap<String, String>) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();
    orbits.iter().map(|(child, _)| orbit_depth(orbits, &mut cache, child)).sum()
}

#[rustfmt::skip]
fn p2_solve(orbits: &HashMap<String, String>) -> usize {
    let you_path = orbit_path(orbits, "YOU");
    let san_path = orbit_path(orbits, "SAN");
    let common_path = you_path.iter().rev()
        .zip(san_path.iter().rev())
        .take_while(|(you, san)| you == san)
        .count();
    you_path.len() + san_path.len() - 2 * common_path
}

fn orbit_path(orbits: &HashMap<String, String>, star: &str) -> Vec<String> {
    let mut path = Vec::new();
    let mut star = star;
    while let Some(parent) = orbits.get(star) {
        path.push(parent.to_owned());
        star = parent;
    }
    path
}

fn orbit_depth(orbits: &HashMap<String, String>, cache: &mut HashMap<String, usize>, star: &str) -> usize {
    match cache.get(star) {
        Some(&depth) => depth,
        None => {
            let depth = match orbits.get(star) {
                Some(parent) => 1 + orbit_depth(orbits, cache, parent),
                None => 0,
            };
            cache.insert(star.to_owned(), depth);
            depth
        }
    }
}

#[rustfmt::skip]
fn main() {
    let inputs: HashMap<_, _> = io::stdin().lock().lines().map(|l| l.unwrap())
        .filter_map(|line| {
            let mut it = line.split(")");
            match (it.next(), it.next()) {
                (Some(parent), Some(child)) => Some((child.to_owned(), parent.to_owned())),
                _ => None
            }
        })
        .collect();

    let result = p1_solve(&inputs);
    println!("part1 result: {}", result);
    assert_eq!(117672, result);

    let result = p2_solve(&inputs);
    println!("part2 result: {}", result);
    assert_eq!(277, result);
}
