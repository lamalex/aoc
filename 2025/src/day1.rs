use std::ops::AddAssign;

pub fn main() {
    let mut dial = Dial::new();
    let data = include_str!("../data/day1.txt");

    let (_, (hits, passes)) = parser::parse_apply_and_count(&mut dial, data).unwrap();
    println!("{hits:?}");
    println!("{passes:?}");
}

#[derive(Debug, PartialEq, Eq)]
struct Dial {
    state: i32,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Left(i32),
    Right(i32),
}

impl Dial {
    #[must_use]
    pub fn new() -> Self {
        Self { state: 50 }
    }

    pub fn state(&self) -> i32 {
        self.state
    }
}

impl AddAssign<Instruction> for Dial {
    fn add_assign(&mut self, rhs: Instruction) {
        self.state = match rhs {
            Instruction::Left(v) => self.state + v,
            Instruction::Right(v) => self.state - v,
        }
        .rem_euclid(100)
    }
}

mod parser {
    #[derive(Debug, PartialEq, Eq)]
    pub struct ZeroStopCount(pub usize);

    #[derive(Debug, PartialEq, Eq)]
    pub struct PassZeroCount(pub i32);

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, line_ending},
        combinator::{map_res, opt},
        multi::fold_many0,
        sequence::{terminated, tuple},
        IResult,
    };

    use super::{Dial, Instruction};

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        let (input, (dir, num)) = tuple((
            alt((tag("R"), tag("L"))),
            map_res(digit1, str::parse::<i32>),
        ))(input)?;

        let inst = match dir {
            "R" => Instruction::Right(num),
            "L" => Instruction::Left(num),
            _ => unreachable!(),
        };

        Ok((input, inst))
    }

    pub fn parse_apply_and_count<'a>(
        dial: &'a mut Dial,
        input: &'a str,
    ) -> IResult<&'a str, (ZeroStopCount, PassZeroCount)> {
        let parser = terminated(parse_instruction, opt(line_ending));

        fold_many0(
            parser,
            || (ZeroStopCount(0), PassZeroCount(0)),
            |(mut hits, mut passes), instr| {
                match instr {
                    Instruction::Left(v) => {
                        passes.0 += (dial.state() + v) / 100;
                    }
                    Instruction::Right(v) => {
                        if dial.state() != 0 && v >= dial.state() {
                            passes.0 += 1;
                        }
                        passes.0 += (dial.state - v).abs() / 100;
                    }
                };

                *dial += instr;

                if dial.state() == 0 {
                    hits.0 += 1;
                }

                (hits, passes)
            },
        )(input)
    }

    #[cfg(test)]
    mod test {
        use super::super::*;

        use test_case::test_case;

        #[test_case("R51", Instruction::Right(51))]
        #[test_case("L1", Instruction::Left(1))]
        fn parse_instruction(instruction: &str, expected: Instruction) {
            let (_, instr) = super::parse_instruction(instruction).unwrap();
            assert_eq!(instr, expected);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test_case::test_case;

    #[test_case(Vec::from([Instruction::Right(51)]), 99)]
    #[test_case(Vec::from([Instruction::Left(51)]), 1)]
    #[test_case(Vec::from([Instruction::Left(50)]), 0)]
    #[test_case(Vec::from([Instruction::Right(50)]), 0)]
    fn add_assign_mut_state(instructions: Vec<Instruction>, expected: i32) {
        let dial = instructions
            .into_iter()
            .fold(Dial::new(), |mut dial, next| {
                dial += next;
                dial
            });
        assert_eq!(dial.state(), expected);
    }
}
