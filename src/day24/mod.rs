use crate::{err, solution, Result};
use std::collections::HashMap;

fn parse_input(input: &str) -> Result<Vec<Vec<usize>>> {
    input
        .lines()
        .map(|line| {
            let mut it = line.chars();
            let mut line = vec![];
            while let Some(c) = it.next() {
                let dir = match c {
                    'e' => 0,
                    'w' => 3,
                    _ => match (c, it.next().ok_or("invalid direction")?) {
                        ('n', 'e') => 1,
                        ('s', 'w') => 4,
                        ('s', 'e') => 2,
                        ('n', 'w') => 5,
                        (a, b) => return err!("invalid direction: {}-{}", a, b),
                    },
                };
                line.push(dir);
            }
            Ok(line)
        })
        .collect::<Result<_>>()
}

fn simplify(line: &[usize]) -> (i32, i32) {
    let mut counts = [0i32; 6];
    line.iter().for_each(|&x| counts[x] += 1);
    let e = counts[0] - counts[3];
    let ne = counts[1] - counts[4];
    let se = counts[2] - counts[5];
    let ne = ne - se;
    let e = e + se;
    println!("counts: {:?}, ({}, {})", counts, e, ne);
    (e, ne)
}

fn part1(input: &str) -> Result<usize> {
    let input = parse_input(input)?;

    let mut m = HashMap::new();
    input.iter().for_each(|line| *m.entry(simplify(line)).or_insert(0) += 1);
    Ok(m.values().filter(|&&x| x % 2 == 1).count())
}

fn part2(input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 0, part2 => 0);

#[cfg(test)]
mod examples {
    crate::test!(part1,
        example1: input!(
                  "sesenwnenenewseeswwswswwnenewsewsw",
                  "neeenesenwnwwswnenewnwwsewnenwseswesw",
                  "seswneswswsenwwnwse",
                  "nwnwneseeswswnenewneswwnewseswneseene",
                  "swweswneswnenwsewnwneneseenw",
                  "eesenwseswswnenwswnwnwsewwnwsene",
                  "sewnenenenesenwsewnenwwwse",
                  "wenwwweseeeweswwwnwwe",
                  "wsweesenenewnwwnwsenewsenwwsesesenwne",
                  "neeswseenwwswnwswswnw",
                  "nenwswwsewswnenenewsenwsenwnesesenew",
                  "enewnwewneswsewnwswenweswnenwsenwsw",
                  "sweneswneswneneenwnewenewwneswswnese",
                  "swwesenesewenwneswnwwneseswwne",
                  "enesenwswwswneneswsenwnewswseenwsese",
                  "wnwnesenesenenwwnenwsewesewsesesew",
                  "nenewswnwewswnenesenwnesewesw",
                  "eneswnwswnwsenenwnwnwwseeswneewsenese",
                  "neswnwewnwnwseenwseesewsenwsweewe",
                  "wseweeenwnesenwwwswnew",
                  ) => 10,
    );
}
