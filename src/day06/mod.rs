fn count<F>(answers: &[&str], f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut questions = [0; 26];
    answers.iter().flat_map(|s| s.as_bytes()).for_each(|&c| {
        questions[(c - b'a') as usize] += 1;
    });
    questions.iter().filter(|&&n| f(n)).count()
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|p| p.split('\n').filter(|s| !s.is_empty()).collect())
        .collect()
}

fn part1(input: &str) -> usize {
    parse_input(input).iter().map(|v| count(&v, |n| n > 0)).sum()
}

fn part2(input: &str) -> usize {
    parse_input(input).iter().map(|v| count(&v, |n| n == v.len())).sum()
}

crate::solution!(part1 => 6416, part2 => 3050);
