use std::io::{self, BufRead};

fn p1_solve(adapters: &[usize]) -> usize {
    let mut nums = vec![0];
    nums.extend(adapters.iter());
    nums.sort_unstable();

    let mut delta = [0, 0, 0, 1];
    nums.iter()
        .zip(nums.iter().skip(1))
        .for_each(|(curr, next)| delta[next - curr] += 1);
    delta[1] * delta[3]
}

fn split(adapters: &[usize]) -> Vec<Vec<usize>> {
    let mut nums = vec![0];
    nums.extend(adapters.iter());
    nums.sort_unstable();

    let mut groups = Vec::new();
    let mut pos = 0;
    for i in 0..nums.len() - 1 {
        if nums[i + 1] - nums[i] == 3 {
            groups.push(nums[pos..i + 1].to_vec());
            pos = i + 1;
        }
    }
    groups
}

fn p2_solve(adapters: &[usize]) -> usize {
    split(adapters).iter().fold(1, |acc, group| acc * p2_solve_group(group))
}

fn p2_solve_group(group: &[usize]) -> usize {
    match group.len() {
        0 | 1 => 1,
        2 => (group[1] - group[0] <= 3) as usize,
        _ => {
            let mut sgroup: Vec<_> = group[1..].to_vec();
            sgroup[0] = group[0];
            p2_solve_group(&group[1..]) + p2_solve_group(&sgroup)
        }
    }
}

fn main() {
    let inputs: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let result = p1_solve(&inputs);
    println!("part 1 result: {}", result);
    assert_eq!(2484, result);

    let result = p2_solve(&inputs);
    println!("part 2 result: {}", result);
    assert_eq!(15790581481472, result);
}
