use core::num;
use std::{cell::RefCell, collections::VecDeque, fmt::Debug};

fn main() {
    let input = include_str!("input.txt");
    let (i, monkeys) = parser::parse_monkey_list(input).unwrap();
    let mut num_inspected = vec![0; monkeys.len()];
    dbg!(i);
    dbg!(&monkeys);
    for _ in 0..20 {
        for monkey in &monkeys {
            println!("Monkey {}:", monkey.id_num);
            while let Some(item) = monkey.items.borrow_mut().pop_front() {
                num_inspected[monkey.id_num as usize] += 1;
                println!("  Monkey inspects an item with a worry level of {item}.");
                let level = (monkey.operation)(item);
                println!("  Worry level updated to {level}");
                let next = (monkey.test)(level);
                println!("  Item with worry level {level} is thrown to monkey {next}");
                monkeys[next as usize].items.borrow_mut().push_back(level);
            }
        }
    }

    dbg!(num_inspected);
}

pub struct Monkey {
    id_num: i64,
    items: RefCell<VecDeque<i64>>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> i64>,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("id_num", &self.id_num)
            .field("items", &self.items)
            .finish()
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.id_num == other.id_num && self.items == other.items
    }
}
impl Eq for Monkey {}

mod parser {
    use std::{cell::RefCell, collections::VecDeque};

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{self, line_ending},
        combinator::{all_consuming, map},
        multi::separated_list1,
        sequence::{preceded, terminated},
        IResult,
    };

    use crate::Monkey;

    pub fn parse_monkey_list(i: &str) -> IResult<&str, Vec<Monkey>> {
        all_consuming(separated_list1(line_ending, parse_monkey))(i)
    }

    fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
        let (i, id_num) = terminated(parse_monkey_id, line_ending)(i)?;
        let (i, items) = preceded(tag("  "), terminated(parse_items, line_ending))(i)?;
        let (i, operation) = preceded(tag("  "), terminated(parse_operation, line_ending))(i)?;
        let (i, test) = preceded(tag("  "), terminated(parse_test, line_ending))(i)?;
        Ok((
            i,
            Monkey {
                id_num,
                items: RefCell::new(items),
                operation,
                test,
            },
        ))
    }

    fn parse_test(i: &str) -> IResult<&str, Box<dyn Fn(i64) -> i64>> {
        let (i, test_op) = terminated(
            preceded(tag("Test: divisible by "), complete::i64),
            line_ending,
        )(i)?;
        let (i, true_ret) = terminated(
            preceded(tag("    If true: throw to monkey "), complete::i64),
            line_ending,
        )(i)?;
        let (i, false_ret) = preceded(tag("    If false: throw to monkey "), complete::i64)(i)?;

        Ok((
            i,
            Box::new(move |new| {
                if new % test_op == 0 {
                    true_ret
                } else {
                    false_ret
                }
            }),
        ))
    }

    fn parse_operation(i: &str) -> IResult<&str, Box<dyn Fn(i64) -> i64>> {
        preceded(
            tag("Operation: new = old "),
            alt((
                map(preceded(tag("* "), complete::i64), |v| {
                    Box::new(move |old| (old * v) / 3_i64) as Box<dyn Fn(i64) -> i64>
                }),
                map(preceded(tag("+ "), complete::i64), |v| {
                    Box::new(move |old| (old + v) / 3_i64) as _
                }),
                map(tag("* old"), |_| Box::new(|old| (old * old) / 3_i64) as _),
            )),
        )(i)
    }

    fn parse_monkey_id(i: &str) -> IResult<&str, i64> {
        preceded(tag("Monkey "), terminated(complete::i64, tag(":")))(i)
    }

    fn parse_items(i: &str) -> IResult<&str, VecDeque<i64>> {
        preceded(
            tag("Starting items: "),
            map(separated_list1(tag(", "), complete::i64), VecDeque::from),
        )(i)
    }

    #[cfg(test)]
    mod test {
        use std::{cell::RefCell, collections::VecDeque};

        use super::{parse_items, parse_monkey, parse_monkey_id};
        use crate::{parser::parse_monkey_list, Monkey};
        use test_case::test_case;

        #[test_case("Starting items: 69", VecDeque::from(vec![69]))]
        #[test_case("Starting items: 61, 94, 85, 52, 81, 90, 94, 70", VecDeque::from(vec![61, 94, 85, 52, 81, 90, 94, 70]))]
        fn test_parse_starting_items(input: &str, expected: VecDeque<i64>) {
            let (_, actual) = parse_items(input).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_parse_monkey_id() {
            let input = "Monkey 0:";
            let expected = 0;
            let (_, actual) = parse_monkey_id(input).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_parse_monkey() {
            let input = "Monkey 1:
  Starting items: 69, 99, 95, 62
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 5
";
            let expected = Monkey {
                id_num: 1,
                items: RefCell::new(VecDeque::from(vec![69, 99, 95, 62])),
                operation: Box::new(|old| (old * old) / 3),
                test: Box::new(|new| if new % 17 == 0 { 2 } else { 5 }),
            };
            let (_i, actual) = parse_monkey(input).unwrap();

            assert_eq!(expected, actual);
            assert_eq!(
                expected
                    .items
                    .borrow()
                    .iter()
                    .map(|v| (expected.operation)(*v))
                    .collect::<Vec<_>>(),
                actual
                    .items
                    .borrow()
                    .iter()
                    .map(|v| (actual.operation)(*v))
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                expected
                    .items
                    .clone()
                    .borrow()
                    .iter()
                    .map(|v| (expected.operation)(*v))
                    .map(|v| (expected.test)(v))
                    .collect::<Vec<_>>(),
                actual
                    .items
                    .clone()
                    .borrow()
                    .iter()
                    .map(|v| (actual.operation)(*v))
                    .map(|v| (actual.test)(v))
                    .collect::<Vec<_>>()
            )
        }

        #[test]
        fn test_parse_monkey_list() {
            let input = "Monkey 0:
  Starting items: 74, 64, 74, 63, 53
  Operation: new = old * 7
  Test: divisible by 5
    If true: throw to monkey 1
    If false: throw to monkey 6

Monkey 1:
  Starting items: 69, 99, 95, 62
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 5
";
            let expected = vec![
                Monkey {
                    id_num: 0,
                    items: RefCell::new(VecDeque::from(vec![74, 64, 74, 63, 53])),
                    operation: Box::new(|old| (old * 7) / 3),
                    test: Box::new(|new| if new % 5 == 0 { 1 } else { 6 }),
                },
                Monkey {
                    id_num: 1,
                    items: RefCell::new(VecDeque::from(vec![69, 99, 95, 62])),
                    operation: Box::new(|old| (old * old) / 3),
                    test: Box::new(|new| if new % 17 == 0 { 2 } else { 5 }),
                },
            ];
            let (_i, actual) = parse_monkey_list(input).unwrap();

            assert_eq!(expected, actual);
        }
    }
}
