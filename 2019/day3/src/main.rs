use std::io::{self, BufRead};

#[derive(Copy, Clone)]
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

struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
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
        .map(|&(direction, distance)| {
            let (dx, dy) = match direction {
                'L' => (-distance, 0),
                'R' => (distance, 0),
                'U' => (0, distance),
                'D' => (0, -distance),
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
    let mut distance = i64::MAX;
    for l1 in L1 {
        for l2 in L2 {
            if let Some(p) = intersection(l1, l2) {
                distance = distance.min(p.x.abs() + p.y.abs())
            }
        }
    }
    distance
}

#[allow(non_snake_case)]
#[rustfmt::skip]
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
}
