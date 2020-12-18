use crate::Solution;
use std::collections::HashSet;

type Point3D = (i32, i32, i32);
type Point4D = (i32, i32, i32, i32);
type Range = std::ops::Range<i32>;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn unit3d_actived(matrix: &HashSet<Point3D>, (x, y, z): Point3D) -> usize {
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

fn unit4d_actived(matrix: &HashSet<Point4D>, (x, y, z, w): Point4D) -> usize {
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

fn part1_proceed(curr: &mut HashSet<Point3D>, (range_x, range_y, range_z): &mut (Range, Range, Range)) {
    let mut next = curr.clone();
    for x in range_x.clone() {
        for y in range_y.clone() {
            for z in range_z.clone() {
                let p = (x, y, z);
                match (curr.contains(&p), unit3d_actived(&curr, p)) {
                    (_, 3) | (true, 4) => next.insert(p),
                    _ => next.remove(&p),
                };
            }
        }
    }
    range_x.start -= 1;
    range_x.end += 1;
    range_y.start -= 1;
    range_y.end += 1;
    range_z.start -= 1;
    range_z.end += 1;
    *curr = next;
}

fn part2_proceed(curr: &mut HashSet<Point4D>, (range_x, range_y, range_z, range_w): &mut (Range, Range, Range, Range)) {
    let mut next = curr.clone();
    for x in range_x.clone() {
        for y in range_y.clone() {
            for z in range_z.clone() {
                for w in range_w.clone() {
                    let p = (x, y, z, w);
                    match (curr.contains(&p), unit4d_actived(&curr, p)) {
                        (_, 3) | (true, 4) => next.insert(p),
                        _ => next.remove(&p),
                    };
                }
            }
        }
    }
    range_x.start -= 1;
    range_x.end += 1;
    range_y.start -= 1;
    range_y.end += 1;
    range_z.start -= 1;
    range_z.end += 1;
    range_w.start -= 1;
    range_w.end += 1;
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

        let range_x = -1..(input[0].len() as i32 + 1);
        let range_y = -1..(input.len() as i32 + 1);
        let range_z = -1..2;
        let mut range = (range_x, range_y, range_z);
        for _ in 0..6 {
            part1_proceed(&mut matrix, &mut range);
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

        let range_x = -1..(input[0].len() as i32 + 1);
        let range_y = -1..(input.len() as i32 + 1);
        let range_z = -1..2;
        let range_w = -1..2;
        let mut range = (range_x, range_y, range_z, range_w);
        for _ in 0..6 {
            part2_proceed(&mut matrix, &mut range);
        }

        let result = matrix.len();
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(202, 2028);
