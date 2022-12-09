use std::cell::RefCell;

fn main() {
    let input = include_str!("input.txt");
    let forest = parse_input(input);
    println!("{}", count_visible(&forest));
    println!(
        "{:?}",
        forest
            .iter()
            .flat_map(|r| r.iter().map(|t| t.borrow().scenic_score()))
            .max()
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tree {
    height: u64,
    neighbor_visibility_info: TreeMeta,
    view_distances: TreeMeta,
}

impl Tree {
    pub fn new(height: u64) -> Self {
        Self {
            height,
            neighbor_visibility_info: TreeMeta::new_empty(),
            view_distances: TreeMeta::new_empty(),
        }
    }

    #[cfg(test)]
    pub fn new_with_meta(height: u64, meta: TreeMeta) -> Self {
        Self {
            height,
            neighbor_visibility_info: meta,
            view_distances: TreeMeta::new_empty(),
        }
    }

    pub fn is_visible(&self) -> bool {
        self.neighbor_visibility_info.north.lt(&Some(self.height))
            || self.neighbor_visibility_info.south.lt(&Some(self.height))
            || self.neighbor_visibility_info.east.lt(&Some(self.height))
            || self.neighbor_visibility_info.west.lt(&Some(self.height))
    }

    pub fn scenic_score(&self) -> u64 {
        self.view_distances.north.unwrap_or_default()
            * self.view_distances.east.unwrap_or_default()
            * self.view_distances.south.unwrap_or_default()
            * self.view_distances.west.unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TreeMeta {
    north: Option<u64>,
    south: Option<u64>,
    east: Option<u64>,
    west: Option<u64>,
}

impl TreeMeta {
    pub fn new_empty() -> Self {
        Self {
            north: None,
            south: None,
            east: None,
            west: None,
        }
    }

    #[cfg(test)]
    pub fn new(
        max_north: Option<u64>,
        max_east: Option<u64>,
        max_south: Option<u64>,
        max_west: Option<u64>,
    ) -> Self {
        Self {
            north: max_north,
            south: max_south,
            east: max_east,
            west: max_west,
        }
    }
}

fn count_visible(forest: &[Vec<RefCell<Tree>>]) -> u64 {
    forest
        .iter()
        .flat_map(|row| row.iter())
        .filter(|t| t.borrow().is_visible())
        .count() as u64
}

fn parse_input(input: &str) -> Vec<Vec<RefCell<Tree>>> {
    let forest: Vec<Vec<RefCell<Tree>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| RefCell::new(Tree::new(c as u64 - b'0' as u64)))
                .collect()
        })
        .collect();

    for i in 1..forest.len() {
        for j in 0..forest[i].len() {
            let t = &forest[i][j];
            let other_t = &forest[i - 1][j];

            let max = Some(
                other_t
                    .borrow()
                    .neighbor_visibility_info
                    .north
                    .map(|m| other_t.borrow().height.max(m))
                    .unwrap_or(other_t.borrow().height),
            );

            t.borrow_mut().neighbor_visibility_info.north = max;

            t.borrow_mut().view_distances.north =
                Some(if t.borrow().height > other_t.borrow().height {
                    let mut view_distance = 0;
                    for i in (0..i).rev() {
                        view_distance += 1;
                        if t.borrow().height <= forest[i][j].borrow().height {
                            break;
                        }
                    }
                    view_distance
                } else {
                    0
                });
        }
    }
    // find max south
    // means start at the bottom and move upwards
    for i in (0..(forest.len() - 1)).rev() {
        for j in 0..forest[i].len() {
            let t = &forest[i][j];
            let other_t = &forest[i + 1][j];
            let max = Some(
                other_t
                    .borrow()
                    .neighbor_visibility_info
                    .south
                    .map(|m| other_t.borrow().height.max(m))
                    .unwrap_or(other_t.borrow().height),
            );
            t.borrow_mut().neighbor_visibility_info.south = max;

            t.borrow_mut().view_distances.south =
                Some(if t.borrow().height > other_t.borrow().height {
                    let mut view_distance = 0;
                    for i in (i + 1)..forest.len() {
                        view_distance += 1;
                        if t.borrow().height <= forest[i][j].borrow().height {
                            break;
                        }
                    }
                    view_distance
                } else {
                    0
                });
        }
    }
    // find max east means scan right to left
    for i in 0..forest.len() {
        for j in (0..(forest[i].len() - 1)).rev() {
            let t = &forest[i][j];
            let other_t = &forest[i][j + 1];
            let max = Some(
                other_t
                    .borrow()
                    .neighbor_visibility_info
                    .east
                    .map(|m| other_t.borrow().height.max(m))
                    .unwrap_or(other_t.borrow().height),
            );
            t.borrow_mut().neighbor_visibility_info.east = max;

            t.borrow_mut().view_distances.east =
                Some(if t.borrow().height > other_t.borrow().height {
                    let mut view_distance = 0;
                    for j in (j + 1)..forest[i].len() {
                        view_distance += 1;
                        if t.borrow().height <= forest[i][j].borrow().height {
                            break;
                        }
                    }
                    view_distance
                } else {
                    0
                });
        }
    }
    // find max west
    // menas scan left to right
    for i in 0..forest.len() {
        for j in 1..forest[i].len() {
            let t = &forest[i][j];
            let other_t = &forest[i][j - 1];
            let max = Some(
                other_t
                    .borrow()
                    .neighbor_visibility_info
                    .west
                    .map(|m| other_t.borrow().height.max(m))
                    .unwrap_or(other_t.borrow().height),
            );
            t.borrow_mut().neighbor_visibility_info.west = max;

            t.borrow_mut().view_distances.west =
                Some(if t.borrow().height > other_t.borrow().height {
                    let mut view_distance = 0;
                    for j in (0..j).rev() {
                        view_distance += 1;
                        if t.borrow().height <= forest[i][j].borrow().height {
                            break;
                        }
                    }
                    view_distance
                } else {
                    0
                });
        }
    }

    forest
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use crate::{count_visible, parse_input, Tree, TreeMeta};

    #[test]
    fn test_parse_input() {
        let input = "30373
25512
65332
33549
35390";

        let expected = vec![
            vec![
                RefCell::new(Tree::new_with_meta(
                    3,
                    TreeMeta::new(None, Some(7), Some(6), None),
                )),
                RefCell::new(Tree::new_with_meta(
                    0,
                    TreeMeta::new(None, Some(7), Some(5), Some(3)),
                )),
                RefCell::new(Tree::new_with_meta(
                    3,
                    TreeMeta::new(None, Some(7), Some(5), Some(3)),
                )),
                RefCell::new(Tree::new_with_meta(
                    7,
                    TreeMeta::new(None, Some(3), Some(9), Some(3)),
                )),
                RefCell::new(Tree::new_with_meta(
                    3,
                    TreeMeta::new(None, None, Some(9), Some(7)),
                )),
            ],
            vec![
                RefCell::new(Tree::new_with_meta(
                    2,
                    TreeMeta::new(Some(3), Some(5), Some(6), None),
                )),
                RefCell::new(Tree::new_with_meta(
                    5,
                    TreeMeta::new(Some(0), Some(5), Some(5), Some(2)),
                )),
                RefCell::new(Tree::new_with_meta(
                    5,
                    TreeMeta::new(Some(3), Some(2), Some(5), Some(5)),
                )),
                RefCell::new(Tree::new_with_meta(
                    1,
                    TreeMeta::new(Some(7), Some(2), Some(9), Some(5)),
                )),
                RefCell::new(Tree::new_with_meta(
                    2,
                    TreeMeta::new(Some(3), None, Some(9), Some(5)),
                )),
            ],
            vec![
                RefCell::new(Tree::new_with_meta(
                    6,
                    TreeMeta::new(Some(3), Some(5), Some(3), None),
                )),
                RefCell::new(Tree::new_with_meta(
                    5,
                    TreeMeta::new(Some(5), Some(3), Some(5), Some(6)),
                )),
                RefCell::new(Tree::new_with_meta(
                    3,
                    TreeMeta::new(Some(5), Some(3), Some(5), Some(6)),
                )),
                RefCell::new(Tree::new_with_meta(
                    3,
                    TreeMeta::new(Some(7), Some(2), Some(9), Some(6)),
                )),
                RefCell::new(Tree::new_with_meta(
                    2,
                    TreeMeta::new(Some(3), None, Some(9), Some(6)),
                )),
            ],
            vec![
                RefCell::new(Tree::new_with_meta(
                    3,
                    TreeMeta::new(Some(6), Some(9), Some(3), None),
                )),
                RefCell::new(Tree::new_with_meta(
                    3,
                    TreeMeta::new(Some(5), Some(9), Some(5), Some(3)),
                )),
                RefCell::new(Tree::new_with_meta(
                    5,
                    TreeMeta::new(Some(5), Some(9), Some(3), Some(3)),
                )),
                RefCell::new(Tree::new_with_meta(
                    4,
                    TreeMeta::new(Some(7), Some(9), Some(9), Some(5)),
                )),
                RefCell::new(Tree::new_with_meta(
                    9,
                    TreeMeta::new(Some(3), None, Some(0), Some(5)),
                )),
            ],
            vec![
                RefCell::new(Tree::new_with_meta(
                    3,
                    TreeMeta::new(Some(6), Some(9), None, None),
                )),
                RefCell::new(Tree::new_with_meta(
                    5,
                    TreeMeta::new(Some(5), Some(9), None, Some(3)),
                )),
                RefCell::new(Tree::new_with_meta(
                    3,
                    TreeMeta::new(Some(5), Some(9), None, Some(5)),
                )),
                RefCell::new(Tree::new_with_meta(
                    9,
                    TreeMeta::new(Some(7), Some(0), None, Some(5)),
                )),
                RefCell::new(Tree::new_with_meta(
                    0,
                    TreeMeta::new(Some(9), None, None, Some(9)),
                )),
            ],
        ];

        let actual = parse_input(input);
        // assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_visible() {
        let input = "30373
25512
65332
33549
35390";

        let expected = count_visible(&parse_input(input));
        assert_eq!(21, expected);
    }

    #[test]
    fn test_scenic_score() {
        let input = "30373
25512
65332
33549
35390";

        let forest = parse_input(input);
        assert_eq!(8, forest[3][2].borrow().scenic_score());
    }
}
