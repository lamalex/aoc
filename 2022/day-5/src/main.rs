use types::MoveInstruction;

fn main() {
    let input = include_str!("input.txt");
    let (input, mut stacks) = parser::parse_crate_drawing(input).unwrap();

    for line in input.lines() {
        let instruction: MoveInstruction = line.into();

        for c in (0..instruction.n)
            .map(|_| stacks.0[instruction.from - 1].pop_front().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            stacks.0[instruction.to - 1].push_front(c);
        }
    }

    for s in stacks.0.iter() {
        if let Some(c) = s.iter().nth(0) {
            print!("{c}");
        }
    }

    println!("");
}

mod types {
    use std::collections::VecDeque;

    use crate::parser;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Stacks(pub Vec<VecDeque<Crate>>);

    impl Stacks {
        pub fn new(data: Vec<VecDeque<Crate>>) -> Self {
            Self(data)
        }
    }

    impl std::fmt::Display for Stacks {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}\n{}",
                self.0
                    .iter()
                    .enumerate()
                    .map(|(i, c)| format!("{i}: {:?}", c.iter().collect::<Vec<&Crate>>()))
                    .collect::<Vec<String>>()
                    .join("\n"),
                (1..=self.0.iter().map(|stack| stack.len()).max().unwrap()).fold(
                    String::from("   "),
                    |mut acc, next| {
                        acc.push_str(&format!("    {next}       "));
                        acc
                    }
                )
            )
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MoveInstruction {
        pub n: usize,
        pub to: usize,
        pub from: usize,
    }

    impl std::fmt::Display for MoveInstruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "move {} from {} to {}", self.n, self.from, self.to)
        }
    }

    impl From<&str> for MoveInstruction {
        fn from(value: &str) -> Self {
            parser::parse_move_cmd(value).unwrap().1
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Crate(char);

    impl From<char> for Crate {
        fn from(value: char) -> Self {
            match value {
                'A'..='Z' => Self(value),
                _ => unreachable!("I know this can't happen"),
            }
        }
    }

    impl std::fmt::Display for Crate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}

mod parser {
    use std::collections::VecDeque;

    use crate::types::{Crate, MoveInstruction, Stacks};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::complete::{digit1, multispace1, newline, satisfy, space1},
        combinator::{map, map_res},
        multi::{separated_list0, separated_list1},
        sequence::{delimited, preceded, terminated, tuple},
        IResult,
    };

    pub fn parse_move_cmd(i: &str) -> IResult<&str, MoveInstruction> {
        map(
            tuple((
                preceded(tag("move "), parse_number),
                preceded(tag(" from "), parse_number),
                preceded(tag(" to "), parse_number),
            )),
            |(n, from, to)| MoveInstruction { n, from, to },
        )(i)
    }

    fn parse_number(i: &str) -> IResult<&str, usize> {
        map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
            s.parse::<usize>()
        })(i)
    }

    pub fn parse_crate_drawing(i: &str) -> IResult<&str, Stacks> {
        let (i, crates): (&str, Vec<Vec<Option<Crate>>>) = terminated(
            parse_lines_of_crates,
            delimited(multispace1, separated_list0(space1, digit1), multispace1),
        )(i)?;

        let num_stacks = crates.iter().last().unwrap().len();

        Ok((
            i,
            Stacks::new(crates.into_iter().fold(
                vec![VecDeque::default(); num_stacks],
                |mut acc, row| {
                    for (i, c) in row.into_iter().enumerate() {
                        if let Some(c) = c {
                            acc[i].push_back(c);
                        }
                    }

                    acc
                },
            )),
        ))
    }

    fn parse_lines_of_crates(i: &str) -> IResult<&str, Vec<Vec<Option<Crate>>>> {
        separated_list1(newline, parse_line_of_crates)(i)
    }

    fn parse_line_of_crates(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
        separated_list1(tag(" "), parse_maybe_crate)(i)
    }

    fn parse_maybe_crate(i: &str) -> IResult<&str, Option<Crate>> {
        alt((map(parse_crate, Option::Some), parse_crate_space))(i)
    }

    fn parse_crate(i: &str) -> IResult<&str, Crate> {
        let (i, id) = delimited(tag("["), satisfy(|c| ('A'..='Z').contains(&c)), tag("]"))(i)?;
        Ok((i, Crate::from(id)))
    }

    fn parse_crate_space<T>(i: &str) -> IResult<&str, Option<T>> {
        map(tag("   "), |_| None)(i)
    }

    #[cfg(test)]
    mod test {
        use std::collections::VecDeque;

        use nom::{error::ErrorKind, Err};

        use crate::{
            parser::{parse_crate_drawing, parse_crate_space, parse_line_of_crates},
            types::{Crate, MoveInstruction, Stacks},
        };

        use super::{parse_crate, parse_lines_of_crates, parse_move_cmd};

        #[test]
        fn parses_single_crate() {
            let expected = Crate::from('T');
            let (_, actual) = parse_crate("[T]").unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn parses_malformed_crate() {
            let error = parse_crate("[t]");
            assert_eq!(
                Err(Err::Error(nom::error::make_error("t]", ErrorKind::Satisfy))),
                error
            );
        }

        #[test]
        fn parses_crate_space() {
            let (i, actual) = parse_crate_space::<()>("   ").unwrap();
            assert_eq!(None, actual);
            assert_eq!("", i);
        }

        #[test]
        fn parses_crate_line() {
            let input = "[T]     [Q]             [S]        ";
            let expected = vec![
                Some(Crate::from('T')),
                None,
                Some(Crate::from('Q')),
                None,
                None,
                None,
                Some(Crate::from('S')),
                None,
                None,
            ];

            let (_, actual) = parse_line_of_crates(input).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn parses_multiple_lines_of_crates() {
            let input = "    [D]    
[N] [C]    
[Z] [M] [P]
";

            let expected = vec![
                vec![None, Some(Crate::from('D')), None],
                vec![Some(Crate::from('N')), Some(Crate::from('C')), None],
                vec![
                    Some(Crate::from('Z')),
                    Some(Crate::from('M')),
                    Some(Crate::from('P')),
                ],
            ];

            let (_, actual) = parse_lines_of_crates(input).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn parses_full_crate_drawing() {
            let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

            let expected = Stacks::new(vec![
                VecDeque::from(vec![Crate::from('N'), Crate::from('Z')]),
                VecDeque::from(vec![Crate::from('D'), Crate::from('C'), Crate::from('M')]),
                VecDeque::from(vec![Crate::from('P')]),
            ]);

            let (_, actual) = parse_crate_drawing(input).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_parse_move_cmd() {
            let cmd = parse_move_cmd("move 1 from 69 to 666").unwrap().1;

            assert_eq!(
                MoveInstruction {
                    n: 1,
                    from: 69,
                    to: 666
                },
                cmd
            );
        }
    }
}
