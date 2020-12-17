use crate::Solution;

fn parse_input(input: &str) -> Vec<(usize, usize, char, &str)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(&['-', ' ', ':'][..]).filter(|&s| !s.is_empty());
            let a: usize = parts.next().unwrap().parse().unwrap();
            let b: usize = parts.next().unwrap().parse().unwrap();
            let t: char = parts.next().unwrap().parse().unwrap();
            let s: &str = parts.next().unwrap();
            (a, b, t, s)
        })
        .collect()
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let result = parse_input(input).into_iter().fold(0, |acc, (a, b, t, s)| {
            acc + (a..=b).contains(&s.chars().filter(|&c| c == t).count()) as usize
        });
        Ok(result.to_string())
    },
    part2: |input| {
        let result = parse_input(input).into_iter().fold(0, |acc, (a, b, t, s)| {
            acc + ((s.chars().nth(a - 1).unwrap_or('-') == t) != (s.chars().nth(b - 1).unwrap_or('-') == t)) as usize
        });
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(538, 489);
