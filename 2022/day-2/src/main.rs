use std::{cmp::Ordering, convert::Infallible, str::FromStr};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Move::Rock => match other {
                Move::Rock => Ordering::Equal,
                Move::Paper => Ordering::Less,
                Move::Scissors => Ordering::Greater,
            },
            Move::Paper => match other {
                Move::Rock => Ordering::Greater,
                Move::Paper => Ordering::Equal,
                Move::Scissors => Ordering::Less,
            },
            Move::Scissors => match other {
                Move::Rock => Ordering::Less,
                Move::Paper => Ordering::Greater,
                Move::Scissors => Ordering::Equal,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct TheirMove(Move);

impl FromStr for TheirMove {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(match s {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("{}, their ruh-roh", s),
        }))
    }
}

#[derive(Debug, Clone)]
struct MyMove(Move);

impl FromStr for MyMove {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(match s {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("{} my ruh-roh", s),
        }))
    }
}

impl std::ops::Add<TheirMove> for MyMove {
    type Output = u64;

    fn add(self, rhs: TheirMove) -> Self::Output {
        let move_score = match self.0 {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let game_score = match self.0.cmp(&rhs.0) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
        
        move_score + game_score
    }
}

fn main() {
    let data = include_str!("input.txt");
    let moves = data
        .split('\n')
        .map(|line| {
            let mut moves = line.split(' ');
            
            (
                moves.next().unwrap().trim().parse().unwrap(),
                moves.next().unwrap().trim().parse().unwrap(),
            )
        })
        .fold(0, |acc, n: (TheirMove, MyMove)| acc + (n.1 + n.0));

    println!("{:?}", moves);
}
