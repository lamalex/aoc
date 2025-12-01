use itertools::{self, Itertools};
use nom::{
    
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::one_of,
    combinator::{self, eof, peek},
    multi::many_till,
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let result = parse_file_2(input);
    println!("result: {}", result);
}

fn parse_line(input: &str) -> i32 {
    // find first digit in input reading from left to right
    let first_digit = input.find(char::is_numeric).unwrap();
    // find last digit in input reading from right to left
    let last_digit = input.rfind(char::is_numeric).unwrap();

    format!(
        "{}{}",
        input.chars().nth(first_digit).unwrap(),
        input.chars().nth(last_digit).unwrap()
    )
    .parse::<i32>()
    .unwrap()
}

fn parse_file_2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line_2)
        .map(|line| line.unwrap().1)
        .sum::<u32>()
}

fn parse_line_2(input: &str) -> IResult<&str, u32> {
    combinator::map(
        combinator::map(
            many_till(alt((
                combinator::map(parse_number, |d| Some(d)),
                combinator::map(take(1_usize), |_| Option::<u32>::None),
            )), eof),
            |(list,_)| list.into_iter().filter_map(|x| x).collect::<Vec<_>>(),
        ),
        |nums| {
            format!(
                "{}{}",
                nums.first().unwrap(),
                nums.last().unwrap()
            )
            .parse()
            .unwrap()
        },
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    dbg!(input);
    alt((
        parse_str_digit,
        combinator::map(one_of("0123456789"), |d| d.to_string().parse().unwrap()),
    ))(input)
}

fn parse_str_digit(input: &str) -> IResult<&str, u32> {
    let num_names = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into_iter()
    .map(|(s, d)| combinator::map(tag(s), move |_: &str| d))
    .collect_tuple::<(_, _, _, _, _, _, _, _, _, _)>()
    .unwrap();

    let (input, num) = peek(alt(num_names))(input)?;
    let (input, _) = take(1_usize)(input)?;

    Ok((input, num))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("7three3zmmfvxtsdsthree", 73)]
    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    fn test_parse_line(line: &str, expected: i32) {
        assert_eq!(parse_line(line), expected)
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
    fn test_parse_str_digit(input: &str, expected: u32) {
        let (_, digit) = parse_str_digit(input).unwrap();
        assert_eq!(digit, expected);
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
    #[test_case("0", 0)]
    #[test_case("1", 1)]
    #[test_case("2", 2)]
    #[test_case("3", 3)]
    #[test_case("4", 4)]
    #[test_case("5", 5)]
    #[test_case("6", 6)]
    #[test_case("7", 7)]
    #[test_case("8", 8)]
    #[test_case("9", 9)]
    #[test_case("69", 6)]
    fn test_parse_number(input: &str, expected: u32) {
        let (_, digit) = parse_number(input).unwrap();
        assert_eq!(digit, expected);
    }

    #[test_case("7three3zmmfvxtsdsthree", 73)]
    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    #[test_case("two1nine", 29)]
    #[test_case("eightwothree", 83)]
    #[test_case("abcone2threexyz", 13)]
    #[test_case("xtwone3four", 24)]
    #[test_case("4nineeightseven2", 42)]
    #[test_case("zoneight234", 14)]
    #[test_case("7pqrstsixteen", 76)]
    #[test_case("oneight", 18)]
    fn test_parse_line_2(input: &str, expected: u32) {
        let (_, number) = parse_line_2(input).unwrap();
        assert_eq!(number, expected);
    }

    #[test]
    fn test_parse_sample_input_2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

        assert_eq!(parse_line_2("two1nine").unwrap().1, 29);
        assert_eq!(parse_line_2("eightwothree").unwrap().1, 83);
        assert_eq!(parse_line_2("abcone2threexyz").unwrap().1, 13);
        assert_eq!(parse_line_2("xtwone3four").unwrap().1, 24);
        assert_eq!(parse_line_2("4nineeightseven2").unwrap().1, 42);
        assert_eq!(parse_line_2("zoneight234").unwrap().1, 14);
        assert_eq!(parse_line_2("7pqrstsixteen").unwrap().1, 76);
        assert_eq!(parse_line_2("69").unwrap().1, 69);
        assert_eq!(parse_file_2(input), 281);
    }
}
