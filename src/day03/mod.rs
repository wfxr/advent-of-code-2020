fn tree_count(map: &[Vec<char>], (dx, dy): (usize, usize)) -> usize {
    map.iter()
        .step_by(dy)
        .fold((0, 0), |(count, pos), row| {
            (count + (row[pos] == '#') as usize, (pos + dx) % row.len())
        })
        .0
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

fn part1(input: &str) -> usize {
    tree_count(&parse_input(input), (3, 1))
}

fn part2(input: &str) -> usize {
    let input = parse_input(input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, &dir| acc * tree_count(&input, dir))
}

crate::solution!(part1 => 237, part2 => 2106818610);
