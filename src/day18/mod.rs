use crate::Solution;
use Expr::{Operand, Operator};
use Operation::{Add, Mul, Par};

#[derive(Debug)]
enum Expr {
    Operand(i64),
    Operator(Operation),
}

#[derive(Debug)]
enum Operation {
    Par,
    Mul,
    Add,
}

fn to_post_expr(expr: &str, prec: fn(&Operation) -> usize) -> Vec<Expr> {
    let mut st = vec![];
    let mut rs = vec![];
    for c in expr.chars() {
        match c {
            ' ' => continue,
            '0'..='9' => rs.push(Operand(c as i64 - '0' as i64)),
            '(' => st.push(Operator(Par)),
            ')' => loop {
                match st.pop() {
                    Some(Operator(Par)) | None => break,
                    Some(op) => rs.push(op),
                };
            },
            op => {
                let op: Operation = match op {
                    '+' => Add,
                    '*' => Mul,
                    _ => panic!(format!("unexpected operator: '{}'", c)),
                };
                loop {
                    match st.last() {
                        Some(Operator(top)) if prec(&top) >= prec(&op) => rs.push(st.pop().unwrap()),
                        _ => break,
                    }
                }
                st.push(Operator(op))
            }
        }
    }
    rs.extend(st.into_iter().rev());
    rs
}

fn evaluate(expr: &[Expr]) -> i64 {
    let mut st: Vec<i64> = vec![];
    for c in expr {
        match c {
            &Operand(c) => st.push(c),
            operator => {
                let (a, b) = (st.pop().unwrap(), st.pop().unwrap());
                let r = match operator {
                    Operator(Add) => a + b,
                    Operator(Mul) => a * b,
                    _ => panic!(format!("unexpected operator: {:?}", operator)),
                };
                st.push(r);
            }
        }
    }
    st[0]
}

fn solve(input: &str, prec: fn(&Operation) -> usize) -> i64 {
    input
        .lines()
        .map(|expr| to_post_expr(expr, prec))
        .map(|expr| evaluate(&expr))
        .sum()
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let result = solve(input, |op| match op {
            Par => 0,
            Add => 1,
            Mul => 1,
        });
        Ok(result.to_string())
    },
    part2: |input| {
        let result = solve(input, |op| match op {
            Par => 0,
            Add => 2,
            Mul => 1,
        });
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(16332191652452, 351175492232654);
