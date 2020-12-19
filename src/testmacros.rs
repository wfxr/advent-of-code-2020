#[macro_export]
macro_rules! test {
    ($part:ident $(=> $answer:expr)? $(,$name:ident: $input:expr => $expected:expr)*) => {
        $(
            #[test]
            fn $part() {
                let input = include_str!("input");
                let res = (super::SOLUTION.$part)(&input).unwrap();
                assert_eq!(stringify!($answer), res);
            }
        )?
        $(
            #[test]
            fn $name() {
                let res = (super::SOLUTION.$part)($input).unwrap();
                assert_eq!(res, stringify!($expected));
            }
        )*
    };
}

#[macro_export]
macro_rules! solution_test {
    ($part1:expr, $part2:expr) => {
        mod test {
            crate::test!(part1 => $part1);
            crate::test!(part2 => $part2);
        }
    };
}
