use crate::Solution;
use std::ops::RangeInclusive;

struct Rules {
    rules: Vec<(String, RangeInclusive<usize>, RangeInclusive<usize>)>,
}

impl Rules {
    fn contains(&self, x: &usize) -> bool {
        self.rules.iter().any(|(_, r1, r2)| r1.contains(x) || r2.contains(x))
    }
}

#[rustfmt::skip]
fn parse_input(input: &str) -> (Rules, Vec<usize>, Vec<Vec<usize>>) {
    let mut it = input.split("\n\n");
    let rules: Vec<_> = it .next() .unwrap() .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let field = line.chars().take_while(|&c| c != ':').collect();
            let ranges: Vec<_> = line
                .split(|c: char| !c.is_digit(10))
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            match &ranges[..] {
                &[n1, n2, n3, n4] => (field, n1..=n2, n3..=n4),
                _ => panic!("parse failed: {}", line),
            }
        })
        .collect();
    let mut tickets = it.next().unwrap().lines().skip(1)
        .chain(it.next().unwrap().lines().skip(1))
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect());

    (Rules { rules }, tickets.next().unwrap(), tickets.collect())
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let (rules, _, nearby_tickets) = parse_input(input);
        let result = nearby_tickets
            .iter()
            .flatten()
            .fold(0, |acc, &v| match rules.contains(&v) {
                true => acc,
                false => acc + v,
            });
        Ok(result.to_string())
    },
    part2: |input| unimplemented!(),
};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        let input = include_str!("input");
        let res = (SOLUTION.part1)(&input).unwrap();
        assert_eq!(res, "32842");
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        let res = (SOLUTION.part2)(&input).unwrap();
        assert_eq!(res, "");
    }
}
