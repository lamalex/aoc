use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReportStatus {
    Safe(Ordering),
    Unsafe,
}

pub mod parser {
    use crate::day2::ReportStatus;
    use itertools::Itertools;
    use nom::{
        character::complete::{digit1, space1},
        combinator::{map, map_res},
        multi::separated_list1,
        IResult,
    };

    pub fn parse_line_safety(input: &str, enable_damper: bool) -> IResult<&str, bool> {
        map(
            separated_list1(space1, map_res(digit1, |d: &str| d.parse::<i32>())),
            |list| {
                if is_line_safe(&list) {
                    true
                } else if enable_damper {
                    for idx in 0..list.len() {
                        let without_idx: Vec<_> = list
                            .iter()
                            .enumerate()
                            .filter(|&(i, _)| i != idx)
                            .map(|(_, &v)| v)
                            .collect();

                        let is_safe = is_line_safe(&without_idx);
                        if is_safe {
                            return true;
                        }
                    }
                    false
                } else {
                    false
                }
            },
        )(input)
    }

    fn is_line_safe(list: &[i32]) -> bool {
        list.into_iter()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| {
                if a == b || (a - b).abs() > 3 {
                    ReportStatus::Unsafe
                } else {
                    ReportStatus::Safe(a.cmp(&b))
                }
            })
            .reduce(|acc, next| match (acc, next) {
                (ReportStatus::Safe(a), ReportStatus::Safe(b)) if a == b => ReportStatus::Safe(a),
                _ => ReportStatus::Unsafe,
            })
            != Some(ReportStatus::Unsafe)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        use test_case::test_case;

        #[test_case("7 6 4 2 1", true)]
        #[test_case("1 2 7 8 9", false)]
        #[test_case("9 7 6 2 1", false)]
        #[test_case("1 3 2 4 5", false)]
        #[test_case("8 6 4 4 1", false)]
        #[test_case("1 3 6 7 9", true)]
        #[test_case("1 1 1 1 1", false)]
        fn test_parse_line_safety(line: &str, expected: bool) {
            let (_, actual) = parse_line_safety(line, false).unwrap();
            assert_eq!(actual, expected);
        }

        #[test_case("7 6 4 2 1", true)]
        #[test_case("9 5 7 8 9", true)]
        #[test_case("1 2 7 8 9", false)]
        #[test_case("9 7 6 2 1", false)]
        #[test_case("1 3 2 4 5", true)]
        #[test_case("8 6 4 4 1", true)]
        #[test_case("1 3 6 7 9", true)]
        #[test_case("1 1 1 1 1", false)]
        #[test_case("1 3 2 4 3", false)]
        #[test_case("1 2 4 5 1", true)]
        fn test_parse_line_safety_2(line: &str, expected: bool) {
            let (_, actual) = parse_line_safety(line, true).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
