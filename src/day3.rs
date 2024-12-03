pub mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res, value},
        sequence::{delimited, separated_pair},
        IResult,
    };

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum Conditional {
        Do,
        Dont,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum Instruction {
        Conditional(Conditional),
        Mul((i64, i64)),
    }

    fn chomp_and_parse<T, F>(input: &str, mut f: F) -> IResult<&str, Vec<T>>
    where F: FnMut(&str) -> IResult<&str, T>
    {
        let mut items: Vec<T> = Vec::with_capacity(1024);
        let mut input = input;

        while input.len() > 0 {
            if let Ok((rem, item)) = f(input) {
                items.push(item);
                input = rem;
            } else {
                input = &input[1..];
            }
        }

        Ok((input, items))
    }

    pub fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        chomp_and_parse(input, parse_instruction)
    }

    pub fn parse_muls(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
        chomp_and_parse(input, parse_mul)
    }

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            alt((
                value(Instruction::Conditional(Conditional::Do), tag("do()")),
                value(Instruction::Conditional(Conditional::Dont), tag("don't()")),
            )),
            map(parse_mul, |pair| Instruction::Mul(pair)),
        ))(input)
    }

    fn parse_mul(input: &str) -> IResult<&str, (i64, i64)> {
        delimited(
            tag("mul("),
            separated_pair(
                map_res(digit1, |d: &str| d.parse::<i64>()),
                tag(","),
                map_res(digit1, |d: &str| d.parse::<i64>()),
            ),
            tag(")"),
        )(input)
    }

    #[cfg(test)]
    mod test {
        use crate::day3::parser::{
            parse_instruction, parse_instructions, Conditional, Instruction,
        };

        #[test]
        fn test_parse_expr_with_cond() {
            let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
            let (_, actual) = parse_instructions(input).unwrap();
            assert_eq!(
                actual,
                vec![
                    Instruction::Mul((2, 4)),
                    Instruction::Conditional(Conditional::Dont),
                    Instruction::Mul((5, 5)),
                    Instruction::Mul((11, 8)),
                    Instruction::Conditional(Conditional::Do),
                    Instruction::Mul((8, 5))
                ]
            );
        }

        #[test]
        fn test_parse_mul() {
            let input = "mul(2,4)";
            let (_, actual) = parse_instruction(input).unwrap();
            assert_eq!(actual, Instruction::Mul((2, 4)));
        }
    }
}
