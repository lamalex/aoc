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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct EdgeMeta {
    max: i64,
    min: i64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid {
    dim: Point<EdgeMeta>,
    knot_pos: Vec<Point<i64>>,
    trail: HashSet<Point<i64>>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            dim: Point::default(),
            knot_pos: vec![Point::default(); 10],
            trail: HashSet::from([Point::default()]),
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
                    if k_1.x > k_2.x {
                        *k_2 += &Right(1)
                    } else if k_1.x < k_2.x {
                        *k_2 += &Left(1)
                    }

                    if k_1.y > k_2.y {
                        *k_2 += &Up(1);
                    } else if k_1.y < k_2.y {
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

            self.dim.update_boundary(self.knot_pos[0]);
        }

    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        println!("x len {} y len {}", self.dim.x.len(), self.dim.y.len());
        let mut grid = vec![vec![std::borrow::Cow::Borrowed("."); self.dim.y.len()]; self.dim.x.len()];
        let offset_x: usize = self.dim.x.min.abs() as usize;
        let offset_y: usize = self.dim.y.max as usize;

        for (i, knot) in self.knot_pos.iter().enumerate().rev() {
            let stamp = if i == 0 {
                std::borrow::Cow::Borrowed("H")
            } else {
                std::borrow::Cow::Owned(i.to_string())
            };

            grid[(offset_y as i64 - knot.y) as usize][(offset_x as i64 + knot.x) as usize] =
                stamp;
        }

        for trail_pt in self.trail.iter() {
            let current = &mut grid[(offset_y as i64 - trail_pt.y) as usize][(offset_x as i64 + trail_pt.x) as usize];
            if !current.chars().any(|c| c.is_alphanumeric()) {
                *current = std::borrow::Cow::Borrowed("#");
            }
        }
        
        let grid = grid.iter().map(|l| l.join("")).join("\n");
        write!(f, "\n{}", grid)
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
struct Point<T> { x: T, y: T }

impl Point<i64> {
    fn distance_to(&self, other: &Self) -> f64 {
        f64::sqrt((self.x - other.x).pow(2) as f64 + (self.y - other.y).pow(2) as f64)
    }
}

impl Point<EdgeMeta> {
    fn update_boundary(&mut self, point: Point<i64>) {
        self.x.min = self.x.min.min(point.x);
        self.x.max = self.x.max.max(point.x);
        self.y.min = self.y.min.min(point.y);
        self.y.max = self.y.max.max(point.y);
    }
}

impl EdgeMeta {
    fn len(&self) -> usize {
        (self.max - self.min).abs() as usize
    }
}

impl AddAssign<&Move> for Point<i64> {
    fn add_assign(&mut self, rhs: &Move) {
        use Move::*;

        let next_pos = match rhs {
            Up(val) => Point { x: self.x, y: self.y.saturating_add(*val as i64) },
            Down(val) => Point { x: self.x, y: self.y.saturating_sub(*val as i64) },
            Left(val) => Point { x: self.x.saturating_sub(*val as i64), y: self.y },
            Right(val) => Point { x: self.x.saturating_add(*val as i64), y: self.y },
         };
        self.x = next_pos.x;
        self.y = next_pos.y;
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

        dbg!(&grid);
        assert_eq!(36, grid.trail.len());
    }

    #[test]
    fn test_distance_fn_lateral() {
        assert_eq!(1., Point::default().distance_to(&Point { x: 0, y: 1 }))
    }

    #[test]
    fn test_distance_fn_diag() {
        assert_eq!(
            std::f64::consts::SQRT_2,
            Point::default().distance_to(&Point { x: 1, y: 1})
        )
    }
}
