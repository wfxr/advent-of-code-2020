use std::io::{self, BufRead};

#[rustfmt::skip]
fn p1_solve(schedule: &[&str], t: usize) -> (usize, usize) {
    schedule.iter()
        .filter_map(|s| s.parse().ok())
        .map(|bus| (bus - t % bus, bus))
        .min().unwrap()
}

#[rustfmt::skip]
fn p2_solve(schedule: &[&str]) -> usize {
    schedule.iter().enumerate()
        .fold((1, 1), |(mut t, mut step), (delay, &s)| {
            if let Ok(bus) = s.parse::<usize>() {
                while (t + delay) % bus != 0 {
                    t += step;
                }
                step *= bus;
            }
            (t, step)
        }).0
}

fn main() {
    let reader = io::stdin();
    let mut lines = reader.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let schedule = lines.next().unwrap().unwrap();
    let schedule: Vec<_> = schedule.split(',').collect();

    let (wait, bus) = p1_solve(&schedule, t);
    println!("part 1 result: {}", wait * bus);
    assert_eq!(2305, wait * bus);

    let result = p2_solve(&schedule);
    println!("part 2 result: {}", result);
    assert_eq!(552612234243498, result);
}
