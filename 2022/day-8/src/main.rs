use std::cell::RefCell;

fn main() {
    let input = include_str!("input.txt");
    let forest = parse_input(input);
    println!("{}", count_visible(&forest));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tree {
    height: u8,
    meta: TreeMeta,
}

impl Tree {
    pub fn new(height: u8) -> Self {
        Self {
            height,
            meta: TreeMeta::new_empty(),
        }
    }

    #[cfg(test)]
    pub fn new_with_meta(height: u8, meta: TreeMeta) -> Self {
        Self { height, meta }
    }

    pub fn is_visible(&self) -> bool {
        self.meta.max_north.lt(&Some(self.height))
            || self.meta.max_south.lt(&Some(self.height))
            || self.meta.max_east.lt(&Some(self.height))
            || self.meta.max_west.lt(&Some(self.height))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TreeMeta {
    max_north: Option<u8>,
    max_south: Option<u8>,
    max_east: Option<u8>,
    max_west: Option<u8>,
}

impl TreeMeta {
    pub fn new_empty() -> Self {
        Self {
            max_north: None,
            max_south: None,
            max_east: None,
            max_west: None,
        }
    }

    #[cfg(test)]
    pub fn new(
        max_north: Option<u8>,
        max_east: Option<u8>,
        max_south: Option<u8>,
        max_west: Option<u8>,
    ) -> Self {
        Self {
            max_north,
            max_south,
            max_east,
            max_west,
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
                .map(|c| RefCell::new(Tree::new(c as u8 - b'0')))
                .collect()
        })
        .collect();

    // find the max northern neighbor
    // means start at the top and move down
    // 30373
    // 25512
    // 65332
    // 33549
    // 35390
    for i in 1..forest.len() {
        for j in 0..forest[i].len() {
            let t = &forest[i][j];
            let other_t = &forest[i - 1][j];

            let max = Some(other_t
                .borrow()
                .meta
                .max_north
                .map(|m| other_t.borrow().height.max(m))
                .unwrap_or(other_t.borrow().height));
            
            t.borrow_mut().meta.max_north = max;
        }
    }
    // find max south
    // means start at the bottom and move upwards
    for i in (0..(forest.len() - 1)).rev() {
        for j in 0..forest[i].len() {
            let t = &forest[i][j];
            let other_t = &forest[i + 1][j];
            let max = Some(other_t
                .borrow()
                .meta
                .max_south
                .map(|m| other_t.borrow().height.max(m))
                .unwrap_or(other_t.borrow().height));
            t.borrow_mut().meta.max_south = max;
        }
    }
    // find max east means scan right to left
    for i in 0..forest.len() {
        for j in (0..(forest[i].len() - 1)).rev() {
            let t = &forest[i][j];
            let other_t = &forest[i][j + 1];
            let max = Some(other_t
                .borrow()
                .meta
                .max_east
                .map(|m| other_t.borrow().height.max(m))
                .unwrap_or(other_t.borrow().height));
            t.borrow_mut().meta.max_east = max;
        }
    }
    // find max west
    // menas scan left to right
    for i in 0..forest.len() {
        for j in 1..forest[i].len() {
            let t = &forest[i][j];
            let other_t = &forest[i][j - 1];
            let max = Some(other_t
                .borrow()
                .meta
                .max_west
                .map(|m| other_t.borrow().height.max(m))
                .unwrap_or(other_t.borrow().height));
            t.borrow_mut().meta.max_west = max;
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
        assert_eq!(expected, actual);
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
}
