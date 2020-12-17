use crate::Solution;
use std::collections::HashMap;

fn valid1(passport: &HashMap<&str, &str>) -> bool {
    match passport.len() {
        7 => !passport.contains_key("cid"),
        8 => true,
        _ => false,
    }
}

fn valid2(passport: &HashMap<&str, &str>) -> bool {
    valid1(passport)
        && passport.iter().fold(true, |acc, (&k, v)| {
            acc && match k {
                "byr" => (1920..=2002).contains(&v.parse().unwrap()),
                "iyr" => (2010..=2020).contains(&v.parse().unwrap()),
                "eyr" => (2020..=2030).contains(&v.parse().unwrap()),
                "hgt" => {
                    let value = v.trim_end_matches(|c| !('0'..='9').contains(&c));
                    let unit = v.trim_start_matches(|c| ('0'..='9').contains(&c));
                    match (value.parse(), unit) {
                        (Ok(value), "cm") => (150..=193).contains(&value),
                        (Ok(value), "in") => (59..=76).contains(&value),
                        _ => false,
                    }
                }
                "hcl" => v.len() == 7 && v.starts_with("#") && v[1..].chars().all(|c| c.is_digit(16)),
                "ecl" => EYE_COLORS.contains(v),
                "pid" => v.len() == 9 && v.chars().all(|c| c.is_digit(10)),
                _ => true,
            }
        })
}

fn solve(passports: &[HashMap<&str, &str>], valid: fn(&HashMap<&str, &str>) -> bool) -> usize {
    passports.iter().fold(0, |acc, passport| acc + valid(passport) as usize)
}

fn parse_input(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|part| {
            part.split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|item| {
                    let mut split = item.split(":");
                    (split.next().unwrap(), split.next().unwrap())
                })
                .collect()
        })
        .collect()
}

const EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let input = parse_input(input);
        let result = solve(&input, valid1);
        Ok(result.to_string())
    },
    part2: |input| {
        let input = parse_input(input);
        let result = solve(&input, valid2);
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
        assert_eq!(res, "208");
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        let res = (SOLUTION.part2)(&input).unwrap();
        assert_eq!(res, "167");
    }
}
