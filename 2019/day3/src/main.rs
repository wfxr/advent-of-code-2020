#![feature(iter_map_while)]
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn offset(&self, dx: i64, dy: i64) -> Self {
        Point::new(self.x + dx, self.y + dy)
    }
}

#[derive(Debug)]
#[rustfmt::skip]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }

    fn length(&self) -> i64 {
        distance(&self.p1, &self.p2)
    }

    fn contains(&self, p: &Point) -> bool {
        (self.p1.x..=self.p2.x).contains(&p.x) && (self.p1.y..=self.p2.y).contains(&p.y)
    }

    fn direction(&self) -> Direction {
        match (self.p1.x - self.p2.x, self.p1.y - self.p2.y) {
            (0, _) => Direction::Y,
            (_, 0) => Direction::X,
            _ => unreachable!(),
        }
    }
}

enum Direction {
    X,
    Y,
}

fn distance(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn intersection(l1: &Line, l2: &Line) -> Option<Point> {
    let (l1, l2) = match (l1.direction(), l2.direction()) {
        (Direction::X, Direction::Y) => (l1, l2),
        (Direction::Y, Direction::X) => (l2, l1),
        _ => return None,
    };
    if (l1.p1.x..=l1.p2.x).contains(&l2.p1.x) && (l2.p1.y..=l2.p2.y).contains(&l1.p1.y) {
        Some(Point::new(l2.p1.x, l1.p1.y))
    } else {
        None
    }
}

fn paths_to_lines(paths: &[(char, i64)]) -> Vec<Line> {
    let mut start = Point::new(0, 0);
    paths
        .iter()
        .map(|&(direction, s)| {
            let (dx, dy) = match direction {
                'L' => (-s, 0),
                'R' => (s, 0),
                'U' => (0, s),
                'D' => (0, -s),
                _ => unreachable!(),
            };
            let end = start.offset(dx, dy);
            let line = Line::new(start, end);
            start = end;
            line
        })
        .collect()
}

#[allow(non_snake_case)]
fn p1_solve(L1: &[(char, i64)], L2: &[(char, i64)]) -> i64 {
    let (L1, L2) = &(paths_to_lines(L1), paths_to_lines(L2));
    L1.iter().fold(i64::MAX, |acc, l1| {
        L2.iter().fold(acc, |acc, l2| {
            acc.min(intersection(l1, l2).map(|p| p.x.abs() + p.y.abs()).unwrap_or(acc))
        })
    })
}

#[allow(non_snake_case)]
fn p2_solve(L1: &[(char, i64)], L2: &[(char, i64)]) -> i64 {
    let (L1, L2) = &(paths_to_lines(L1), paths_to_lines(L2));
    L1.iter()
        .flat_map(|l1| L2.iter().filter_map(move |l2| intersection(l1, l2)))
        .fold(i64::MAX, |acc, p| acc.min(steps(L1, p) + steps(L2, p)))
}

#[rustfmt::skip]
#[allow(non_snake_case)]
fn steps(L: &[Line], p: Point) -> i64 {
    let (i, s) = L .iter()
        .take_while(|l| !l.contains(&p))
        .fold((0, 0), |(i, acc), l| (i + 1, acc + l.length()));
    s + Line::new(L[i].p1, p).length()
}

#[rustfmt::skip]
#[allow(non_snake_case)]
fn main() {
    let inputs: Vec<Vec<(char, i64)>> = io::stdin().lock().lines()
        .map(|l| {
            l.unwrap().split(',')
                .map(|l| (l.chars().nth(0).unwrap(), l[1..].to_owned().parse().unwrap()))
                .collect()
        })
        .collect();

    let result = p1_solve(&inputs[0], &inputs[1]);
    println!("part 1 result: {}", result);
    assert_eq!(2180, result);

    let result = p2_solve(&inputs[0], &inputs[1]);
    println!("part 2 result: {}", result);
    assert_eq!(112316, result);
}
