use crate::{err, solution, Result};
use std::collections::HashMap;

fn parse_input(input: &str) -> Result<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|line| {
            let mut it = line.chars();
            let (mut e, mut ne) = (0, 0);
            while let Some(c) = it.next() {
                let (de, dne) = match c {
                    'e' => (1, 0),
                    'w' => (-1, 0),
                    _ => match (c, it.next().ok_or("invalid direction")?) {
                        ('n', 'e') => (0, 1),
                        ('s', 'w') => (0, -1),
                        ('s', 'e') => (1, -1),
                        ('n', 'w') => (-1, 1),
                        (a, b) => return err!("invalid direction: {}-{}", a, b),
                    },
                };
                e += de;
                ne += dne;
            }
            Ok((e, ne))
        })
        .collect::<Result<_>>()
}

fn part1(input: &str) -> Result<usize> {
    let input = parse_input(input)?;
    let mut m = HashMap::new();
    input.iter().for_each(|tile| *m.entry(tile).or_insert(0) += 1);
    Ok(m.values().filter(|&&x| x % 2 == 1).count())
}

fn part2(input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 289, part2 => 0);

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
