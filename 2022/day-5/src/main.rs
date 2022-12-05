use std::collections::VecDeque;

use itertools::Itertools;
use types::{CrateId, MoveInstruction};
fn main() {
    let input = include_str!("input.txt");
    let mut stacks: [VecDeque<CrateId>; 9] = [
        VecDeque::default(),
        VecDeque::default(),
        VecDeque::default(),
        VecDeque::default(),
        VecDeque::default(),
        VecDeque::default(),
        VecDeque::default(),
        VecDeque::default(),
        VecDeque::default(),
    ];

    let mut parsing_stack: bool = true;
    for line in input.lines() {
        if parsing_stack && line.chars().nth(1).unwrap().is_ascii_digit() {
            parsing_stack = false;
            println!("{:?}", &stacks);
            continue;
        }

        if parsing_stack {
            for (i, crate_str) in line
                .chars()
                .into_iter()
                .chunks(4)
                .into_iter()
                .map(|mut chunk| chunk.join(""))
                .enumerate()
            {
                if let Some(c) = crate_str.chars().nth(1) {
                    if c.is_ascii_alphanumeric() {
                        stacks[i].push_back(c.into())
                    }
                }
            }

            continue;
        }

        let instruction: MoveInstruction = line.into();
        println!("{instruction}");
        for _ in 0..instruction.n {
            stacks[instruction.from - 1]
                .pop_front()
                .into_iter()
                .inspect(|c| println!("{c}"))
                .map(|v| stacks[instruction.to - 1].push_front(v))
                .next();
        }
    }

    for s in stacks.iter() {
        if let Some(c) = s.iter().nth(0) {
            print!("{c}");
        }
    }

    println!("");
}

mod types {
    #[derive(Debug, Clone)]
    pub struct MoveInstruction {
        pub n: usize,
        pub to: usize,
        pub from: usize,
    }

    impl std::fmt::Display for MoveInstruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "move {} from {} to {}", self.n, self.from, self.to)
        }
    }

    impl From<&str> for MoveInstruction {
        fn from(value: &str) -> Self {
            let mut parts = value.split(char::is_whitespace);

            parts.next();
            let n = parts.next().unwrap().parse().unwrap();
            parts.next();
            let from = parts.next().unwrap().parse().unwrap();
            parts.next();
            let to = parts.next().unwrap().parse().unwrap();
            Self { n, to, from }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct CrateId(char);

    impl From<char> for CrateId {
        fn from(value: char) -> Self {
            match value {
                'A'..='Z' => Self(value),
                _ => unreachable!("I know this can't happen"),
            }
        }
    }

    impl std::fmt::Display for CrateId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    #[test]
    fn chunk_chars() {
        let s = "[T]     [Q]             [S]        ";
        let strings: Vec<String> = s
            .chars()
            .into_iter()
            .chunks(4)
            .into_iter()
            .map(|mut chunk| chunk.join(""))
            .collect();
        assert_eq!(Vec::<String>::default(), strings);
    }
}
