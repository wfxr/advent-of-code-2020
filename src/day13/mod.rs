use crate::Solution;

fn parse_input(input: &str) -> (usize, Vec<&str>) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse().unwrap(),
        lines.next().unwrap().split(',').collect(),
    )
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let (t, schedule) = parse_input(input);
        let (wait, bus): (usize, usize) = schedule
            .iter()
            .filter_map(|s| s.parse().ok())
            .map(|bus| (bus - t % bus, bus))
            .min()
            .unwrap();
        Ok((wait * bus).to_string())
    },
    part2: |input| {
        let (_, schedule) = parse_input(input);
        let (result, _) = schedule
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
            });
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(2305, 552612234243498);
