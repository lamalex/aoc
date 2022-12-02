#[derive(Copy, Debug, Clone, PartialEq, Eq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    const fn beats(&self) -> Self {
        match *self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    const fn loses_to(&self) -> Self {
        match *self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn vs(&self, other: &Self) -> Outcome {
        if self.beats() == *other {
            Outcome::Win
        } else if self.loses_to() == *other {
            Outcome::Loss
        }else {
            Outcome::Draw
        }
    }
}

#[derive(Copy, Debug, Clone)]
struct TheirMove(Move);

impl From<&str> for TheirMove {
    fn from(s: &str) -> Self {
        Self(match s {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("{}, their ruh-roh", s),
        })
    }
}

#[derive(Copy, Debug, Clone)]
struct MyMove(Move);

impl From<&str> for MyMove {
    fn from(s: &str) -> Self {
        Self(match s {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("{} my ruh-roh", s),
        })
    }
}

impl MyMove {
    fn score(&self) -> u64 {
        self.0 as u64
    }
}

impl std::ops::Add<TheirMove> for MyMove {
    type Output = u64;

    fn add(self, rhs: TheirMove) -> Self::Output {
        let outcome = self.0.vs(&rhs.0);

        self.score() + outcome.score()
    }
}

impl From<(TheirMove, Outcome)> for MyMove {
    fn from(value: (TheirMove, Outcome)) -> Self {
        let their_move = value.0.0;
        
        Self(match value.1 {
            Outcome::Win => their_move.loses_to(),
            Outcome::Loss => their_move.beats(),
            Outcome::Draw => their_move,
        })
    }
}

#[derive(Copy, Debug, Clone, PartialOrd, PartialEq, Eq)]
enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl Outcome {
    fn score(&self) -> u64 {
        *self as u64
    }
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("{} outcome ruh-roh", s),
        }
    }
}

fn main() {
    let parse_from_their_my = |a: &str, b: &str| {
        (
            TheirMove::from(a),
            MyMove::from(b)
        )
    };

    let parse_from_their_outcome = |a: &str, b: &str| {
        let their_move = TheirMove::from(a);

        (their_move, MyMove::from((their_move, Outcome::from(b))))
    };

    let output: (u64, u64) = include_str!("input.txt")
        .split('\n')
        .map(|line| {
            let mut pair = line.split(' ').take(2);
            let pair = (pair.next().unwrap(), pair.next().unwrap());

            (
                parse_from_their_my(pair.0, pair.1),
                parse_from_their_outcome(pair.0, pair.1),
            )
        })
        .fold((0, 0), |acc, next| {
            let pt1_sum = acc.0;
            let pt1_next = next.0;

            let pt2_sum = acc.1;
            let pt2_next = next.1;

            (
                pt1_sum + (pt1_next.1 + pt1_next.0),
                pt2_sum + (pt2_next.1 + pt2_next.0),
            )
        });
    println!("{output:?}");
}
