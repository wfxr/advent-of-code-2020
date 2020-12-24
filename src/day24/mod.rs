use crate::{err, solution, Result};
use std::collections::HashMap;
use std::collections::HashSet;

type Tile = (i32, i32);
const NEIGHBORS: &[Tile] = &[(1, 0), (-1, 0), (0, 1), (0, -1), (1, -1), (-1, 1)]; // (E, NE)

fn parse_input(input: &str) -> Result<Vec<Tile>> {
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

fn black_tiles(input: &str) -> Result<HashSet<Tile>> {
    Ok(parse_input(input)?.into_iter().fold(HashSet::new(), |mut acc, tile| {
        if acc.take(&tile).is_none() {
            acc.insert(tile);
        }
        acc
    }))
}

fn part1(input: &str) -> Result<usize> {
    Ok(black_tiles(input)?.len())
}

fn part2(input: &str) -> Result<usize> {
    fn count_neighbors(blacks: &HashSet<Tile>) -> HashMap<Tile, u8> {
        blacks.iter().fold(HashMap::new(), |mut acc, (x, y)| {
            NEIGHBORS.iter().for_each(|(dx, dy)| {
                *acc.entry((x + dx, y + dy)).or_insert(0) += 1;
            });
            acc
        })
    }
    Ok((0..100)
        .fold(black_tiles(input)?, |blacks, _| {
            count_neighbors(&blacks)
                .iter()
                .filter_map(|(k, v)| match (blacks.contains(k), v) {
                    (true, n) if (1..3).contains(n) => Some(*k),
                    (false, 2) => Some(*k),
                    _ => None,
                })
                .collect()
        })
        .len())
}

solution!(part1 => 289, part2 => 3551);

#[cfg(test)]
mod examples {
    use crate::{input, test};
    const SAMPLE: &str = input!(
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
    );
    test!(part1, example1: SAMPLE => 10);
    test!(part2, example1: SAMPLE => 2208);
}
