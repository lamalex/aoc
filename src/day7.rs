#[derive(Clone, Copy, Debug)]
pub enum Token {
    Output(u64),
    Operand(u64),
    Operation(fn(u64, u64) -> u64)
}

#[cfg(test)]
mod test {

}

pub mod parser {
    use nom::{bytes::complete::tag, character::complete::{newline, space1, u64}, multi::separated_list1, sequence::separated_pair, IResult};

    pub fn parse_lines(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
        separated_list1(newline, parse_line)(input)
    }

    fn parse_line(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
        separated_pair(u64, tag(": "), separated_list1(space1, u64))(input)
    }

    #[cfg(test)]
    mod test {
        use test_case::test_case;

        use super::{parse_line, parse_lines};

        #[test]
        fn test_parse_sample_input() {
            let expected = vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20])
            ];

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
            let (_, actual) = parse_lines(input).unwrap();
            assert_eq!(actual, expected);
        }

        #[test_case("190: 10 19", (190, vec![10,19]))]
        #[test_case("3267: 81 40 27", (3267, vec![81,40,27]))]
        fn test_parse_line(input: &str, expected: (u64, Vec<u64>)) {
            let (_, actual) = parse_line(input).unwrap();
            assert_eq!(actual, expected);
        }
    }
}