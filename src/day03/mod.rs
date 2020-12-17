use crate::Solution;

fn tree_count(map: &[Vec<char>], (dx, dy): (usize, usize)) -> usize {
    map.iter()
        .step_by(dy)
        .fold((0, 0), |(count, pos), row| {
            (count + (row[pos] == '#') as usize, (pos + dx) % row.len())
        })
        .0
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

const P1_SLOPE: (usize, usize) = (3, 1);
const P2_SLOPES: &[(usize, usize)] = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let result = tree_count(&parse_input(input), P1_SLOPE);
        Ok(result.to_string())
    },
    part2: |input| {
        let input = parse_input(input);
        let result = P2_SLOPES.iter().fold(1, |acc, &dir| acc * tree_count(&input, dir));
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(237, 2106818610);
