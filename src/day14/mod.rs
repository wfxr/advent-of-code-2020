use crate::{solution, Result};
use std::collections::HashMap;

fn parse_mask(mask: &str, target: char) -> u64 {
    mask.chars()
        .map(|c| if c == target { 1 } else { 0 })
        .fold(0, |x, b| (x << 1) + b)
}

fn update_memory(mask: u64, addr: u64, val: u64, memory: &mut HashMap<u64, u64>) {
    if mask == 0 {
        memory.insert(addr, val);
    } else {
        let x = mask & (!mask + 1); // get right-most '1' of mask
        let mask = mask & !x; // clear right-most '1' of mask
        update_memory(mask, addr & !x, val, memory);
        update_memory(mask, addr | x, val, memory);
    }
}

fn parse_input(input: &str) -> Result<Vec<(&str, Vec<(u64, u64)>)>> {
    input
        .split("mask = ")
        .map(|s| {
            let mut it = s.split('\n');
            let mask = it.next().ok_or("missing mask")?.trim();
            let lines: Vec<(u64, u64)> = it
                .filter(|s| !s.is_empty())
                .map(|line| {
                    let mut it = line.split(|c: char| !c.is_digit(10)).filter(|s| !s.is_empty());
                    Ok((
                        it.next().ok_or("missing mem")?.parse()?,
                        it.next().ok_or("missing val")?.parse()?,
                    ))
                })
                .collect::<Result<_>>()?;
            Ok((mask, lines))
        })
        .collect()
}

fn part1(input: &str) -> Result<u64> {
    Ok(parse_input(input)?
        .iter()
        .fold(HashMap::new(), |mut acc, (mask, lines)| {
            let (mask0, mask1) = (!parse_mask(mask, '0'), parse_mask(mask, '1'));
            acc.extend(lines.iter().map(|(mem, val)| (mem, val & mask0 | mask1)));
            acc
        })
        .values()
        .sum())
}

fn part2(input: &str) -> Result<u64> {
    Ok(parse_input(input)?
        .iter()
        .fold(HashMap::new(), |mut acc, (mask, lines)| {
            let (mask1, maskx) = (parse_mask(mask, '1'), parse_mask(mask, 'X'));
            lines
                .iter()
                .for_each(|&(mem, val)| update_memory(maskx, mem | mask1, val, &mut acc));
            acc
        })
        .values()
        .sum())
}

solution!(part1 => 5902420735773, part2 => 3801988250775);
