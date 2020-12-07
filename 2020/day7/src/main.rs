use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[rustfmt::skip]
fn p1_solve(rule: &HashMap<String, Vec<(String, usize)>>, color: &str) -> usize {
    let mut result = HashSet::new();
    let mut s = HashSet::new();
    s.insert(color.to_owned());
    while !s.is_empty() {
        s = rule.iter()
            .filter(|(_, childs)| childs.iter().any(|(color, _)| s.contains(color)))
            .map(|(color, _)| {
                result.insert(color.clone());
                color.clone()
            })
            .collect();
    }
    result.len()
}

#[rustfmt::skip]
fn p2_solve(rule: &HashMap<String, Vec<(String, usize)>>, color: &str) -> usize {
    match rule.get(color) {
        Some(childs) => {
            1 + childs.iter().fold(0, |acc, (color, size)| acc + size * p2_solve(rule, color))
        }
        None => 0,
    }
}

#[rustfmt::skip]
fn main() {
    let inputs: HashMap<_, _> = io::stdin().lock().lines().map(|l| l.unwrap())
        .map(|line| {
            let mut it = line.split(' ').filter(|s| !s.is_empty());
            let color = [it.next().unwrap(), it.next().unwrap()].join(" ");
            let childs: Vec<(String, usize)> = it.skip(2).collect::<Vec<_>>().chunks(4)
                .filter_map(|g| match *g {
                    [n, c1, c2, _] => Some(([c1, c2].join(" "), n.parse().unwrap())),
                    _ => None,
                })
                .collect();
            (color, childs)
        })
        .collect();
    let color = "shiny gold";

    let result = p1_solve(&inputs, color);
    println!("part 1 result: {}", result);
    assert_eq!(126, result);

    let result = p2_solve(&inputs, color) - 1;
    println!("part 2 result: {}", result);
    assert_eq!(220149, result);
}
