use itertools::Itertools;
use std::{collections::HashSet, ops::AddAssign};

fn main() {
    let input = include_str!("input.txt");

    let grid = input
        .lines()
        .map(Move::from)
        .fold(Grid::new(), |mut grid, next_move| {
            grid.step(next_move);
            grid
        });

    println!("{}", grid.trail.len());
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid {
    h_pos: Point,
    t_pos: Point,
    trail: HashSet<Point>,
}

impl Grid {
    pub fn step(&mut self, step_move: Move) {
        use Move::*;
        println!("{step_move:?}");
        for step in step_move.iter() {
            self.h_pos += &step;

            if self.t_pos.distance_to(&self.h_pos) > std::f64::consts::SQRT_2 {
                // there's no else case! if distance is 0 in this direction
                // then no move
                if self.h_pos.0 > self.t_pos.0 {
                    self.t_pos += &Right(1)
                } else if self.h_pos.0 < self.t_pos.0 {
                    self.t_pos += &Left(1)
                }

                if self.h_pos.1 > self.t_pos.1 {
                    self.t_pos += &Up(1);
                } else if self.h_pos.1 < self.t_pos.1 {
                    self.t_pos += &Down(1);
                }
                println!("moved t_pos {:?}", self.t_pos);
                self.trail.insert(self.t_pos);
            }
        }
    }
}

impl Grid {
    pub fn new() -> Self {
        Self {
            h_pos: Point(0, 0),
            t_pos: Point(0, 0),
            trail: HashSet::from([Point(0, 0)]),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(i64, i64);

impl Point {
    fn distance_to(&self, other: &Self) -> f64 {
        f64::sqrt((self.0 - other.0).pow(2) as f64 + (self.1 - other.1).pow(2) as f64)
    }
}

impl AddAssign<&Move> for Point {
    fn add_assign(&mut self, rhs: &Move) {
        use Move::*;

        let next_pos = match rhs {
            Up(val) => Point(self.0, self.1.saturating_add(*val as i64)),
            Down(val) => Point(self.0, self.1.saturating_sub(*val as i64)),
            Left(val) => Point(self.0.saturating_sub(*val as i64), self.1),
            Right(val) => Point(self.0.saturating_add(*val as i64), self.1),
        };

        self.0 = next_pos.0;
        self.1 = next_pos.1;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Move {
    fn iter(&self) -> impl Iterator<Item = Move> + '_ {
        use Move::*;

        let moves = match self {
            Up(d) | Down(d) | Left(d) | Right(d) => d.clone(),
        };

        (0..moves).into_iter().map(move |_| match self {
            Up(_) => Up(1),
            Down(_) => Down(1),
            Left(_) => Left(1),
            Right(_) => Right(1),
        })
    }
}

impl From<&str> for Move {
    fn from(cmd: &str) -> Self {
        let (dir, val): (&str, &str) = cmd.split(' ').take(2).collect_tuple().unwrap();
        let val: usize = val.parse().unwrap();

        match dir {
            "U" => Self::Up(val),
            "D" => Self::Down(val),
            "L" => Self::Left(val),
            "R" => Self::Right(val),
            _ => panic!("unexpected move direction {}", dir),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Grid, Move, Point};
    use test_case::test_case;

    #[test_case("U 1", Move::Up(1))]
    #[test_case("D 5", Move::Down(5))]
    #[test_case("L 2", Move::Left(2))]
    #[test_case("R 9", Move::Right(9))]
    fn from_str_to_move(input: &str, expected: Move) {
        let actual = Move::from(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_trail_compute() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let grid = input
            .lines()
            .map(Move::from)
            .fold(Grid::new(), |mut grid, next_move| {
                grid.step(next_move);
                grid
            });

        assert_eq!(13, grid.trail.len());
    }

    #[test]
    fn test_distance_fn_lateral() {
        assert_eq!(1., Point(0, 0).distance_to(&Point(0, 1)))
    }

    #[test]
    fn test_distance_fn_diag() {
        assert_eq!(
            std::f64::consts::SQRT_2,
            Point(0, 0).distance_to(&Point(1, 1))
        )
    }
}
