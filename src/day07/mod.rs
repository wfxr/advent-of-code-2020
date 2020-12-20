use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> HashMap<String, Vec<(String, usize)>> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(' ').filter(|s| !s.is_empty());
            let color = [it.next().unwrap(), it.next().unwrap()].join(" ");
            let childs: Vec<(String, usize)> = it
                .skip(2)
                .collect::<Vec<_>>()
                .chunks(4)
                .filter_map(|g| match *g {
                    [n, c1, c2, _] => Some(([c1, c2].join(" "), n.parse().unwrap())),
                    _ => None,
                })
                .collect();
            (color, childs)
        })
        .collect()
}

const COLOR: &str = "shiny gold";

fn bag_size(rule: &HashMap<String, Vec<(String, usize)>>, color: &str) -> usize {
    match rule.get(color) {
        Some(childs) => 1 + childs.iter().fold(0, |acc, (color, n)| acc + n * bag_size(rule, color)),
        None => 0,
    }
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let mut result = HashSet::new();
    let mut s = HashSet::new();
    s.insert(COLOR);
    while !s.is_empty() {
        s = input
            .iter()
            .filter(|&(_, childs)| childs.iter().any(|(color, _)| s.contains(&color.as_ref())))
            .map(|(color, _)| {
                result.insert(color);
                color.as_ref()
            })
            .collect();
    }
    result.len()
}

fn part2(input: &str) -> usize {
    bag_size(&parse_input(input), COLOR) - 1
}

crate::solution!(part1 => 126, part2 => 220149);
