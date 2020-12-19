use crate::Solution;

fn solve(input: &str, occupied: fn(&[Vec<char>], (usize, usize)) -> usize) -> usize {
    let seats = parse_input(input);
    let mut curr: Vec<_> = seats.iter().map(|row| row.to_vec()).collect();
    let mut changed = true;
    while changed {
        changed = false;
        let mut next = curr.clone();
        for i in 0..next.len() {
            for j in 0..next[i].len() {
                next[i][j] = match curr[i][j] {
                    '#' if occupied(&curr, (i, j)) >= 5 => 'L',
                    'L' if occupied(&curr, (i, j)) == 0 => '#',
                    seat => seat,
                };
                changed |= next[i][j] != curr[i][j];
            }
        }
        curr = next;
    }
    curr.iter().flatten().filter(|&&seat| seat == '#').count()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let result = solve(input, |seats, (i, j)| {
            // self included
            seats
                .iter()
                .take((i + 2).min(seats.len()))
                .skip(i.saturating_sub(1))
                .flat_map(|row| row.iter().take((j + 2).min(row.len())).skip(j.saturating_sub(1)))
                .filter(|&&seat| seat == '#')
                .count()
        });
        Ok(result.to_string())
    },
    part2: |input| {
        let result = solve(input, |seats, (i, j)| {
            let (maxdi, maxdj) = (seats.len() - i - 1, seats[i].len() - j - 1);

            fn occupied(mut iter: impl Iterator<Item = char>) -> usize {
                iter.find(|&s| s != '.').map(|s| (s == '#') as usize).unwrap_or(0)
            }

            // self not included
            let mut sum = 0;
            sum += occupied((1..=i).map(|d| seats[i - d][j])); // up
            sum += occupied((1..=j).map(|d| seats[i][j - d])); // left
            sum += occupied((1..=maxdi).map(|d| seats[i + d][j])); // down
            sum += occupied((1..=maxdj).map(|d| seats[i][j + d])); // right
            sum += occupied((1..=i.min(j)).map(|d| seats[i - d][j - d])); // up-left
            sum += occupied((1..=i.min(maxdj)).map(|d| seats[i - d][j + d])); // up-right
            sum += occupied((1..=(maxdi).min(j)).map(|d| seats[i + d][j - d])); // down-left
            sum += occupied((1..=(maxdi).min(maxdj)).map(|d| seats[i + d][j + d])); // down-right

            sum
        });
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(2453, 2159);
