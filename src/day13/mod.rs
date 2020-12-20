fn parse_input(input: &str) -> (usize, Vec<&str>) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse().unwrap(),
        lines.next().unwrap().split(',').collect(),
    )
}

fn part1(input: &str) -> usize {
    let (t, schedule) = parse_input(input);
    let (wait, bus): (usize, usize) = schedule
        .iter()
        .filter_map(|s| s.parse().ok())
        .map(|bus| (bus - t % bus, bus))
        .min()
        .unwrap();
    wait * bus
}

fn part2(input: &str) -> usize {
    let (_, schedule) = parse_input(input);
    schedule
        .iter()
        .enumerate()
        .fold((1, 1), |(mut t, mut step), (delay, &s)| {
            if let Ok(bus) = s.parse::<usize>() {
                while (t + delay) % bus != 0 {
                    t += step;
                }
                step *= bus;
            }
            (t, step)
        })
        .0
}

crate::solution!(part1 => 2305, part2 => 552612234243498);
