use aoc_2024::day7::{parser::parse_lines, Token};
use itertools::Itertools;

pub fn main() {
    let input = include_str!("../../data/day7.txt");
    let res = compute(input);

    println!("{res}");
}

fn compute(input: &str) -> u64 {
    let (_, lines) = parse_lines(input).unwrap();
    lines
        .into_iter()
        .filter(|(output, line)| {
            (0..line.len() - 1)
                .map(|_| {
                    vec![
                        Token::Operation(|a: u64, b: u64| format!("{a}{b}").parse().unwrap()),
                        Token::Operation(u64::saturating_mul),
                        Token::Operation(u64::saturating_add),
                    ]
                })
                .multi_cartesian_product()
                .map(|ops| {
                    vec![Token::Operation(u64::saturating_add)]
                        .into_iter()
                        .chain(ops.into_iter())
                        .interleave(line.into_iter().map(|&v| Token::Operand(v)))
                })
                .any(|chain| {
                    chain
                        .chunks(2)
                        .into_iter()
                        .flat_map(|chunk| chunk.tuples::<(_, _)>())
                        // .inspect(|pair| { dbg!(pair); })
                        .fold(0u64, |acc, (operation, operand)| {
                            let func = match operation {
                                Token::Operation(func) => func,
                                _ => panic!("first value must be an operation"),
                            };
                            let rhs = match operand {
                                Token::Operand(value) => value,
                                _ => panic!("second value must be an operand"),
                            };

                            func(acc, rhs)
                        })
                        == *output
                })
        })
        .map(|(output, _)| output)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::compute;

    #[test]
    fn test_sample_input() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        assert_eq!(compute(input), 11387)
    }
}
