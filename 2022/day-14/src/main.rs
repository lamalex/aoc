fn main() {
    let input = include_str!("input.txt");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

impl From<(u64, u64)> for Point {
    fn from((x, y): (u64, u64)) -> Self {
        Self { x, y }
    }
}

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::{self, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    use crate::Point;

    fn parse_list_of_lists_of_points(i: &str) -> IResult<&str, Vec<Vec<Point>>> {
        separated_list1(line_ending, parse_list_of_points)(i)
    }

    fn parse_list_of_points(i: &str) -> IResult<&str, Vec<Point>> {
        separated_list1(tag(" -> "), parse_pair_to_point)(i)
    }

    fn parse_pair_to_point(i: &str) -> IResult<&str, Point> {
        map(
            separated_pair(complete::u64, tag(","), complete::u64),
            Point::from,
        )(i)
    }

    #[cfg(test)]
    mod test {
        use super::parse_pair_to_point;
        use crate::{
            parser::{parse_list_of_lists_of_points, parse_list_of_points},
            Point,
        };
        use test_case::test_case;

        #[test]
        fn test_parse_list_of_lists_of_points() {
            let i = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

            let (_, actual) = parse_list_of_lists_of_points(i).unwrap();
            let expected = vec![
                vec![
                    Point::from((498, 4)),
                    Point::from((498, 6)),
                    Point::from((496, 6)),
                ],
                vec![
                    Point::from((503, 4)),
                    Point::from((502, 4)),
                    Point::from((502, 9)),
                    Point::from((494, 9)),
                ],
            ];

            assert_eq!(expected, actual);
        }

        #[test_case("498,4 -> 498,6 -> 496,6", vec![Point::from((498,4)), Point::from((498,6)), Point::from((496,6))])]
        #[test_case("503,4 -> 502,4 -> 502,9 -> 494,9", vec![Point::from((503,4)), Point::from((502,4)), Point::from((502,9)), Point::from((494,9))])]
        fn test_parse_rock_loc(i: &str, expected: Vec<Point>) {
            let (_, actual) = parse_list_of_points(i).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_parse_pair_to_point() {
            let input = "498,4";
            let (_, actual) = parse_pair_to_point(input).unwrap();

            assert_eq!(Point { x: 498, y: 4 }, actual);
        }
    }
}
