use std::iter::Iterator;

fn find_sum() -> i64 {
    let input = include_str!("../data/day1.txt");
    input.lines().fold(0, |sum, line| {
        sum + format!(
            "{}{}",
            scan_and_parse_to_int(line.chars()),
            scan_and_parse_to_int(line.chars().rev())
        )
        .parse::<i64>()
        .unwrap()
    })
}

fn scan_and_parse_to_int(mut iter: impl Iterator<Item = char>) -> i64 {
    iter.find(|c| c.is_numeric())
        .unwrap_or('0')
        .to_digit(10)
        .unwrap_or(0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_find_sum() {
        assert_eq!(find_sum(), 55017);
    }

    #[test_case("abc".chars(), 0)]
    #[test_case("ab6c".chars(), 6)]
    #[test_case("69ab68c".chars(), 6)]
    #[test_case("abcd9".chars(), 9)]
    #[test_case("abc".chars().rev(), 0)]
    #[test_case("ab6c".chars().rev(), 6)]
    #[test_case("69ab68c".chars().rev(), 8)]
    #[test_case("abcd9".chars().rev(), 9)]
    fn test_scan_and_parse_to_int(iter: impl Iterator<Item = char>, expected: i64) {
        assert_eq!(scan_and_parse_to_int(iter), expected);
    }
}
