use crate::{solution, Result};
use std::collections::HashMap;

fn valid1(passport: &HashMap<&str, &str>) -> bool {
    match passport.len() {
        7 => !passport.contains_key("cid"),
        8 => true,
        _ => false,
    }
}

fn valid2(passport: &HashMap<&str, &str>) -> Result<bool> {
    Ok(valid1(passport)
        && passport.iter().try_fold::<_, _, Result<_>>(true, |acc, (&k, v)| {
            Ok(acc
                && match k {
                    "byr" => (1920..=2002u32).contains(&v.parse()?),
                    "iyr" => (2010..=2020u32).contains(&v.parse()?),
                    "eyr" => (2020..=2030u32).contains(&v.parse()?),
                    "hgt" => {
                        let value = v.trim_end_matches(|c| !('0'..='9').contains(&c));
                        let unit = v.trim_start_matches(|c| ('0'..='9').contains(&c));
                        match (value.parse(), unit) {
                            (Ok(value), "cm") => (150..=193).contains(&value),
                            (Ok(value), "in") => (59..=76).contains(&value),
                            _ => false,
                        }
                    }
                    "hcl" => v.len() == 7 && v.starts_with('#') && v[1..].chars().all(|c| c.is_digit(16)),
                    "ecl" => EYE_COLORS.contains(v),
                    "pid" => v.len() == 9 && v.chars().all(|c| c.is_digit(10)),
                    _ => true,
                })
        })?)
}

fn solve(passports: &[HashMap<&str, &str>], valid: fn(&HashMap<&str, &str>) -> Result<bool>) -> Result<usize> {
    passports
        .iter()
        .try_fold(0, |acc, passport| Ok(acc + valid(passport)? as usize))
}

fn parse_input(input: &str) -> Result<Vec<HashMap<&str, &str>>> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|part| {
            part.split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|item| {
                    let mut split = item.split(':');
                    Ok((
                        split.next().ok_or("missing field")?,
                        split.next().ok_or("missing value")?,
                    ))
                })
                .collect()
        })
        .collect()
}

const EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn part1(input: &str) -> Result<usize> {
    solve(&parse_input(input)?, |passport| Ok(valid1(passport)))
}
fn part2(input: &str) -> Result<usize> {
    solve(&parse_input(input)?, valid2)
}

solution!(part1 => 208, part2 => 167);
