use crate::{err, solution, Result};

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn turn(&self, degrees: i64) -> Result<Self> {
        match degrees {
            90 => Ok(Point::new(-self.y, self.x)),
            180 => Ok(Point::new(-self.x, -self.y)),
            270 => Ok(Point::new(self.y, -self.x)),
            _ => err!("invalid degrees: {}", degrees),
        }
    }

    fn mul(&self, rhs: i64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }

    fn add(&self, rhs: &Point) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

fn parse_input(input: &str) -> Result<Vec<(char, i64)>> {
    input
        .lines()
        .map(|line| {
            Ok((
                line.chars().next().ok_or("empty line")?,
                line.get(1..).ok_or("missing number")?.parse()?,
            ))
        })
        .collect()
}

fn part1(input: &str) -> Result<usize> {
    let input = parse_input(input)?;
    let mut ship = Point::new(0, 0);
    let mut direction = Point::new(1, 0);
    for &(ins, v) in &input {
        match ins {
            'N' => ship.y += v,
            'S' => ship.y -= v,
            'E' => ship.x += v,
            'W' => ship.x -= v,
            'L' => direction = direction.turn(v)?,
            'R' => direction = direction.turn(360 - v)?,
            'F' => ship = ship.add(&direction.mul(v)),
            _ => return err!("instruction: {}", ins),
        }
    }
    Ok(ship.manhattan())
}

fn part2(input: &str) -> Result<usize> {
    let input = parse_input(input)?;
    let mut waypoint = Point::new(10, 1);
    let mut ship = Point::new(0, 0);
    for &(ins, v) in &input {
        match ins {
            'N' => waypoint.y += v,
            'S' => waypoint.y -= v,
            'E' => waypoint.x += v,
            'W' => waypoint.x -= v,
            'L' => waypoint = waypoint.turn(v)?,
            'R' => waypoint = waypoint.turn(360 - v)?,
            'F' => ship = ship.add(&waypoint.mul(v)),
            _ => return err!("instruction: {}", ins),
        }
    }
    Ok(ship.manhattan())
}

solution!(part1 => 1956, part2 => 126797);
