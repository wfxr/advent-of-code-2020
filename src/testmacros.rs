#[macro_export]
macro_rules! solution_test {
    ($part1:expr, $part2:expr) => {
        mod test {
            mod solution {
                use super::super::*;
                #[test]
                fn part1() {
                    let input = include_str!("input");
                    let res = (SOLUTION.part1)(&input).unwrap();
                    assert_eq!(res, stringify!($part1));
                }

                #[test]
                fn part2() {
                    let input = include_str!("input");
                    let res = (SOLUTION.part2)(&input).unwrap();
                    assert_eq!(res, stringify!($part2));
                }
            }
        }
    };
}
