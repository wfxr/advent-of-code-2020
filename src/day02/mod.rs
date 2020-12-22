use crate::{solution, Result};

fn parse_input(input: &str) -> Result<Vec<(usize, usize, char, &str)>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(&['-', ' ', ':'][..]).filter(|&s| !s.is_empty());
            let a: usize = parts.next().ok_or("missing a")?.parse()?;
            let b: usize = parts.next().ok_or("missing b")?.parse()?;
            let t: char = parts.next().ok_or("missing t")?.parse()?;
            let s: &str = parts.next().ok_or("missing s")?;
            Ok((a, b, t, s))
        })
        .collect()
}

fn part1(input: &str) -> Result<usize> {
    Ok(parse_input(input)?.into_iter().fold(0, |acc, (a, b, t, s)| {
        acc + (a..=b).contains(&s.chars().filter(|&c| c == t).count()) as usize
    }))
}

fn part2(input: &str) -> Result<usize> {
    Ok(parse_input(input)?.into_iter().fold(0, |acc, (a, b, t, s)| {
        acc + ((s.chars().nth(a - 1).unwrap_or('-') == t) != (s.chars().nth(b - 1).unwrap_or('-') == t)) as usize
    }))
}

solution!(part1 => 538, part2 => 489);
