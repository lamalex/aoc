pub mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::digit1,
        combinator::map_res,
        sequence::{delimited, separated_pair},
        IResult,
    };

    pub fn parse_muls(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
        let mut pairs: Vec<(i64, i64)> = Vec::with_capacity(1024);
        let mut input = input;

        while input.len() > 0 {
            if let Ok((rem, pair)) = parse_mul(input) {
                pairs.push(pair);
                input = rem;
            } else {
                input = &input[1..];
            }
        }

        Ok((input, pairs))
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
        use crate::day3::parser::{parse_mul, parse_muls};

        #[test]
        fn test_parse_expression() {
            let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
            let (_, actual) = parse_muls(input).unwrap();
            assert_eq!(actual, vec![(2,4), (5,5), (11,8), (8,5)]);
            assert_eq!(actual.iter().map(|(a,b)| a*b).sum::<i64>(), 161);
        }

        #[test]
        fn test_parse_mul() {
            let input = "mul(2,4)";
            let (_, actual) = parse_mul(input).unwrap();
            assert_eq!(actual, (2,4));
        }
    }
}
