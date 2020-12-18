use crate::Solution;
use std::collections::{HashMap, HashSet};

type Point = (i8, i8, i8, i8);

#[rustfmt::skip]
const NEIGHBORS: &[Point] = &[
    (-1, 0, 0, 0), ( 1, 0, 0, 0), ( 0,-1, 0, 0), (-1,-1, 0, 0), ( 1,-1, 0, 0),
    ( 0, 1, 0, 0), (-1, 1, 0, 0), ( 1, 1, 0, 0), ( 0, 0,-1, 0), (-1, 0,-1, 0),
    ( 1, 0,-1, 0), ( 0,-1,-1, 0), (-1,-1,-1, 0), ( 1,-1,-1, 0), ( 0, 1,-1, 0),
    (-1, 1,-1, 0), ( 1, 1,-1, 0), ( 0, 0, 1, 0), (-1, 0, 1, 0), ( 1, 0, 1, 0),
    ( 0,-1, 1, 0), (-1,-1, 1, 0), ( 1,-1, 1, 0), ( 0, 1, 1, 0), (-1, 1, 1, 0),
    ( 1, 1, 1, 0), ( 0, 0, 0,-1), (-1, 0, 0,-1), ( 1, 0, 0,-1), ( 0,-1, 0,-1),
    (-1,-1, 0,-1), ( 1,-1, 0,-1), ( 0, 1, 0,-1), (-1, 1, 0,-1), ( 1, 1, 0,-1),
    ( 0, 0,-1,-1), (-1, 0,-1,-1), ( 1, 0,-1,-1), ( 0,-1,-1,-1), (-1,-1,-1,-1),
    ( 1,-1,-1,-1), ( 0, 1,-1,-1), (-1, 1,-1,-1), ( 1, 1,-1,-1), ( 0, 0, 1,-1),
    (-1, 0, 1,-1), ( 1, 0, 1,-1), ( 0,-1, 1,-1), (-1,-1, 1,-1), ( 1,-1, 1,-1),
    ( 0, 1, 1,-1), (-1, 1, 1,-1), ( 1, 1, 1,-1), ( 0, 0, 0, 1), (-1, 0, 0, 1),
    ( 1, 0, 0, 1), ( 0,-1, 0, 1), (-1,-1, 0, 1), ( 1,-1, 0, 1), ( 0, 1, 0, 1),
    (-1, 1, 0, 1), ( 1, 1, 0, 1), ( 0, 0,-1, 1), (-1, 0,-1, 1), ( 1, 0,-1, 1),
    ( 0,-1,-1, 1), (-1,-1,-1, 1), ( 1,-1,-1, 1), ( 0, 1,-1, 1), (-1, 1,-1, 1),
    ( 1, 1,-1, 1), ( 0, 0, 1, 1), (-1, 0, 1, 1), ( 1, 0, 1, 1), ( 0,-1, 1, 1),
    (-1,-1, 1, 1), ( 1,-1, 1, 1), ( 0, 1, 1, 1), (-1, 1, 1, 1), ( 1, 1, 1, 1),
];

fn count_neighbors(matrix: &HashSet<Point>, offsets: &[Point]) -> HashMap<Point, usize> {
    let mut m = HashMap::new();
    matrix.iter().for_each(|(x, y, z, w)| {
        offsets.iter().for_each(|(dx, dy, dz, dw)| {
            *m.entry((x + dx, y + dy, z + dz, w + dw)).or_insert(0) += 1;
        })
    });
    m
}

fn solve(input: &str, neighbors: &[Point]) -> usize {
    let mut matrix: HashSet<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(y, c)| if c == '#' { Some((x as i8, y as i8, 0, 0)) } else { None })
        })
        .collect();
    for _ in 0..6 {
        matrix = count_neighbors(&matrix, neighbors)
            .iter()
            .filter_map(|(p, count)| match (matrix.contains(p), count) {
                (true, 2) | (_, 3) => Some(*p),
                _ => None,
            })
            .collect();
    }
    matrix.len()
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| Ok(solve(input, &NEIGHBORS[..26]).to_string()),
    part2: |input| Ok(solve(input, NEIGHBORS).to_string()),
};

#[cfg(test)]
crate::solution_test!(202, 2028);
