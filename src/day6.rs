use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowIdx(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColIdx(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    row_idx: RowIdx,
    col_idx: ColIdx,
}

impl<'a> Position {
    pub fn to(&'a self, end: &'a Self) -> PositionIterator<'a> {
        PositionIterator { curr: *self, end }
    }
}

impl From<(RowIdx, ColIdx)> for Position {
    fn from((row_idx, col_idx): (RowIdx, ColIdx)) -> Self {
        Self { row_idx, col_idx }
    }
}
pub struct PositionIterator<'a> {
    curr: Position,
    end: &'a Position,
}

impl<'a> Iterator for PositionIterator<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if &self.curr == self.end {
            return None;
        }

        // if we're moving along vertically (row-wise)
        if self.curr.col_idx == self.end.col_idx {
            // moving upwards
            let offset_op = if self.curr.row_idx < self.end.row_idx {
                usize::saturating_add
            } else {
                usize::saturating_sub
            };

            self.curr = Position {
                row_idx: RowIdx(offset_op(self.curr.row_idx.0, 1)),
                col_idx: self.curr.col_idx,
            };
        } else if self.curr.row_idx == self.end.row_idx {
            let offset_op = if self.curr.col_idx < self.end.col_idx {
                usize::saturating_add
            } else {
                usize::saturating_sub
            };

            self.curr = Position {
                row_idx: self.curr.row_idx,
                col_idx: ColIdx(offset_op(self.curr.col_idx.0, 1)),
            }
        } else {
            panic!("Positions can only move vertically or horizontally")
        }

        Some(self.curr)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    // FIXME: (perf) This could just be a Vec<GridItem> and then
    // implement index on the grid.
    grid: Vec<Vec<GridItem>>,
    grid_size: (usize, usize),
    guard_loc: Position,
    guard_direction: Direction,
}

impl Direction {
    pub fn next(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

impl World {
    #[must_use]
    fn new(grid: Vec<Vec<GridItem>>) -> Self {
        let grid_size = (grid.len(), grid[0].len());
        // This is probably dumb, but i dont' feel like figuring out how to
        // track indexes in the nom parser so that this can just be known during parse.
        // I'm sure it's possible but it's almost 7pm and i'm tired!

        let guard_loc = grid
            .iter()
            .flatten()
            .enumerate()
            .find(|(_, item)| matches!(item, GridItem::Guard { .. }))
            .map(|(idx, _)| Position::from((RowIdx(idx / grid_size.1), ColIdx(idx % grid_size.1))))
            .unwrap();

        World {
            grid,
            guard_loc,
            grid_size,
            guard_direction: Direction::North,
        }
    }

    pub fn next_rock_in_sight(&self) -> Result<Position,Position> {
        let iter: Box<dyn Iterator<Item = (Position, GridItem)>> = match self.guard_direction {
            Direction::North => Box::new(
                self.grid[0..self.guard_loc.row_idx.0]
                    .iter()
                    .enumerate()
                    .map(|(row_idx, row)| {
                        (
                            Position::from((RowIdx(row_idx), self.guard_loc.col_idx)),
                            row[self.guard_loc.col_idx.0],
                        )
                    })
                    .rev(),
            ),
            Direction::South => Box::new(
                self.grid[self.guard_loc.row_idx.0..]
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(row_idx, row)| {
                        (
                            Position::from((
                                RowIdx(row_idx + self.guard_loc.row_idx.0),
                                self.guard_loc.col_idx,
                            )),
                            row[self.guard_loc.col_idx.0],
                        )
                    }),
            ),
            Direction::West => Box::new(
                self.grid[self.guard_loc.row_idx.0][0..self.guard_loc.col_idx.0]
                    .iter()
                    .enumerate()
                    .map(|(col_idx, &item)| {
                        (
                            Position::from((self.guard_loc.row_idx, ColIdx(col_idx))),
                            item,
                        )
                    })
                    .rev(),
            ),
            Direction::East => Box::new(
                self.grid[self.guard_loc.row_idx.0][self.guard_loc.col_idx.0..]
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(col_idx, &item)| {
                        (
                            Position::from((
                                self.guard_loc.row_idx,
                                ColIdx(col_idx + self.guard_loc.col_idx.0),
                            )),
                            item,
                        )
                    }),
            ),
        };

        iter.find_or_last(|(_, e)| matches!(e, GridItem::Rock))
            .map(|(loc, item)| 
                match item {
                    GridItem::Rock => Ok(loc),
                    _ => Err(loc),
                })
                .unwrap_or(Err(self.guard_loc))
    }

    pub fn walk_guard(&mut self) -> Result<Position, Position> {
        let pos = self.next_rock_in_sight().map(|pos| {
            Position::from(match self.guard_direction {
                Direction::North => (RowIdx(pos.row_idx.0 + 1), pos.col_idx),
                Direction::South => (RowIdx(pos.row_idx.0 - 1), pos.col_idx),
                Direction::East => (pos.row_idx, ColIdx(pos.col_idx.0 - 1)),
                Direction::West => (pos.row_idx, ColIdx(pos.col_idx.0 + 1)),
            })
        });

        self.guard_direction = self.guard_direction.next();

        let real_pos = match pos {
            Ok(pos) => pos,
            Err(pos) => pos,
        };

        self.guard_loc = real_pos;

        pos
    }

    pub fn place_obstacle(&mut self, spot: Position) -> Result<(), ()> {
        match self.grid[spot.row_idx.0][spot.col_idx.0] {
            GridItem::Empty => {
                self.grid[spot.row_idx.0][spot.col_idx.0] = GridItem::Rock;
                Ok(())
            },
            _ => Err(())
        }
    }

    pub fn guard_location(&self) -> &Position {
        &self.guard_loc
    }

    pub fn guard_direction(&self) -> &Direction {
        &self.guard_direction
    }

    pub fn grid_size(&self) -> (usize, usize) {
        self.grid_size
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GridItem {
    Empty,
    Rock,
    Guard,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Guard {
    facing: Direction,
}

#[cfg(test)]
mod test {
    use crate::day6::{ColIdx, Direction, GridItem::*, RowIdx, World};
    use std::sync::LazyLock;

    use super::{parser::world, Position};

    pub static SAMPLE: LazyLock<World> = LazyLock::new(|| World {
        grid: vec![
            vec![
                Empty, Empty, Empty, Empty, Rock, Empty, Empty, Empty, Empty, Empty,
            ],
            vec![
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Rock,
            ],
            vec![
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            ],
            vec![
                Empty, Empty, Rock, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            ],
            vec![
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Rock, Empty, Empty,
            ],
            vec![
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            ],
            vec![
                Empty, Rock, Empty, Empty, Guard, Empty, Empty, Empty, Empty, Empty,
            ],
            vec![
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Rock, Empty,
            ],
            vec![
                Rock, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            ],
            vec![
                Empty, Empty, Empty, Empty, Empty, Empty, Rock, Empty, Empty, Empty,
            ],
        ],
        grid_size: (10, 10),
        guard_loc: Position::from((RowIdx(6), ColIdx(4))),
        guard_direction: Direction::North,
    });

    #[test]
    fn test_next_rock_in_sight_north() {
        let expected = Ok(Position::from((RowIdx(0), ColIdx(4))));
        let actual = SAMPLE.next_rock_in_sight();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next_rock_in_sight_south() {
        let input = r#"....#.....
........^#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...
"#;

        let (_, mut world) = world(input).unwrap();
        world.guard_direction = Direction::South;

        let expected = Ok(Position::from((RowIdx(7), ColIdx(8))));
        let actual = world.next_rock_in_sight();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next_rock_in_sight_west() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#......^.
........#.
#.........
......#...
"#;

        let (_, mut world) = world(input).unwrap();
        world.guard_direction = Direction::West;

        let expected = Ok(Position::from((RowIdx(6), ColIdx(1))));
        let actual = world.next_rock_in_sight();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next_rock_in_sight_east() {
        let input = r#"....#.....
.........#
..........
..#.......
..^....#..
..........
.#........
........#.
#.........
......#...
"#;

        let (_, mut world) = world(input).unwrap();
        world.guard_direction = Direction::East;

        let expected = Ok(Position::from((RowIdx(4), ColIdx(7))));
        let actual = world.next_rock_in_sight();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next_rock_in_sight_none() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#........
.......^#.
#.........
......#...
"#;

        let (_, mut world) = world(input).unwrap();
        world.guard_direction = Direction::South;

        let expected = Err(Position::from((RowIdx(9), ColIdx(7))));
        let actual = world.next_rock_in_sight();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next_rock_in_sight_edge() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#^..
"#;

        let (_, mut world) = world(input).unwrap();
        world.guard_direction = Direction::South;

        let expected = Err(Position::from((RowIdx(9), ColIdx(7))));
        let actual = world.next_rock_in_sight();

        assert_eq!(actual, expected);
    }
}

pub mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::newline,
        combinator::{map, value},
        multi::{many1, separated_list0},
        IResult,
    };

    use super::{GridItem, World};

    pub fn world(input: &str) -> IResult<&str, World> {
        map(
            separated_list0(newline, grid_row), 
            |grid| World::new(grid)
        )(input)
    }

    fn grid_row(input: &str) -> IResult<&str, Vec<GridItem>> {
        many1(grid_item)(input)
    }

    fn grid_item(input: &str) -> IResult<&str, GridItem> {
        alt((
            value(GridItem::Empty, tag(".")),
            value(GridItem::Rock, tag("#")),
            value(GridItem::Guard, tag("^")),
        ))(input)
    }

    #[cfg(test)]
    mod test {

        use crate::day6::{parser::world, test::SAMPLE, GridItem};
        use test_case::test_case;

        use super::{grid_item, grid_row};

        #[test]
        fn test_parse_sample_input() {
            let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

            let (_, actual) = world(input).unwrap();
            assert_eq!(actual.grid.len(), SAMPLE.grid.len());
            assert_eq!(
                actual.grid.iter().map(|r| r.len()).collect::<Vec<_>>(),
                SAMPLE.grid.iter().map(|r| r.len()).collect::<Vec<_>>()
            );
            assert_eq!(actual, *SAMPLE);
        }

        #[test]
        fn parse_grid_line() {
            use crate::day6::GridItem::*;

            let input = "....#.....";
            let (_, actual) = grid_row(input).unwrap();
            let expected = vec![
                Empty, Empty, Empty, Empty, Rock, Empty, Empty, Empty, Empty, Empty,
            ];

            assert_eq!(actual, expected);
        }

        #[test_case(".", GridItem::Empty)]
        #[test_case("#", GridItem::Rock)]
        #[test_case("^", GridItem::Guard)]
        fn test_parse_griditem(input: &str, expected: GridItem) {
            let (_, actual) = grid_item(input).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
