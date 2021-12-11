use crate::{err, solution, Result};
use std::collections::HashMap;

const N: usize = 10;

#[derive(Debug)]
struct Tile {
    cells: [u8; N * N],
    rot:   u8,
    flip:  u8,
}

#[rustfmt::skip]
impl Tile {
    fn row(&self, i: usize) -> Vec<u8> { (0..N).map(|j| self.get(i, j)).collect() }
    fn col(&self, j: usize) -> Vec<u8> { (0..N).map(|i| self.get(i, j)).collect() }

    fn top(&self) -> Vec<u8> { self.row(0) }
    fn bot(&self) -> Vec<u8> { self.row(N - 1) }
    fn lhs(&self) -> Vec<u8> { self.col(0) }
    fn rhs(&self) -> Vec<u8> { self.col(N - 1) }

    fn get(&self, i: usize, j: usize) -> u8 {
        let max = N - 1;
        let (i, j) = match self.rot % 4 {
            0 => (i, j),
            1 => (j, max - i),
            2 => (max - i, max - j),
            _ => (max - j, i),
        };
        let (i, j) = match self.flip % 2 {
            0 => (i, j),
            _ => (i, max - j),
        };
        self.cells[i * N + j]
    }

    fn possible_borders(&self) -> Vec<Vec<u8>> {
        vec![
            self.top(), self.top().into_iter().rev().collect(),
            self.bot(), self.bot().into_iter().rev().collect(),
            self.lhs(), self.lhs().into_iter().rev().collect(),
            self.rhs(), self.rhs().into_iter().rev().collect(),
        ]
    }

    fn trans(&mut self, flip: u8, rot: u8) {
        self.flip = flip % 2;
        self.rot = rot % 4;
    }
}

const TRANSFORMS: &[(u8, u8)] = &[(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (1, 3)];

fn parse_input(input: &str) -> Result<HashMap<usize, Tile>> {
    input
        .split("\n\n")
        .map(|part| match part.split_once('\n') {
            Some((title, cells)) => {
                let id = title.trim_matches(|c: char| !c.is_digit(10)).parse()?;
                let cells = cells
                    .lines()
                    .flat_map(|line| line.bytes())
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|_| format!("invalid tile: {cells}"))?;
                Ok((id, Tile { cells, rot: 0, flip: 0 }))
            }
            None => err!("missing tile id"),
        })
        .collect()
}

fn get_borders(tiles: &HashMap<usize, Tile>) -> HashMap<Vec<u8>, Vec<usize>> {
    tiles.iter().fold(HashMap::new(), |mut borders, (&id, tile)| {
        tile.possible_borders().into_iter().for_each(|border| {
            borders.entry(border).or_insert_with(Vec::new).push(id);
        });
        borders
    })
}

fn get_corners(borders: &HashMap<Vec<u8>, Vec<usize>>) -> Vec<usize> {
    borders
        .values()
        .filter(|ids| ids.len() == 1)
        .flatten()
        .fold(HashMap::new(), |mut acc, &id| {
            *acc.entry(id).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter_map(|(&id, &count)| (count == 4).then_some(id))
        .collect()
}

fn part1(input: &str) -> Result<usize> {
    Ok(get_corners(&get_borders(&parse_input(input)?)).iter().product())
}

const MONSTER: &[&[u8]] = &[
    b"                  # ",
    b"#    ##    ##    ###",
    b" #  #  #  #  #  #   ",
];

// FIXME: still in a messy
#[allow(clippy::needless_range_loop)]
fn part2(input: &str) -> Result<usize> {
    let mut tiles = parse_input(input)?;
    let borders = get_borders(&tiles);
    let corners = get_corners(&borders);

    // find the top-left corner
    let mut curr_id = *corners.get(0).ok_or("corner not found")?;
    let curr_tile = tiles
        .get_mut(&curr_id)
        .ok_or_else(|| format!("corner tile({curr_id}) not found"))?;
    TRANSFORMS
        .iter()
        .find(|&&(flip, rot)| {
            curr_tile.trans(flip, rot);
            borders[&curr_tile.top()].len() == 1 && borders[&curr_tile.lhs()].len() == 1
        })
        .ok_or("can't transform to right position")?;

    let n: usize = (tiles.len() as f64).sqrt() as usize; // image_size
    let mut grids = vec![vec![0; n]; n];

    // left-top corner
    grids[0][0] = curr_id;

    // left border
    for i in 1..n {
        let up_id = curr_id;
        let up_tile = &tiles[&up_id];
        let up_tile_bot = up_tile.bot();
        curr_id = *borders[&up_tile_bot]
            .iter()
            .find(|&&id| id != up_id)
            .ok_or_else(|| format!("bottom tile id({}) not found", up_id))?;
        let curr_tile = tiles
            .get_mut(&curr_id)
            .ok_or_else(|| format!("bottom tile({}) not found", up_id))?;

        TRANSFORMS
            .iter()
            .find(|&&(flip, rot)| {
                curr_tile.trans(flip, rot);
                curr_tile.top() == up_tile_bot
            })
            .ok_or("no matched tile found")?;
        grids[i][0] = curr_id;
    }

    // remaining cells
    for i in 0..n {
        curr_id = grids[i][0];
        for j in 1..n {
            let left_tile = &tiles[&curr_id];
            let left_tile_rhs = left_tile.rhs();
            curr_id = *borders[&left_tile_rhs]
                .iter()
                .find(|&&id| id != curr_id)
                .ok_or_else(|| format!("right tile id({}) not found", curr_id))?;
            let curr_tile = tiles
                .get_mut(&curr_id)
                .ok_or_else(|| format!("right tile({}) not found", curr_id))?;
            TRANSFORMS
                .iter()
                .find(|&&(flip, rot)| {
                    curr_tile.trans(flip, rot);
                    curr_tile.lhs() == left_tile_rhs
                })
                .ok_or("no matched tile found")?;
            grids[i][j] = curr_id;
        }
    }

    let m = N - 2;
    let mut image = vec![vec![0u8; m * n]; m * n];
    (0..n)
        .flat_map(|i| (0..n).map(move |j| (i, j)))
        .flat_map(|(i, j)| (0..m).flat_map(move |ii| (0..m).map(move |jj| (i, j, ii, jj))))
        .for_each(|(i, j, ii, jj)| {
            image[i * m + ii][j * m + jj] = tiles[&grids[i][j]].get(ii + 1, jj + 1) as u8;
        });
    count_monsters(&image)
}

#[allow(clippy::needless_range_loop)]
fn rot(image: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let n = image.len();
    let mut new = vec![vec![0u8; n]; n];
    for i in 0..n {
        for j in 0..n {
            new[i][j] = image[j][n - 1 - i];
        }
    }
    new
}
fn flip(image: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let n = image.len();
    let mut new = vec![vec![0u8; n]; n];
    for i in 0..n {
        for j in 0..n {
            new[i][j] = image[i][n - 1 - j];
        }
    }
    new
}

fn count_monsters(image: &[Vec<u8>]) -> Result<usize> {
    let n = image.len();
    let h = MONSTER.len();
    let w = MONSTER[0].len();

    let check_monster = |image: &[Vec<u8>], i: usize, j: usize| {
        let mut found = true;
        for ii in 0..h {
            for jj in 0..w {
                if MONSTER[ii][jj] == b'#' && image[i + ii][j + jj] != b'#' {
                    found = false;
                }
            }
        }
        found
    };

    let has_monsters = |image: &[Vec<u8>]| {
        for i in 0..n - h {
            for j in 0..n - w {
                if check_monster(image, i, j) {
                    return true;
                }
            }
        }
        false
    };

    let mark_monster = |image: &mut [Vec<u8>], i: usize, j: usize| {
        for ii in 0..h {
            for jj in 0..w {
                if MONSTER[ii][jj] == b'#' {
                    image[i + ii][j + jj] = b'O'
                }
            }
        }
    };

    let mark_monsters = |image: &mut [Vec<u8>]| {
        for i in 0..n - h {
            for j in 0..n - w {
                if check_monster(image, i, j) {
                    mark_monster(image, i, j);
                }
            }
        }
    };

    let mut image = image.to_vec();
    let mut found = false;
    if !found {
        for _ in 0..4 {
            image = rot(&image);
            if has_monsters(&image) {
                found = true;
                break;
            }
        }
    }
    if !found {
        image = flip(&image);
        for _ in 0..4 {
            image = rot(&image);
            if has_monsters(&image) {
                found = true;
                break;
            }
        }
    }
    if !found {
        return err!("monsters not found");
    }

    mark_monsters(&mut image);

    Ok(image.iter().flatten().filter(|&&c| c == b'#').count())
}

solution!(part1 => 8272903687921, part2 => 2304);
