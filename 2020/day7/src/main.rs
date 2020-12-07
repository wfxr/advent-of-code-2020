use std::collections::HashSet;
use std::io::{self, BufRead};

struct Bag {
    color:  String,
    childs: Vec<(String, usize)>,
}

#[rustfmt::skip]
fn p1_solve(rule: &[Bag]) -> usize {
    let mut result = HashSet::new();

    let mut s = HashSet::new();
    s.insert("shiny gold".to_owned());
    while !s.is_empty() {
        s = rule.iter()
            .filter(|bag| bag.childs.iter().any(|(color, _)| s.contains(color)))
            .map(|bag| {
                result.insert(bag.color.clone());
                bag.color.clone()
            })
            .collect();
    }
    result.len()
}

#[rustfmt::skip]
fn main() {
    let inputs: Vec<_> = io::stdin().lock().lines().map(|l| l.unwrap())
        .map(|line| {
            let mut it = line.split(' ').filter(|s| !s.is_empty());
            let color = [it.next().unwrap(), it.next().unwrap()].join(" ");
            let childs: Vec<(String, usize)> = it.skip(2).collect::<Vec<_>>().chunks(4)
                .filter_map(|g| match g {
                    &[n, c1, c2, _] => Some(([c1, c2].join(" "), n.parse().unwrap())),
                    _ => None,
                })
                .collect();
            Bag { color, childs }
        })
        .collect();

    let result = p1_solve(&inputs);
    println!("part 1 result: {}", result);
    assert_eq!(126, result);
}
