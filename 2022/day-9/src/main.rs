use itertools::Itertools;
use lending_iterator::{lending_iterator::constructors::windows_mut, prelude::*};
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
    dim: (usize, usize),
    knot_pos: Vec<Point>,
    trail: HashSet<Point>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            // i don't want to do the math to calculate this from the input
            dim: (121, 121),
            knot_pos: vec![Point(0, 0); 10],
            trail: HashSet::from([Point(0, 0)]),
        }
    }

    pub fn step(&mut self, step_move: Move) {
        use Move::*;
        for step in step_move.iter() {
            self.knot_pos[0] += &step;

            let mut i = 0;
            let last_idx = self.knot_pos.len() - 2;
            let mut iter = self.knot_pos.windows_mut::<2>();

            while let Some([ref mut k_1, ref mut k_2]) = iter.next() {
                if k_2.distance_to(&k_1) > std::f64::consts::SQRT_2 {
                    // there's no else case! if distance is 0 in this direction
                    // then no move
                    
                    if k_1.0 > k_2.0 {
                        *k_2 += &Right(1)
                    } else if k_1.0 < k_2.0 {
                        *k_2 += &Left(1)
                    }

                    if k_1.1 > k_2.1 {
                        *k_2 += &Up(1);
                    } else if k_1.1 < k_2.1 {
                        *k_2 += &Down(1);
                    }
                    if i == last_idx {
                        self.trail.insert(*k_2);
                    }
                }
                i += 1;
            }
            
            
            #[cfg(debug_assertions)]
            {
                println!("{}", self);
                let mut answer = String::new();
                let _ = std::io::stdin().read_line(&mut answer);
            }
        }

    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const OFFSET_X: usize = 61;
        const OFFSET_Y: usize = 10;
        let mut grid = vec![vec![std::borrow::Cow::Borrowed("."); self.dim.1]; self.dim.0];

        for (i, knot) in self.knot_pos.iter().enumerate().rev() {
            let stamp = if i == 0 {
                std::borrow::Cow::Borrowed("H")
            } else {
                std::borrow::Cow::Owned(i.to_string())
            };

            grid[(OFFSET_Y as i64 - knot.1) as usize][(OFFSET_X as i64 + knot.0) as usize] =
                stamp;
        }

        for trail_pt in self.trail.iter() {
            let current = &mut grid[(OFFSET_Y as i64 - trail_pt.1) as usize][(OFFSET_X as i64 + trail_pt.0) as usize];
            if !current.chars().any(|c| c.is_alphanumeric()) {
                *current = std::borrow::Cow::Borrowed("#");
            }
        }
        
        let grid = grid.iter().map(|l| l.join("")).join("\n");
        write!(f, "\n{}", grid)
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
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        let grid = input
            .lines()
            .map(Move::from)
            .inspect(|m| println!("== {m:?} =="))
            .fold(Grid::new(), |mut grid, next_move| {
                grid.step(next_move);
                grid
            });

        assert_eq!(36, grid.trail.len());
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
