fn main() {
    println!("Hello, world!");
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

fn count_visible(forest: &[Vec<Tree>]) -> u64 {
    forest
        .iter()
        .flat_map(|row| row.iter())
        .filter(|t| t.is_visible())
        .count() as u64
}

fn parse_input(input: &str) -> Vec<Vec<Tree>> {
    let forest = input
        .lines()
        .map(|line| line.chars().map(|c| Tree::new(c as u8 - b'0')).collect())
        .collect();

    forest
}

#[cfg(test)]
mod test {
    use crate::{count_visible, parse_input, Tree, TreeMeta};

    #[test]
    fn test_parse_input() {
        let input = "30373
25512
65332
33549
35390";

        let _expected = &[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ];

        let expected = vec![
            vec![
                Tree::new_with_meta(3, TreeMeta::new(None, Some(7), Some(6), None)),
                Tree::new_with_meta(0, TreeMeta::new(None, Some(7), Some(5), Some(3))),
                Tree::new_with_meta(3, TreeMeta::new(None, Some(7), Some(5), Some(3))),
                Tree::new_with_meta(7, TreeMeta::new(None, Some(3), Some(9), Some(3))),
                Tree::new_with_meta(3, TreeMeta::new(None, None, Some(9), Some(7))),
            ],
            vec![
                Tree::new_with_meta(2, TreeMeta::new(Some(3), Some(5), Some(6), None)),
                Tree::new_with_meta(5, TreeMeta::new(Some(0), Some(5), Some(5), Some(2))),
                Tree::new_with_meta(5, TreeMeta::new(Some(3), Some(2), Some(5), Some(5))),
                Tree::new_with_meta(1, TreeMeta::new(Some(7), Some(2), Some(9), Some(5))),
                Tree::new_with_meta(2, TreeMeta::new(Some(3), None, Some(9), Some(5))),
            ],
            vec![
                Tree::new_with_meta(6, TreeMeta::new(Some(3), Some(5), Some(3), None)),
                Tree::new_with_meta(5, TreeMeta::new(Some(5), Some(3), Some(5), Some(6))),
                Tree::new_with_meta(3, TreeMeta::new(Some(5), Some(3), Some(5), Some(6))),
                Tree::new_with_meta(3, TreeMeta::new(Some(7), Some(2), Some(9), Some(6))),
                Tree::new_with_meta(2, TreeMeta::new(Some(3), None, Some(9), Some(6))),
            ],
            vec![
                Tree::new_with_meta(3, TreeMeta::new(Some(6), Some(9), Some(3), None)),
                Tree::new_with_meta(3, TreeMeta::new(Some(5), Some(9), Some(5), Some(3))),
                Tree::new_with_meta(5, TreeMeta::new(Some(5), Some(9), Some(3), Some(3))),
                Tree::new_with_meta(4, TreeMeta::new(Some(7), Some(9), Some(9), Some(5))),
                Tree::new_with_meta(9, TreeMeta::new(Some(3), None, Some(0), Some(5))),
            ],
            vec![
                Tree::new_with_meta(3, TreeMeta::new(Some(6), Some(9), None, None)),
                Tree::new_with_meta(5, TreeMeta::new(Some(5), Some(9), None, Some(3))),
                Tree::new_with_meta(3, TreeMeta::new(Some(5), Some(9), None, Some(5))),
                Tree::new_with_meta(9, TreeMeta::new(Some(7), Some(0), None, Some(5))),
                Tree::new_with_meta(0, TreeMeta::new(Some(9), None, None, Some(9))),
            ],
        ];

        dbg!(&expected);

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
