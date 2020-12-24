use crate::{solution, Result};

fn parse_input(input: &str) -> Result<(usize, Vec<Option<usize>>)> {
    let mut lines = input.lines();
    Ok((
        lines.next().ok_or("missing start time")?.parse()?,
        lines
            .next()
            .ok_or("missing bus schedule")?
            .split(',')
            .map(|s| s.parse().ok())
            .collect(),
    ))
}

fn part1(input: &str) -> Result<usize> {
    let (t, schedule) = parse_input(input)?;
    let (wait, bus): (usize, usize) = schedule
        .into_iter()
        .filter_map(|s| s.map(|bus| (bus - t % bus, bus)))
        .min()
        .ok_or("empty")?;
    Ok(wait * bus)
}

fn part2(input: &str) -> Result<usize> {
    let (_, schedule) = parse_input(input)?;
    Ok(schedule
        .into_iter()
        .enumerate()
        .fold((1, 1), |(mut t, mut step), (delay, s)| {
            if let Some(bus) = s {
                while (t + delay) % bus != 0 {
                    t += step;
                }
                step *= bus;
            }
            (t, step)
        })
        .0)
}

solution!(part1 => 2305, part2 => 552612234243498);
