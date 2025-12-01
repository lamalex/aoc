pub fn main() {
    let input = include_str!("../data/day2.txt");
    let sum = day2::sum_invalid(input);

    println!("Sum of invalid: {sum}");
}

mod day2 {
    pub fn sum_invalid(input: &str) -> u64 {
        let (_, ranges) = parser::parse_ranges(input).unwrap();
        ranges.into_iter().flatten().filter(|&n| is_repeated_half(n)).sum()
    }

    fn is_repeated_half(n: u64) -> bool {
        let mut digits = 0;
        let mut tmp = n;
        while tmp > 0 {
            tmp /= 10;
            digits += 1;
        }

        if digits == 0 || digits % 2 != 0 {
            return false;
        }

        let half = digits / 2;
        let pow10 = 10u64.pow(half as u32);

        let right = n % pow10;
        let left = n / pow10;

        left == right
    }

    #[cfg(test)]
    mod test {
        use test_case::test_case;

        #[test_case("119-210", 0)]
        #[test_case("42-65", 99)]
        fn test_sum_invalid_over_range(input: &str, expected: u64) {
            assert_eq!(super::sum_invalid(input), expected)
        }

        #[test]
        fn test_sample_input() {
            let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
            let sum = super::sum_invalid(input);

            assert_eq!(sum, 1227775554);
        }

        #[test_case(11, true)]
        #[test_case(1188511885, true)]
        #[test_case(13, false)]
        fn is_repeated_half(v: u64, expected: bool) {
            assert_eq!(super::is_repeated_half(v), expected)
        }
    }
        #[cfg(test)]
        mod test {
            use test_case::test_case;

            #[test_case("11-22", 11..=22)]
            #[test_case("95-115", 95..=115)]
            #[test_case("998-1012", 998..=1012)]
            fn parse_range(input: &str, expected: std::ops::RangeInclusive<u64>) {
                let (_, range) = super::parse_range(input).unwrap();
                assert_eq!(range, expected);
            }

            #[test_case(
mod parser {
    use nom::{
        bytes::complete::tag, character::complete::u64, combinator::map,
        multi::separated_list1, sequence::separated_pair, IResult,
    };

    pub fn parse_ranges(input: &str) -> IResult<&str, Vec<std::ops::RangeInclusive<u64>>> {
        separated_list1(tag(","), parse_range)(input)
    }

    pub fn parse_range(input: &str) -> IResult<&str, std::ops::RangeInclusive<u64>> {
        map(
            separated_pair(u64, tag("-"), u64),
            |(start, end)| start..=end,
        )(input)
    }
                "11-22,95-115,998-1012,1188511880-1188511890",
                Vec::from([
                    11..=22,
                    95..=115,
                    998..=1012,
                    1188511880..=1188511890
                ])
            )]
            fn parse_ranges(input: &str, expected: Vec<std::ops::RangeInclusive<u64>>) {
                let (_, range) = super::parse_ranges(input).unwrap();
                assert_eq!(range, expected);
            }
        }
    }
}
