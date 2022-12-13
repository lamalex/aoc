use itertools::Itertools;
use parser::parse_list_of_pairs;
use std::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    let (_, pairs) = parse_list_of_pairs(input).unwrap();
    println!(
        "{}",
        pairs
            .iter()
            .enumerate()
            .filter_map(|(i, pair)| if pair.0 <= pair.1 { Some(i + 1) } else { None })
            .inspect(move |i| println!("i = {i}"))
            .sum::<usize>()
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Unit(u32),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use Value::*;

        match self {
            Unit(u_self) => match other {
                Unit(u_other) => u_self.cmp(u_other),
                List(_) => List(vec![Unit(*u_self)]).cmp(other),
            },
            List(l_self) => match other {
                Unit(u_other) => self.cmp(&List(vec![Unit(*u_other)])),
                List(l_other) => l_self
                    .iter()
                    .zip_longest(l_other.iter())
                    .map(|zip| match zip {
                        itertools::EitherOrBoth::Both(v_s, v_o) => v_s.cmp(v_o),
                        itertools::EitherOrBoth::Left(_) => Ordering::Greater,
                        itertools::EitherOrBoth::Right(_) => Ordering::Less,
                    })
                    .find(|ord| match ord {
                        Ordering::Less | Ordering::Greater => true,
                        Ordering::Equal => false,
                    })
                    .unwrap_or(Ordering::Equal),
            },
        }
    }
}

#[cfg(test)]
const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

#[cfg(test)]
mod test {
    use crate::parser::{parse_list_of_pairs, parse_value};
    use crate::INPUT;
    use std::cmp::Ordering;
    use test_case::test_case;

    #[test]
    fn test_verify_sum() {
        let (_, pairs) = parse_list_of_pairs(INPUT).unwrap();
        let actual = pairs
            .iter()
            .enumerate()
            .filter_map(|(i, pair)| if pair.0 <= pair.1 { Some(i + 1) } else { None })
            .sum::<usize>();

        assert_eq!(13, actual);
    }

    #[test_case("[1]", "[2]", Ordering::Less)]
    #[test_case("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less)]
    #[test_case("[[1],[2,3,4]]", "[[1],4]", Ordering::Less)]
    #[test_case("[9]", "[[8,7,6]]", Ordering::Greater)]
    #[test_case("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less)]
    #[test_case("[7,7,7,7]", "[7,7,7]", Ordering::Greater)]
    #[test_case("[]", "[3]", Ordering::Less)]
    #[test_case("[[[]]]", "[[]]", Ordering::Greater)]
    #[test_case(
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        Ordering::Greater
    )]
    fn test_cmp_pairs(a: &str, b: &str, expected: Ordering) {
        let (_, a) = parse_value(a).unwrap();
        let (_, b) = parse_value(b).unwrap();

        assert_eq!(expected, a.cmp(&b));
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{self, line_ending},
        combinator::map,
        multi::separated_list0,
        sequence::{delimited, separated_pair, terminated},
        IResult,
    };

    pub(crate) fn parse_list_of_pairs(i: &str) -> IResult<&str, Vec<(Value, Value)>> {
        separated_list0(line_ending, terminated(parse_value_pair, line_ending))(i)
    }

    pub(crate) fn parse_value_pair(i: &str) -> IResult<&str, (Value, Value)> {
        separated_pair(parse_value, line_ending, parse_value)(i)
    }

    pub(crate) fn parse_value(i: &str) -> IResult<&str, Value> {
        map(
            delimited(
                tag("["),
                separated_list0(
                    tag(","),
                    alt((parse_value, map(complete::u32, Value::Unit))),
                ),
                tag("]"),
            ),
            |v| Value::List(v),
        )(i)
    }

    use crate::Value;

    #[cfg(test)]
    mod test {
        use super::Value::*;

        use crate::{
            parser::{parse_list_of_pairs, parse_value, parse_value_pair},
            Value, INPUT,
        };
        use test_case::test_case;

        #[test]
        fn test_parse_list_of_pairs() {
            let expected = vec![
                (
                    List(vec![Unit(1), Unit(1), Unit(3), Unit(1), Unit(1)]),
                    List(vec![Unit(1), Unit(1), Unit(5), Unit(1), Unit(1)]),
                ),
                (
                    List(vec![
                        List(vec![Unit(1)]),
                        List(vec![Unit(2), Unit(3), Unit(4)]),
                    ]),
                    List(vec![List(vec![Unit(1)]), Unit(4)]),
                ),
                (
                    List(vec![Unit(9)]),
                    List(vec![List(vec![Unit(8), Unit(7), Unit(6)])]),
                ),
                (
                    List(vec![List(vec![Unit(4), Unit(4)]), Unit(4), Unit(4)]),
                    List(vec![
                        List(vec![Unit(4), Unit(4)]),
                        Unit(4),
                        Unit(4),
                        Unit(4),
                    ]),
                ),
                (
                    List(vec![Unit(7), Unit(7), Unit(7), Unit(7)]),
                    List(vec![Unit(7), Unit(7), Unit(7)]),
                ),
                (List(vec![]), List(vec![Unit(3)])),
                (
                    List(vec![List(vec![List(vec![])])]),
                    List(vec![List(vec![])]),
                ),
                (
                    List(vec![
                        Unit(1),
                        List(vec![
                            Unit(2),
                            List(vec![
                                Unit(3),
                                List(vec![Unit(4), List(vec![Unit(5), Unit(6), Unit(7)])]),
                            ]),
                        ]),
                        Unit(8),
                        Unit(9),
                    ]),
                    List(vec![
                        Unit(1),
                        List(vec![
                            Unit(2),
                            List(vec![
                                Unit(3),
                                List(vec![Unit(4), List(vec![Unit(5), Unit(6), Unit(0)])]),
                            ]),
                        ]),
                        Unit(8),
                        Unit(9),
                    ]),
                ),
            ];

            let (_, actual) = parse_list_of_pairs(INPUT).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_parse_pair_of_values() {
            let input = "[9]
[[8,7,6]]";

            let expected = (
                List(vec![Unit(9)]),
                List(vec![List(vec![Unit(8), Unit(7), Unit(6)])]),
            );

            let (_, actual) = parse_value_pair(input).unwrap();
            assert_eq!(expected, actual);
        }

        #[test_case("[[[]]]", List(vec![List(vec![List(vec![])])]))]
        #[test_case("[9]", List(vec![Unit(9)]))]
        #[test_case("[1,1,3,1,1]", List(vec![Unit(1), Unit(1), Unit(3), Unit(1), Unit(1)]))]
        #[test_case("[[1],[2,3,4]]", List(vec![List(vec![Unit(1)]), List(vec![Unit(2), Unit(3), Unit(4)])]))]
        fn test_parse_list_of_unit(i: &str, expected: Value) {
            let (_, actual) = parse_value(i).unwrap();

            assert_eq!(expected, actual);
        }
    }
}
