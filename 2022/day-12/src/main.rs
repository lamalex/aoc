use pathfinding::prelude::dijkstra;
use std::ops::{Index, Sub};

fn main() {
    let input = include_str!("input.txt");
    let pathfinder: PathFinder = input.into();
    println!("{}", pathfinder.grid);
    println!("Start {}", pathfinder.start_loc);
    println!("Destination {}", pathfinder.dest_loc);

    let r: usize = pathfinder
        .grid
        .0
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if c == &Cell::Step(b'a') {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
        .filter_map(|a_cell| {
            dijkstra(
                &a_cell,
                |p| pathfinder.grid.successors(*p),
                |p| *p == pathfinder.dest_loc,
            )
        })
        .map(|paths| paths.1)
        .min()
        .unwrap();

    println!("shortest path is {}", r);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Start,
    End,
    Step(u8),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Start => write!(f, "{}", "S"),
            Cell::End => write!(f, "{}", "E"),
            Cell::Step(v) => write!(f, "{}", *v as char),
        }
    }
}

impl From<Cell> for i64 {
    fn from(c: Cell) -> Self {
        match c {
            Cell::Start => (b'a' - 1) as i64,
            Cell::End => (b'z' + 1) as i64,
            Cell::Step(v) => v as i64,
        }
    }
}

impl Sub<Cell> for Cell {
    type Output = i64;

    fn sub(self, rhs: Cell) -> Self::Output {
        i64::from(self) - i64::from(rhs)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Start => match other {
                Self::Start => std::cmp::Ordering::Equal,
                _ => std::cmp::Ordering::Less,
            },
            Self::End => std::cmp::Ordering::Greater,
            Self::Step(v_s) => match other {
                Self::Start => std::cmp::Ordering::Greater,
                Self::End => std::cmp::Ordering::Less,
                Self::Step(v_o) => v_s.cmp(v_o),
            },
        }
    }
}

impl From<Cell> for char {
    fn from(input: Cell) -> Self {
        match input {
            Cell::Start => 'S',
            Cell::End => 'E',
            Cell::Step(v) => v as char,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> std::fmt::Display for Point<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y {}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TopoMap(Vec<Vec<Cell>>);
impl std::fmt::Display for TopoMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|row| row.iter().map(|c| char::from(*c)).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

impl Index<Point<usize>> for TopoMap {
    type Output = Cell;

    fn index(&self, index: Point<usize>) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl TopoMap {
    fn successors(
        &self,
        from_pos: Point<usize>,
    ) -> impl IntoIterator<Item = (Point<usize>, usize)> + '_ {
        let curr = self[from_pos];
        vec![
            from_pos
                .y
                .checked_sub(1)
                .map(|y| Point { x: from_pos.x, y }),
            from_pos
                .y
                .checked_add(1)
                .map(|y| Point { x: from_pos.x, y }),
            from_pos
                .x
                .checked_sub(1)
                .map(|x| Point { x, y: from_pos.y }),
            from_pos
                .x
                .checked_add(1)
                .map(|x| Point { x, y: from_pos.y }),
        ]
        .into_iter()
        .filter_map(|p| {
            if let Some(p_p) = p {
                if p_p.y < self.0.len() && p_p.x < self.0[0].len() {
                    p
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter_map(move |p| {
            if self[p] <= curr || self[p] - curr == 1 {
                Some((p, 1))
            } else {
                None
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PathFinder {
    start_loc: Point<usize>,
    dest_loc: Point<usize>,
    grid: TopoMap,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'a'..='z' => Cell::Step(c as u8),
            'S' => Cell::Start,
            'E' => Cell::End,
            _ => panic!("You're an ERROR in my INPUT!"),
        }
    }
}

impl From<&str> for TopoMap {
    fn from(input: &str) -> Self {
        TopoMap(
            input
                .lines()
                .map(|line| line.chars().map(Cell::from).collect())
                .collect(),
        )
    }
}

impl From<&str> for PathFinder {
    fn from(input: &str) -> Self {
        let grid: TopoMap = input.into();
        let mut start_loc = None;
        let mut dest_loc = None;

        for (x, y, v) in grid
            .0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, v)))
        {
            match v {
                &Cell::Start => {
                    start_loc = Some(Point { x, y });
                }
                &Cell::End => {
                    dest_loc = Some(Point { x, y });
                }
                &Cell::Step(_) => continue,
            }
        }

        PathFinder {
            start_loc: start_loc.unwrap(),
            dest_loc: dest_loc.unwrap(),
            grid,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Cell::*;
    use crate::PathFinder;
    use crate::Point;
    use crate::TopoMap;
    use pathfinding::prelude::dijkstra;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn find_shortest_path() {
        let pf: PathFinder = INPUT.into();
        let r = dijkstra(
            &pf.start_loc,
            |p| pf.grid.successors(*p),
            |p| *p == pf.dest_loc,
        );

        assert_eq!(r.unwrap().1, 31);
    }

    #[test]
    fn test_parse_char_to_cell() {
        for h in 'a'..='z' {
            let expected = Step(h as u8);
            assert_eq!(expected, h.into());
        }

        assert_eq!(End, 'E'.into());
        assert_eq!(Start, 'S'.into());
    }

    #[test]
    fn test_parse_grid_to_topomap() {
        assert_eq!(expected_topomap_from_input(), INPUT.into());
    }

    #[test]
    fn test_parse_grid_to_pathfinder() {
        let expected = PathFinder {
            start_loc: Point { x: 0, y: 0 },
            dest_loc: Point { x: 5, y: 2 },
            grid: expected_topomap_from_input(),
        };

        let actual = INPUT.into();
        assert_eq!(expected, actual);
    }

    fn expected_topomap_from_input() -> TopoMap {
        TopoMap(vec![
            vec![
                Start,
                Step(b'a'),
                Step(b'b'),
                Step(b'q'),
                Step(b'p'),
                Step(b'o'),
                Step(b'n'),
                Step(b'm'),
            ],
            vec![
                Step(b'a'),
                Step(b'b'),
                Step(b'c'),
                Step(b'r'),
                Step(b'y'),
                Step(b'x'),
                Step(b'x'),
                Step(b'l'),
            ],
            vec![
                Step(b'a'),
                Step(b'c'),
                Step(b'c'),
                Step(b's'),
                Step(b'z'),
                End,
                Step(b'x'),
                Step(b'k'),
            ],
            vec![
                Step(b'a'),
                Step(b'c'),
                Step(b'c'),
                Step(b't'),
                Step(b'u'),
                Step(b'v'),
                Step(b'w'),
                Step(b'j'),
            ],
            vec![
                Step(b'a'),
                Step(b'b'),
                Step(b'd'),
                Step(b'e'),
                Step(b'f'),
                Step(b'g'),
                Step(b'h'),
                Step(b'i'),
            ],
        ])
    }
}
