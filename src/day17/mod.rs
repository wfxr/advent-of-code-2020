use crate::Solution;
use std::collections::HashSet;

type Point3D = (i32, i32, i32);
type Point4D = (i32, i32, i32, i32);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn unit3d_actived(matrix: &HashSet<Point3D>, (x, y, z): &Point3D) -> usize {
    let mut count = 0;
    for dx in -1..2 {
        for dy in -1..2 {
            for dz in -1..2 {
                count += matrix.contains(&(x + dx, y + dy, z + dz)) as usize
            }
        }
    }
    count
}

fn unit4d_actived(matrix: &HashSet<Point4D>, (x, y, z, w): &Point4D) -> usize {
    let mut count = 0;
    for dx in -1..2 {
        for dy in -1..2 {
            for dz in -1..2 {
                for dw in -1..2 {
                    count += matrix.contains(&(x + dx, y + dy, z + dz, w + dw)) as usize
                }
            }
        }
    }
    count
}

fn part1_proceed(curr: &mut HashSet<Point3D>) {
    let mut next = curr.clone();
    let mut calculated = HashSet::new();
    curr.iter().for_each(|(x, y, z)| {
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    let p = (x + dx, y + dy, z + dz);
                    if calculated.contains(&p) {
                        continue;
                    }
                    let state = curr.contains(&p);
                    let next_state = next_state(state, unit3d_actived(&curr, &p));
                    match (state, next_state) {
                        (true, false) => next.remove(&p),
                        (false, true) => next.insert(p),
                        _ => false,
                    };
                    calculated.insert(p);
                }
            }
        }
    });
    *curr = next;
}

fn next_state(state: bool, count: usize) -> bool {
    match state {
        true => count == 3 || count == 4,
        false => count == 3,
    }
}

fn part2_proceed(curr: &mut HashSet<Point4D>) {
    let mut next = curr.clone();
    let mut calculated = HashSet::new();
    curr.iter().for_each(|(x, y, z, w)| {
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    for dw in -1..2 {
                        let p = (x + dx, y + dy, z + dz, w + dw);
                        if calculated.contains(&p) {
                            continue;
                        }
                        let state = curr.contains(&p);
                        let next_state = next_state(state, unit4d_actived(&curr, &p));
                        match (state, next_state) {
                            (true, false) => next.remove(&p),
                            (false, true) => next.insert(p),
                            _ => false,
                        };
                        calculated.insert(p);
                    }
                }
            }
        }
    });
    *curr = next;
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let input = parse_input(input);
        let mut matrix = HashSet::new();
        for x in 0..input[0].len() {
            for y in 0..input.len() {
                if input[x][y] == '#' {
                    matrix.insert((x as i32, y as i32, 0));
                }
            }
        }
        for _ in 0..6 {
            part1_proceed(&mut matrix);
        }
        let result = matrix.len();
        Ok(result.to_string())
    },
    part2: |input| {
        let input = parse_input(input);
        let mut matrix = HashSet::new();
        for x in 0..input[0].len() {
            for y in 0..input.len() {
                if input[x][y] == '#' {
                    matrix.insert((x as i32, y as i32, 0, 0));
                }
            }
        }
        for _ in 0..6 {
            part2_proceed(&mut matrix);
        }
        let result = matrix.len();
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(202, 2028);
