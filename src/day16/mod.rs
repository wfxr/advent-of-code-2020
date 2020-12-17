use crate::Solution;
use std::ops::RangeInclusive;

type Rule = (String, RangeInclusive<usize>, RangeInclusive<usize>);

#[rustfmt::skip]
fn parse_input(input: &str) -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
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

    (rules, tickets.next().unwrap(), tickets.collect())
}

fn any_matching(rules: &[Rule], v: &usize) -> bool {
    rules.iter().any(|rule| matching(rule, v))
}

fn matching((_, r1, r2): &Rule, v: &usize) -> bool {
    r1.contains(v) || r2.contains(v)
}

fn solve_mapping(matrix: &mut Vec<Vec<bool>>) -> Vec<usize> {
    let n = matrix.len();
    let mut mapping = vec![0; n];
    let (mut fields, mut matched) = (vec![false; n], 0);
    while matched < n {
        for j in 0..n {
            if fields[j] {
                continue;
            }
            let mut it = (0..n).map(|i| (i, matrix[i][j])).filter(|&(_, x)| x);
            match (it.next(), it.next()) {
                (Some((i, _)), None) => {
                    matrix[i].iter_mut().for_each(|x| *x = false);
                    matrix[i][j] = true;
                    mapping[i] = j;
                    fields[j] = true;
                    matched += 1;
                }
                _ => {}
            };
        }
    }
    mapping
}

#[rustfmt::skip]
pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let (rules, _, nearby_tickets) = parse_input(input);
        let result = nearby_tickets.iter().flatten()
            .fold(0, |acc, &v| acc + !any_matching(&rules, &v) as usize * v);
        Ok(result.to_string())
    },
    part2: |input| {
        let (rules, ticket, nearby_tickets) = parse_input(input);
        let valid_tickets: Vec<_> = nearby_tickets.iter()
            .filter(|ticket| ticket.iter().all(|v| any_matching(&rules, v)))
            .collect();
        let mut matrix: Vec<Vec<_>> = rules.iter()
            .map(|rule| {
                (0..rules.len())
                    .map(|i| valid_tickets.iter().all(|ticket| matching(rule, &ticket[i])))
                    .collect()
            })
            .collect();
        let mapping = solve_mapping(&mut matrix);
        let result = rules
            .iter()
            .enumerate()
            .filter(|(_, (field, ..))| field.starts_with("departure"))
            .fold(1, |acc, (i, ..)| acc * ticket[mapping[i]]);
        Ok(result.to_string())
    },
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
        assert_eq!(res, "2628667251989");
    }
}
