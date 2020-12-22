use crate::{solution, Result};
use std::ops::RangeInclusive;

type Rule = (String, RangeInclusive<usize>, RangeInclusive<usize>);

fn parse_input(input: &str) -> Result<(Vec<Rule>, Vec<usize>, Vec<Vec<usize>>)> {
    let mut it = input.split("\n\n");
    let rules: Vec<_> = it
        .next()
        .ok_or("missing rule part")?
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let field = line.chars().take_while(|&c| c != ':').collect();
            let ranges: Vec<_> = line
                .split(|c: char| !c.is_digit(10))
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            match ranges[..] {
                [n1, n2, n3, n4] => Ok((field, n1..=n2, n3..=n4)),
                _ => Err(format!("error parsing rule: {}", line).into()),
            }
        })
        .collect::<Result<_>>()?;

    let parse_ticket = |line: &str| {
        line.split(',')
            .map(|s| s.parse::<usize>().map_err(|e| e.into()))
            .collect()
    };

    let your_ticket = parse_ticket(
        it.next()
            .ok_or("missing your ticket part")?
            .lines()
            .nth(1)
            .ok_or("missing your ticket")?,
    )?;

    let other_tickets = it
        .next()
        .ok_or("missing other tickets part")?
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect::<Result<_>>()?;
    Ok((rules, your_ticket, other_tickets))
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
        for (j, field) in fields.iter_mut().enumerate() {
            if *field {
                continue;
            }
            let mut it = (0..n).map(|i| (i, matrix[i][j])).filter(|&(_, x)| x);
            if let (Some((i, _)), None) = (it.next(), it.next()) {
                matrix[i].iter_mut().for_each(|x| *x = false);
                matrix[i][j] = true;
                mapping[i] = j;
                *field = true;
                matched += 1;
            }
        }
    }
    mapping
}

fn part1(input: &str) -> Result<usize> {
    let (rules, _, nearby_tickets) = parse_input(input)?;
    Ok(nearby_tickets
        .iter()
        .flatten()
        .fold(0, |acc, &v| acc + !any_matching(&rules, &v) as usize * v))
}

fn part2(input: &str) -> Result<usize> {
    let (rules, ticket, nearby_tickets) = parse_input(input)?;
    let valid_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|v| any_matching(&rules, v)))
        .collect();
    let mut matrix: Vec<Vec<_>> = rules
        .iter()
        .map(|rule| {
            (0..rules.len())
                .map(|i| valid_tickets.iter().all(|ticket| matching(rule, &ticket[i])))
                .collect()
        })
        .collect();
    let mapping = solve_mapping(&mut matrix);
    Ok(rules
        .iter()
        .enumerate()
        .filter(|(_, (field, ..))| field.starts_with("departure"))
        .fold(1, |acc, (i, ..)| acc * ticket[mapping[i]]))
}

solution!(part1 => 32842, part2 => 2628667251989);
