use crate::Solution;
use ExprNode::{Operand, Operator};

#[derive(Debug)]
enum ExprNode {
    Operand(i64),
    Operator(char),
}

fn to_postfix(expr: &str, prec: fn(char) -> u8) -> Vec<ExprNode> {
    let mut st = vec![];
    let mut rs = vec![];
    for c in expr.chars() {
        match c {
            ' ' => continue,
            '0'..='9' => rs.push(Operand(c as i64 - '0' as i64)),
            '(' => st.push(Operator(c)),
            ')' => loop {
                match st.pop() {
                    Some(Operator('(')) | None => break,
                    Some(op) => rs.push(op),
                };
            },
            op => {
                loop {
                    match st.last() {
                        Some(&Operator('(')) => break,
                        Some(&Operator(top)) if prec(top) <= prec(op) => rs.push(st.pop().unwrap()),
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

fn evaluate(expr: &[ExprNode]) -> i64 {
    let mut st = vec![];
    for node in expr {
        match node {
            &Operand(val) => st.push(val),
            operator => {
                let (rhs, lhs) = (st.pop().unwrap(), st.pop().unwrap());
                let val = match operator {
                    Operator('+') => lhs + rhs,
                    Operator('*') => lhs * rhs,
                    _ => panic!(format!("unexpected operator: {:?}", operator)),
                };
                st.push(val);
            }
        }
    }
    st[0]
}

fn solve(input: &str, prec: fn(char) -> u8) -> i64 {
    input
        .lines()
        .map(|expr| to_postfix(expr, prec))
        .map(|expr| evaluate(&expr))
        .sum()
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let result = solve(input, |op| match op {
            '+' => 0,
            '*' => 0,
            _ => panic!(format!("unexpected operator: '{}'", op)),
        });
        Ok(result.to_string())
    },
    part2: |input| {
        let result = solve(input, |op| match op {
            '+' => 0,
            '*' => 1,
            _ => panic!(format!("unexpected operator: '{}'", op)),
        });
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(16332191652452, 351175492232654);
