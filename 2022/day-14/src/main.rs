use std::{
    convert::Infallible,
    fmt,
    ops::{Add, Index, IndexMut},
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");
    let mut grid: Grid = input.parse().unwrap();
    println!("{}", grid.sand_fill());
    println!("{grid}");
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct Point {
    x: u64,
    y: u64,
}

impl From<(u64, u64)> for Point {
    fn from((x, y): (u64, u64)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Offset {
    x: i64,
    y: i64,
}

impl Add<&Offset> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: &Offset) -> Self::Output {
        self.x
            .checked_add_signed(rhs.x)
            .into_iter()
            .zip(self.y.checked_add_signed(rhs.y).into_iter())
            .map(Point::from)
            .next()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Object {
    Air,
    Rock,
    Sand,
    Start,
}

impl Object {
    fn to_str(&self) -> &'static str {
        match self {
            Object::Air => ".",
            Object::Rock => "#",
            Object::Sand => "o",
            Object::Start => "+",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    minx: usize,
    inner: Vec<Vec<Object>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .enumerate()
                .map(|(i, row)| format!(
                    "{:03} {}",
                    i,
                    row[(self.minx - 1)..]
                        .iter()
                        .map(|c| c.to_str())
                        .collect::<Vec<_>>()
                        .join("")
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rock_points) = parser::parse_to_rock_points(s).unwrap();
        let [minx, maxx, maxy]: [usize; 3] =
            rock_points
                .iter()
                .fold([usize::MAX, usize::MIN, usize::MIN], |acc, p| {
                    [
                        acc[0].min(p.x as usize),
                        acc[1].max(p.x as usize),
                        acc[2].max(p.y as usize),
                    ]
                });
        let mut grid = Self {
            minx,
            inner: vec![vec![Object::Air; maxx + 1]; maxy + 1],
        };

        grid[Point::from((500, 0))] = Object::Start;
        Ok(rock_points.into_iter().fold(grid, |mut grid, p| {
            grid[p] = Object::Rock;
            grid
        }))
    }
}

impl Grid {
    fn sand_fill(&mut self) -> usize {
        let mut i = 0;
        let moves = [
            Offset { x: 0, y: 1 },
            Offset { x: -1, y: 1 },
            Offset { x: 1, y: 1 },
        ];
        loop {
            let mut start = Point::from((500, 0));
            while let Some(next) = moves
                .iter()
                .filter_map(|mov| start + mov)
                .inspect(|pt| {
                    println!(
                        "{:?} is {:?}",
                        pt,
                        self.inner
                            .get(pt.y as usize)
                            .map(|row| row.get(pt.x as usize))
                    )
                })
                .find(|pt| {
                    self.inner
                        .get(pt.y as usize)
                        .iter()
                        .find_map(|row| row.get(pt.x as usize))
                        == Some(&Object::Air)
                })
            {
                start = next;
            }
            self[start] = Object::Sand;
            if start.y as usize + 1 >= self.inner.len() {
                break;
            }
            i += 1;

            #[cfg(debug_assertions)]
            {
                println!("{self}");
                println!("{i}");
                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer);
            }
        }

        i
    }
}

impl Index<Point> for Grid {
    type Output = Object;

    fn index(&self, index: Point) -> &Self::Output {
        &self.inner[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.inner[index.y as usize][index.x as usize]
    }
}

#[cfg(test)]
mod test {
    use crate::{Grid, Object, Point};

    #[test]
    fn test_count_drop_grains() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let mut grid: Grid = input.parse().unwrap();

        let grains = grid.sand_fill();
        println!("{grid}");
        assert_eq!(24, grains);
    }

    #[test]
    fn test_generate_grid() {
        let mut expected: Grid = Grid {
            minx: 494,
            inner: vec![vec![Object::Air; 504]; 10],
        };

        let rock_points = vec![
            Point::from((498, 4)),
            Point::from((498, 5)),
            Point::from((498, 6)),
            Point::from((496, 6)),
            Point::from((497, 6)),
            Point::from((498, 6)),
            Point::from((502, 4)),
            Point::from((503, 4)),
            Point::from((502, 4)),
            Point::from((502, 5)),
            Point::from((502, 6)),
            Point::from((502, 7)),
            Point::from((502, 8)),
            Point::from((502, 9)),
            Point::from((494, 9)),
            Point::from((495, 9)),
            Point::from((496, 9)),
            Point::from((497, 9)),
            Point::from((498, 9)),
            Point::from((499, 9)),
            Point::from((500, 9)),
            Point::from((501, 9)),
            Point::from((502, 9)),
        ];
        for point in rock_points {
            expected[point] = Object::Rock;
        }
        expected[Point::from((500, 0))] = Object::Start;

        println!("{expected}");
        let i = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let actual: Grid = i.parse().unwrap();
        println!("{actual}");
        assert_eq!(expected, actual);
    }
}

mod parser {
    use itertools::Itertools;
    use nom::{
        bytes::complete::tag,
        character::complete::{self, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    use crate::Point;

    pub fn parse_to_rock_points(i: &str) -> IResult<&str, Vec<Point>> {
        map(parse_list_of_lists_of_points, |list_of_lists| {
            list_of_lists
                .into_iter()
                .flat_map(|list| {
                    list.into_iter().tuple_windows().flat_map(|(p1, p2)| {
                        let x_range = p1.x.min(p2.x)..=p1.x.max(p2.x);
                        let y_range = p1.y.min(p2.y)..=p1.y.max(p2.y);
                        x_range
                            .into_iter()
                            .cartesian_product(y_range)
                            .map(Point::from)
                    })
                })
                .collect::<Vec<_>>()
        })(i)
    }

    fn parse_list_of_lists_of_points(i: &str) -> IResult<&str, Vec<Vec<Point>>> {
        separated_list1(line_ending, parse_list_of_points)(i)
    }

    fn parse_list_of_points(i: &str) -> IResult<&str, Vec<Point>> {
        separated_list1(tag(" -> "), parse_pair_to_point)(i)
    }

    fn parse_pair_to_point(i: &str) -> IResult<&str, Point> {
        map(
            separated_pair(complete::u64, tag(","), complete::u64),
            Point::from,
        )(i)
    }

    #[cfg(test)]
    mod test {
        use super::parse_pair_to_point;
        use crate::{
            parser::{parse_list_of_lists_of_points, parse_list_of_points, parse_to_rock_points},
            Point,
        };
        use test_case::test_case;

        #[test]
        fn test_parse_to_all_points() {
            let i = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

            let expected = vec![
                Point::from((498, 4)),
                Point::from((498, 5)),
                Point::from((498, 6)),
                Point::from((496, 6)),
                Point::from((497, 6)),
                Point::from((498, 6)),
                Point::from((502, 4)),
                Point::from((503, 4)),
                Point::from((502, 4)),
                Point::from((502, 5)),
                Point::from((502, 6)),
                Point::from((502, 7)),
                Point::from((502, 8)),
                Point::from((502, 9)),
                Point::from((494, 9)),
                Point::from((495, 9)),
                Point::from((496, 9)),
                Point::from((497, 9)),
                Point::from((498, 9)),
                Point::from((499, 9)),
                Point::from((500, 9)),
                Point::from((501, 9)),
                Point::from((502, 9)),
            ];

            let (_, actual) = parse_to_rock_points(i).unwrap();
            assert_eq!(expected, actual);
        }
        #[test]
        fn test_parse_list_of_lists_of_points() {
            let i = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

            let (_, actual) = parse_list_of_lists_of_points(i).unwrap();
            let expected = vec![
                vec![
                    Point::from((498, 4)),
                    Point::from((498, 6)),
                    Point::from((496, 6)),
                ],
                vec![
                    Point::from((503, 4)),
                    Point::from((502, 4)),
                    Point::from((502, 9)),
                    Point::from((494, 9)),
                ],
            ];

            assert_eq!(expected, actual);
        }

        #[test_case("498,4 -> 498,6 -> 496,6", vec![Point::from((498,4)), Point::from((498,6)), Point::from((496,6))])]
        #[test_case("503,4 -> 502,4 -> 502,9 -> 494,9", vec![Point::from((503,4)), Point::from((502,4)), Point::from((502,9)), Point::from((494,9))])]
        fn test_parse_rock_loc(i: &str, expected: Vec<Point>) {
            let (_, actual) = parse_list_of_points(i).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_parse_pair_to_point() {
            let input = "498,4";
            let (_, actual) = parse_pair_to_point(input).unwrap();

            assert_eq!(Point { x: 498, y: 4 }, actual);
        }
    }
}
