use std::collections::HashMap;

struct Tile {
    size:  usize,
    cells: Vec<char>,
}

impl Tile {
    fn row(&self, i: usize) -> impl DoubleEndedIterator<Item = &char> {
        self.cells.iter().skip(i * self.size).take(self.size)
    }
    fn col(&self, i: usize) -> impl DoubleEndedIterator<Item = &char> {
        self.cells.iter().skip(i).step_by(self.size)
    }
}

fn parse_tile<'a>(lines: impl Iterator<Item = &'a str>) -> Tile {
    let mut size = 0;
    let cells: Vec<_> = lines.inspect(|_| size += 1).flat_map(|l| l.chars()).collect();
    assert_eq!(size * size, cells.len());
    Tile { size, cells }
}

fn part1(input: &str) -> usize {
    let mut borders = HashMap::new();
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut it = part.lines();
            let id: usize = it
                .next()
                .unwrap()
                .trim_matches(|c: char| !c.is_digit(10))
                .parse()
                .unwrap();
            (id, parse_tile(it))
        })
        .for_each(|(id, tile)| {
            [
                tile.row(0).collect(),
                tile.col(0).collect(),
                tile.row(tile.size - 1).collect(),
                tile.col(tile.size - 1).collect(),
                tile.row(0).rev().collect(),
                tile.col(0).rev().collect(),
                tile.row(tile.size - 1).rev().collect(),
                tile.col(tile.size - 1).rev().collect(),
            ]
            .iter()
            .for_each(|border: &String| {
                borders
                    .entry(border.clone())
                    .and_modify(|prev| {
                        if *prev != Some(id) {
                            *prev = None
                        }
                    })
                    .or_insert(Some(id));
            });
        });
    let mut counts = HashMap::new();
    borders.values().flatten().for_each(|id| {
        *counts.entry(id).or_insert(0) += 1;
    });
    counts
        .iter()
        .filter(|&(_, &count)| count == 4)
        .map(|(&id, _)| id)
        .product()
}

fn part2(_input: &str) -> usize {
    unimplemented!()
}

crate::solution!(part1 => 8272903687921, part2 => 0);
