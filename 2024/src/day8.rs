use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq,)]
pub struct PointPair(pub Point, pub Point);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: i64,
    y: i64
}

impl Point {
    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }
}

impl From<(RowIdx, ColIdx)> for Point {
    fn from(source: (RowIdx, ColIdx)) -> Self {
        Self {
            x: source.1.0,
            y: source.0.0,
        }
    }
}

impl From<(X, Y)> for Point {
    fn from(value: (X, Y)) -> Self {
        Self {
            x: value.0.0.round() as i64,
            y: value.1.0.round() as i64,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct X(f64);
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Y(f64);

impl Sub for X {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Mul<f64> for X {
    type Output = f64;
    
    fn mul(self, rhs: f64) -> Self::Output {
        self.0 * rhs
    }
}

impl Sub for Y {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Add<f64> for Y {
    type Output = f64;
    
    fn add(self, rhs: f64) -> Self::Output {
        self.0 + rhs
    }
}

impl PointPair {
    /// give two antenna of the same variant they create antinodes
    /// at locations along the line the create, on the outside of the
    /// antenna pair, each antinode at a distance equal to the distance
    /// between the antenna pair.
    /// 
    /// to find antinodes we can find the line defined by each antenna point
    /// find the distance (*d*) between the antenna pair
    /// for each antenna, draw a circle with radius *d* around the point
    /// and where that circle intersects the line is where there's an antinode
    pub fn antinodes(&self, grid_size: (i64, i64)) -> Vec<Point> {
        let (x_0, y_0) = (X(self.0.x as f64), Y(self.0.y as f64));
        let (x_1, y_1) = (X(self.1.x as f64), Y(self.1.y as f64));

        let d = ((x_1 - x_0).powi(2) + (y_1 - y_0).powi(2)).sqrt();

        if self.0.x == self.1.x {
            return (1..).map(|mult| {
                Point {
                    x: self.0.x,
                    y: self.0.y.max(self.1.y) - ((d as i64) * mult)
                }
            })
            .take_while(|point| point.x() >= 0 && point.y() >= 0 && point.x() < grid_size.0 && point.y() < grid_size.1)
            .inspect(|p| {
                println!("{self:?} {p:?}");
            })
            .chain((1..).map(|mult| {
                Point {
                    x: self.0.x,
                    y: self.0.y.min(self.1.y) + ((d as i64) * mult)
                }
            })
            .take_while(|point| point.x() >= 0 && point.y() >= 0 && point.x() < grid_size.0 && point.y() < grid_size.1))
            .inspect(|p| {
                println!("{self:?} {p:?}");
            })
            .collect() 
        // special case horizontal line just because it's easy
        } else if self.0.y == self.1.y {
            return (1..).map(|mult| {
                Point {
                    x: self.0.x.max(self.1.x) - ((d as i64) * mult),
                    y: self.0.y
                }
            })
            .take_while(|point| point.x() >= 0 && point.y() >= 0 && point.x() < grid_size.0 && point.y() < grid_size.1)
            .inspect(|p| {
                println!("{self:?} {p:?}");
            })
            .chain((1..).map(|mult| {
                Point {
                    x: self.0.x.min(self.1.x) + ((d as i64) * mult),
                    y: self.0.y
                }
            })
            .take_while(|point| point.x() >= 0 && point.y() >= 0 && point.x() < grid_size.0 && point.y() < grid_size.1))
            .inspect(|p| {
                println!("{self:?} {p:?}");
            })
            .collect();
        }

        let slope = (y_1 - y_0) / (x_1 - x_0);
        let y_for_x_on_line = |x: X| Y(y_0 + (slope * (x - x_0)));

        let xs_d_from_pt = |d: f64, x0: X| {
            let offset = d / (1.0 + slope.powi(2)).sqrt();
            (X(x0.0 - offset), X(x0.0 + offset))
        };

        let (lesser, greater) = if x_0 < x_1 { 
            (x_0, x_1)
        } else {
             (x_1, x_0)
        };

        (1..).map(|mult| {
            let right = xs_d_from_pt(d * (mult as f64), lesser).1;
            Point {
                x: right.0 as i64,
                y: y_for_x_on_line(right).0 as i64
            }
        })
        .take_while(|point| point.x() >= 0 && point.y() >= 0 && point.x() < grid_size.0 && point.y() < grid_size.1)
        .inspect(|p| {
                println!("{self:?} {p:?}");
            })
        .chain((1..).map(|mult| {
            let left = xs_d_from_pt(d * (mult as f64), greater).0;
            Point {
                x: left.0 as i64,
                y: y_for_x_on_line(left).0 as i64
            }
        })
        .take_while(|point| point.x() >= 0 && point.y() >= 0 && point.x() < grid_size.0 && point.y() < grid_size.1))
        .inspect(|p| {
                println!("{self:?} {p:?}");
            })
        .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RowIdx(i64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ColIdx(i64);

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::{PointPair, Point, ColIdx, RowIdx};

    #[test_case(
        PointPair(
            (RowIdx(3), ColIdx(5)).into(),
            (RowIdx(5), ColIdx(5)).into()
        ),
        [(RowIdx(1), ColIdx(5)).into(), (RowIdx(7), ColIdx(5)).into()]
    )]
    #[test_case(
        PointPair(
            (RowIdx(5), ColIdx(4)).into(),
            (RowIdx(5), ColIdx(5)).into()
        ),
        [(RowIdx(5), ColIdx(3)).into(), (RowIdx(5), ColIdx(6)).into()]
    )]
    #[test_case(
        PointPair(
            (RowIdx(3), ColIdx(4)).into(),
            (RowIdx(5), ColIdx(5)).into(),
        ),
        [(RowIdx(1), ColIdx(3)).into(), (RowIdx(7), ColIdx(6)).into()]
    )]
    #[test_case(
        PointPair(
            (RowIdx(4), ColIdx(8)).into(),
            (RowIdx(5), ColIdx(5)).into(),
        ),
        [(RowIdx(3), ColIdx(11)).into(), (RowIdx(6), ColIdx(2)).into()]
    )]
    fn test_antinodes(pair: PointPair, expected: [Point;2 ]) {
        assert_eq!(pair.antinodes((12, 12)), expected);
    }
}


pub mod parser {
    use std::collections::{HashMap, HashSet};
    use super::{RowIdx, ColIdx, Point};

    pub fn parse(input: &str) -> HashMap<char, HashSet<Point>> {
        input.lines().enumerate()
            .map(|(idx, line)| (RowIdx(idx as i64), line))
            .fold(HashMap::new(), |mut map, (row_idx, row)| {
                for (col_idx, value) in row.chars().enumerate()
                    .filter(|(_, c)| c.is_alphanumeric()) {
                        let set = map.entry(value).or_default();
                        set.insert((row_idx, ColIdx(col_idx as i64)).into());
                    };
                map
            })
    }

    #[cfg(test)]
    mod test {
        use core::assert_eq;
        use std::collections::{HashMap, HashSet};

        use crate::day8::{Point, ColIdx, RowIdx};

        #[test]
        fn test_sample_input() {
            let input = r#"
   012345678901
0  .........#..
1  ##....#.0...
2  .#.#.0..#...
3  ..#..#.0..#.
4  ..##0.......
5  ....#.A..#..
6  .#...##....#
7  ...#..#.....
8  #....#.#A...
9 ...#.....#..
10 ....#....#..
11 .#........#."#;
            let expected = HashMap::<char, HashSet<Point>>::from([
                ('0', HashSet::from([
                    (RowIdx(1), ColIdx(8)).into(),
                    (RowIdx(2), ColIdx(5)).into(),
                    (RowIdx(3), ColIdx(7)).into(),
                    (RowIdx(4), ColIdx(4)).into(),
                ])),
                ('A', HashSet::from([
                    (RowIdx(5), ColIdx(6)).into(),
                    (RowIdx(8), ColIdx(8)).into(),
                    (RowIdx(9), ColIdx(9)).into(),
                ]))
            ]);

            let actual = super::parse(input);
            assert_eq!(actual, expected);
        }
        
    }
}