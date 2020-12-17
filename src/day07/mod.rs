use crate::Solution;
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

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
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
        Ok(result.len().to_string())
    },
    part2: |input| {
        let input = parse_input(input);
        let result = bag_size(&input, COLOR) - 1;
        Ok(result.to_string())
    },
};

fn bag_size(rule: &HashMap<String, Vec<(String, usize)>>, color: &str) -> usize {
    match rule.get(color) {
        Some(childs) => 1 + childs.iter().fold(0, |acc, (color, n)| acc + n * bag_size(rule, color)),
        None => 0,
    }
}

#[cfg(test)]
crate::solution_test!(126, 220149);
