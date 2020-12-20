#[macro_export]
macro_rules! test {
    ($part:ident $(,$name:ident: $input:expr => $expected:expr)* $(,)?) => {
        mod $part {
            use super::super::SOLUTION;
            #[allow(unused_imports)]
            use crate::input;
            $(
                #[test]
                fn $name() {
                    let res = (SOLUTION.$part)($input).unwrap();
                    assert_eq!(res, stringify!($expected));
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! solution_test {
    ($part:ident => $answer:expr) => {
        #[test]
        fn $part() {
            let input = include_str!("input");
            let res = (super::SOLUTION.$part)(&input).unwrap();
            assert_eq!(stringify!($answer), res);
        }
    };
}

#[macro_export]
macro_rules! input {
    ($line:expr) => { $line };
    ($line:expr, $($rest:expr),+ $(,)?) => { concat!($line, '\n', input!($($rest),+)) };
}
