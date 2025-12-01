pub mod parser {

    use nom::{
        character::complete::{digit1, space1},
        combinator::map_res,
        error::{FromExternalError, ParseError},
        sequence::separated_pair,
        IResult, Parser,
    };
    use std::num::ParseIntError;

    pub fn parse_nums(input: &str) -> IResult<&str, (i32, i32)> {
        separated_pair(map_to_int(digit1), space1, map_to_int(digit1))(input)
    }

    fn map_to_int<'a, F, E1>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, i32, E1>
    where
        F: Parser<&'a str, &'a str, E1>,
        E1: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
    {
        map_res(f, |digits: &str| digits.parse::<i32>())
    }

    #[cfg(test)]
    mod test {
        use super::parse_nums;

        #[test]
        fn test_parse_num_pair() {
            let input = "3   4";
            let expected = (3, 4);
            let actual = parse_nums(input);
            assert!(actual.is_ok());
            let (_, actual) = actual.unwrap();
            assert_eq!(actual, expected);
        }
    }
}
