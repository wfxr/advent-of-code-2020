use crate::{solution, Result};
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Result<HashMap<String, Vec<(String, usize)>>> {
    let color_of = |part1, part2| format!("{} {}", part1, part2);
    input
        .lines()
        .map(|line| {
            let mut it = line.split(' ').filter(|s| !s.is_empty());
            let color = color_of(
                it.next().ok_or("missing color part1")?,
                it.next().ok_or("missing color part2")?,
            );
            let childs: Vec<(String, usize)> = it
                .skip(2)
                .collect::<Vec<_>>()
                .chunks(4)
                .filter(|g| g.len() == 4)
                .map(|g| Ok((color_of(g[1], g[2]), g[0].parse()?)))
                .collect::<Result<_>>()?;
            Ok((color, childs))
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

fn part1(input: &str) -> Result<usize> {
    let input = parse_input(input)?;
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
    Ok(result.len())
}

fn part2(input: &str) -> Result<usize> {
    Ok(bag_size(&parse_input(input)?, COLOR) - 1)
}

solution!(part1 => 126, part2 => 220149);
