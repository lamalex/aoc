fn find_sum(input: &str) -> u32 {
    input.lines().flat_map(parser::parse_line).fold(0, |acc, (_, val)| acc + val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sum() {
        let data = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;
        assert_eq!(find_sum(data), 281);
    }
}

mod parser {
    use nom::{
        branch::alt, bytes::complete::tag_no_case, character::complete::digit1, combinator::{map, value}, error::{Error, ErrorKind}, FindSubstring, IResult
    };

    const ZERO: &str = "zero";
    const ONE: &str = "one";
    const TWO: &str = "two";
    const THREE: &str = "three";
    const FOUR: &str = "four";
    const FIVE: &str = "five";
    const SIX: &str = "six";
    const SEVEN: &str = "seven";
    const EIGHT: &str = "eight";
    const NINE: &str = "nine";

    pub fn parse_line(input: &str) -> IResult<&str, u32> {
        let mut res = vec![];
        let mut input = input;
        
        while input.len() > 0 {
            if let Ok((rem, nums)) = parse_one(input) {
                res.push(nums);
                input = rem;
            } else {
                input = &input[1..];
            }
        }

        let res = format!("{}{}", res[0], res[res.len() - 1]).parse().unwrap();

        Ok((input, res))
    }

    fn parse_one(input: &str) -> IResult<&str, u32> {
        alt((parse_word_digit, parse_digit))(input)
    }

    fn parse_word_digit(input: &str) -> IResult<&str, u32> {
        let (rem, parsed) = alt((
            value(0, tag_no_case(ZERO)),
            value(1, tag_no_case(ONE)),
            value(2, tag_no_case(TWO)),
            value(3, tag_no_case(THREE)),
            value(4, tag_no_case(FOUR)),
            value(5, tag_no_case(FIVE)),
            value(6, tag_no_case(SIX)),
            value(7, tag_no_case(SEVEN)),
            value(8, tag_no_case(EIGHT)),
            value(9, tag_no_case(NINE)),
        ))(input)?;

        let idx = match input.find_substring(rem) {
            Some(0) => input.len() - 1,
            Some(x) => x - 1, 
            None => input.len(),
        };

        Ok((&input[idx..], parsed))
    }

    fn parse_digit(input: &str) -> IResult<&str, u32> {
        let (first, rem) = match input.chars().next() {
            Some(c) if c.is_digit(10) => input.split_at(1),
            _ => return Err(nom::Err::Error(Error::new(input, ErrorKind::Digit))),
        };

        let res: Result<u32, _> = first.parse();
        match res {
            Ok(parsed) => Ok((rem, parsed)),
            Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::Digit))),
        }
    }

    fn parse_digits(input: &str) -> IResult<&str, Vec<u32>> {
        map(digit1, |d: &str| {
            d.chars().map(|d: char| d.to_digit(10).unwrap()).collect()
        })(input)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use test_case::test_case;

        #[test_case("two1nine", 29)]
        #[test_case("eightwothree", 83)]
        #[test_case("abcone2threexyz", 13)]
        #[test_case("xtwone3four", 24)]
        #[test_case("4nineeightseven2", 42)]
        #[test_case("zoneight234", 14)]
        #[test_case("7pqrstsixteen", 76)]
        fn test_scan_and_parse_to_int(input: &str, expected: u32) {
            let (_, actual) = parse_line(input).unwrap();
            assert_eq!(actual, expected);
        }

        #[test_case("zero", 0)]
        #[test_case("one", 1)]
        #[test_case("two", 2)]
        #[test_case("three", 3)]
        #[test_case("four", 4)]
        #[test_case("five", 5)]
        #[test_case("six", 6)]
        #[test_case("seven", 7)]
        #[test_case("eight", 8)]
        #[test_case("nine", 9)]
        fn test_parse_word_digit(word: &str, expected: u32) {
            let (rem, actual) = parse_word_digit(word).unwrap();
            assert_eq!(actual, expected);
            assert_eq!(&word[word.len() - 1..], rem)
        }

        #[test_case("69", vec![6,9])]
        #[test_case("420", vec![4,2,0])]
        #[test_case("0193", vec![0,1,9,3])]
        fn test_parse_digits(input: &str, expected: Vec<u32>) {
            let (_, parsed) = parse_digits(input).unwrap();
            assert_eq!(parsed, expected);
        }

        #[test_case("6", 6)]
        #[test_case("4", 4)]
        #[test_case("0", 0)]
        fn test_parse_digit(input: &str, expected: u32) {
            let (_, parsed) = parse_digit(input).unwrap();
            assert_eq!(parsed, expected);
        }
    }
}
