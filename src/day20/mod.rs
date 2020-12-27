use crate::{err, solution, Result};
use std::collections::HashMap;

#[derive(Debug)]
struct Tile {
    size:  usize,
    cells: Vec<char>,
    rot:   u8,
    flip:  u8,
}

#[rustfmt::skip]
impl Tile {
    fn row(&self, i: usize) -> Vec<char> { (0..self.size).map(|j| self.get(i, j)).collect() }
    fn col(&self, j: usize) -> Vec<char> { (0..self.size).map(|i| self.get(i, j)).collect() }

    fn top(&self) -> Vec<char> { self.row(0) }
    fn bot(&self) -> Vec<char> { self.row(self.size - 1) }
    fn lhs(&self) -> Vec<char> { self.col(0) }
    fn rhs(&self) -> Vec<char> { self.col(self.size - 1) }

    fn get(&self, i: usize, j: usize) -> char {
        let max = self.size - 1;
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
        self.cells[i * self.size + j]
    }

    fn possible_borders(&self) -> Vec<Vec<char>> {
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

fn parse_tile<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Tile> {
    let mut size = 0;
    let cells: Vec<_> = lines.inspect(|_| size += 1).flat_map(|l| l.chars()).collect();
    match size * size {
        n if n == cells.len() => Ok(Tile {
            size,
            cells,
            rot: 0,
            flip: 0,
        }),
        _ => err!("tile size inconsistent"),
    }
}

fn parse_input(input: &str) -> Result<HashMap<usize, Tile>> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut it = part.lines();
            let id: usize = it
                .next()
                .ok_or("missing tile id")?
                .trim_matches(|c: char| !c.is_digit(10))
                .parse()?;
            Ok((id, parse_tile(it)?))
        })
        .collect::<Result<_>>()
}

fn get_borders(tiles: &HashMap<usize, Tile>) -> HashMap<Vec<char>, Vec<usize>> {
    tiles.iter().fold(HashMap::new(), |mut borders, (&id, tile)| {
        tile.possible_borders().into_iter().for_each(|border| {
            borders.entry(border).or_insert_with(Vec::new).push(id);
        });
        borders
    })
}

fn get_corners(borders: &HashMap<Vec<char>, Vec<usize>>) -> Vec<usize> {
    borders
        .values()
        .filter(|ids| ids.len() == 1)
        .flatten()
        .fold(HashMap::new(), |mut acc, &id| {
            *acc.entry(id).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter_map(|(&id, &count)| if count == 4 { Some(id) } else { None })
        .collect()
}

fn part1(input: &str) -> Result<usize> {
    Ok(get_corners(&get_borders(&parse_input(input)?)).iter().product())
}

// FIXME: still in a messy
fn part2(input: &str) -> Result<usize> {
    let mut tiles = parse_input(input)?;
    let borders = get_borders(&tiles);
    let corners = get_corners(&borders);

    // find the top-left corner
    let mut curr = *corners.get(0).ok_or("corner not found")?;
    let curr_tile = tiles
        .get_mut(&curr)
        .ok_or_else(|| format!("corner tile({}) not found", curr))?;
    TRANSFORMS
        .iter()
        .find(|&&(flip, rot)| {
            curr_tile.trans(flip, rot);
            borders[&curr_tile.top()].len() == 1 && borders[&curr_tile.lhs()].len() == 1
        })
        .ok_or("can't transform to right position")?;

    let n: usize = (tiles.len() as f64).sqrt() as usize; // image_size
    let mut grids = vec![vec![0; n]; n];
    for i in 0..n {
        grids[i][0] = curr;
        for j in 1..n {
            let left_tile = &tiles[&curr];
            let border = left_tile.rhs();
            curr = *borders[&border]
                .iter()
                .find(|&&id| id != curr)
                .ok_or_else(|| format!("right tile id({}) not found", curr))?;
            let curr_tile = tiles
                .get_mut(&curr)
                .ok_or_else(|| format!("right tile({}) not found", curr))?;
            let mut found = false;
            for &(flip, rot) in TRANSFORMS {
                curr_tile.trans(flip, rot);
                if curr_tile.lhs() == border {
                    found = true;
                    break;
                }
            }
            if !found {
                return err!("no matched tile found");
            }
            grids[i][j] = curr;
        }
        if i == n - 1 {
            break; // last row
        }
        let up_id = grids[i][0];
        let up_tile = &tiles[&up_id];
        let border = up_tile.bot();
        curr = *borders[&border]
            .iter()
            .find(|&&id| id != up_id)
            .ok_or_else(|| format!("bottom tile id({}) not found", up_id))?;
        let curr_tile = tiles
            .get_mut(&curr)
            .ok_or_else(|| format!("bottom tile({}) not found", up_id))?;
        let mut found = false;
        for &(flip, rot) in TRANSFORMS {
            curr_tile.trans(flip, rot);
            if curr_tile.top() == border {
                found = true;
                break;
            }
        }
        if !found {
            return err!("no matched tile found");
        }
    }

    let m = &tiles[&grids[0][0]].size - 2;
    let mut image = vec![vec![0u8; m * n]; m * n];
    for i in 0..n {
        for j in 0..n {
            let tile = &tiles[&grids[i][j]];
            for ii in 0..m {
                for jj in 0..m {
                    image[i * m + ii][j * m + jj] = tile.get(ii + 1, jj + 1) as u8;
                }
            }
        }
    }

    Ok(count_monsters(&image)?)
}

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

const MONSTER: &[&[u8]] = &[
    b"                  # ",
    b"#    ##    ##    ###",
    b" #  #  #  #  #  #   ",
];

solution!(part1 => 8272903687921, part2 => 2304);
