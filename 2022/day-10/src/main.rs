use downcast_rs::{impl_downcast, Downcast};
use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
enum Poll {
    Pending,
    Ready,
}

trait Instruction: Debug + Downcast {
    fn execute(&mut self, register: &mut Register) -> Poll;
}
impl_downcast!(Instruction);

#[derive(Debug, PartialEq, Eq)]
struct Noop;
impl Instruction for Noop {
    fn execute(&mut self, _register: &mut Register) -> Poll {
        Poll::Ready
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Addx {
    x: i64,
    ticks: usize,
}

impl Addx {
    fn new(x: i64) -> Self {
        Self { x, ticks: 0 }
    }
}
impl Instruction for Addx {
    fn execute(&mut self, mut register: &mut Register) -> Poll {
        self.ticks += 1;
        if self.ticks == 1 {
            Poll::Pending
        } else if self.ticks == 2 {
            register.0 += self.x;
            Poll::Ready
        } else {
            panic!("execute called too many times");
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Register(i64);

struct Cpu {
    register: Register,
    instruction: Option<Box<dyn Instruction>>,
}

impl Cpu {
    #[must_use]
    fn new() -> Self {
        Self {
            register: Register(1),
            instruction: None,
        }
    }

    fn load_instruction(&mut self, instruction: Box<dyn Instruction>) {
        self.instruction = Some(instruction);
    }

    fn execute_instruction(&mut self) -> Poll {
        let result = self
            .instruction
            .as_mut()
            .map(|instruction| instruction.execute(&mut self.register))
            .unwrap_or(Poll::Ready);
        if result == Poll::Ready {
            self.instruction = None;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::{Addx, Instruction, Noop, Poll, Register};

    #[test]
    fn test_addx_mutate_register() {
        let mut register = Register(0);
        let mut instruction = Addx::new(5);
        let result = instruction.execute(&mut register);
        assert_eq!(Poll::Pending, result);
        let result = instruction.execute(&mut register);
        assert_eq!(Poll::Ready, result);
        assert_eq!(Register(5), register);
    }

    #[test]
    fn test_noop_does_nothing() {
        let mut register = Register(0);
        let mut instruction = Noop;

        let result = instruction.execute(&mut register);
        assert_eq!(Poll::Ready, result);
        assert_eq!(Register(0), register);
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{self, newline},
        combinator::map,
        multi::separated_list0,
        sequence::preceded,
        IResult,
    };

    use crate::{Addx, Instruction, Noop};

    fn parse_instruction_list(i: &str) -> IResult<&str, Vec<Box<dyn Instruction>>> {
        separated_list0(newline, parse_instruction)(i)
    }

    fn parse_instruction(i: &str) -> IResult<&str, Box<dyn Instruction>> {
        alt((
            map(parse_noop, |noop| Box::new(noop) as _),
            map(parse_addx, |addx| Box::new(addx) as _),
        ))(i)
    }

    fn parse_noop(i: &str) -> IResult<&str, Noop> {
        map(tag("noop"), |_| Noop)(i)
    }

    fn parse_addx(i: &str) -> IResult<&str, Addx> {
        map(preceded(tag("addx "), complete::i64), |i| Addx::new(i))(i)
    }

    #[cfg(test)]
    mod test {
        use super::{parse_instruction, parse_instruction_list, parse_noop};
        use crate::{parser::parse_addx, Addx, Cpu, Instruction, Noop, Poll};
        use std::fmt::Debug;
        use test_case::test_case;

        #[test]
        fn test_calculate_signal_strength() {
            let input = include_str!("input.txt");
            let (_, instruction_list) = parse_instruction_list(&input).unwrap();

            let mut cycles = 0;
            let mut cpu = Cpu::new();

            let mut signal_sum = 0;

            for instruction in instruction_list {
                cpu.load_instruction(instruction);
                loop {
                    cycles += 1;
                    if cycles == 20 {
                        signal_sum += 20 * cpu.register.0;
                    } else if cycles == 60 {
                        signal_sum += 60 * cpu.register.0;
                    } else if cycles == 100 {
                        signal_sum += 100 * cpu.register.0;
                    } else if cycles == 140 {
                        signal_sum += 140 * cpu.register.0;
                    } else if cycles == 180 {
                        signal_sum += 180 * cpu.register.0;
                    } else if cycles == 220 {
                        signal_sum += 220 * cpu.register.0;
                    }
                    if cpu.execute_instruction() == Poll::Ready {
                        break;
                    }
                }
            }

            assert_eq!(13140, signal_sum);
        }

        #[test]
        fn test_parse_instruction_list() {
            let input = "noop
addx 3
addx -5";
            let expected_0 = Noop;
            let expected_1 = Addx::new(3);
            let expected_2 = Addx::new(-5);

            let (_, actual) = parse_instruction_list(&input).unwrap();

            assert_eq!(Some(&expected_0), actual[0].downcast_ref::<Noop>());
            assert_eq!(Some(&expected_1), actual[1].downcast_ref::<Addx>());
            assert_eq!(Some(&expected_2), actual[2].downcast_ref::<Addx>());
        }

        #[test_case("addx 69", Addx::new(69))]
        fn test_parse_instruction<T>(i: &str, expected: T)
        where
            T: Instruction + PartialEq + Eq + Debug,
        {
            let (_, actual) = parse_instruction(i).unwrap();
            let actual = actual.downcast_ref::<T>();

            assert_eq!(Some(&expected), actual);
        }

        #[test]
        fn test_parse_noop() {
            let input = "noop";
            let expected = Noop;

            let (_, actual) = parse_noop(input).unwrap();

            assert_eq!(expected, actual);
        }

        #[test_case(-10)]
        #[test_case(5)]
        #[test_case(69)]
        fn test_parse_addx(v: i64) {
            let input = format!("addx {v}");
            let expected = Addx::new(v);

            let (_, actual) = parse_addx(&input).unwrap();

            assert_eq!(expected, actual);
        }
    }
}
