use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Disk(Vec<DiskItem>);

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.0 {
            write!(f, "{}", item)?;
        }

        Ok(())
    }
}

impl Disk {
    pub fn defrag(&mut self) {
        let empty_iter = self.0.iter().enumerate().filter(|(_, i)| matches!(i, DiskItem::Empty { size: _ })).map(|(idx, _)| idx).collect::<Vec<_>>().into_iter();
        let item_iter = self.0.iter().enumerate().rev().filter(|(_, i)| matches!(i, DiskItem::Item { size: _, id: _ })).map(|(idx, _)| idx).collect::<Vec<_>>().into_iter();

        for (empty_idx, item_idx) in empty_iter.zip(item_iter).take_while(|(e_i, i_i)| e_i < i_i)  {
            self.0.swap(empty_idx, item_idx);
        }
    }

    pub fn filewise_defrag(&mut self) {
        let mut item_iter = self.0.iter().enumerate().rev().filter(|(_, i)| matches!(i, DiskItem::Item { size: _, id: _ })).map(|(idx, _)| idx).collect::<Vec<_>>().into_iter().peekable();
        let mut item_idx = item_iter.next().unwrap();

        while item_idx > 0 {
            let (item_size, item_id) = match self.0[item_idx] {
                DiskItem::Empty { size: _ } => panic!("should never get an empty item"),
                DiskItem::Item { size, id } => (size, id)
            };

            let empty_idx = self.0.iter().enumerate().filter(|(_, i)| matches!(i, DiskItem::Empty { size: _ })).map(|(idx, item)| (idx, match item {
                DiskItem::Empty { size } => *size,
                DiskItem::Item { size: _, id: _ } => panic!("empty idx should never have an item"),
            }))
            .find(|(idx, size)| *size >= item_size && *idx < item_idx)
            .map(|(idx, _)| idx);

            let mut empty_offset = 0;

            loop {
                if let Some(empty_idx) = empty_idx {
                    self.0.swap(empty_idx + empty_offset, item_idx);
                    empty_offset += 1;
                }

                // println!("{self} {:?} {item_idx}", empty_idx.map(|i| i + empty_offset));
                
                if let Some(next_idx) = item_iter.next(){
                    item_idx = next_idx
                } else {
                    break;
                }

                let next_item_id = match self.0[item_idx] {
                    DiskItem::Empty { size: _ } => panic!("should never get an empty item"),
                    DiskItem::Item { size: _, id } => id
                };

                if next_item_id != item_id {
                    break;
                }
            }
        }
    }

    pub fn checksum(&self) -> usize {
        self.0.iter().enumerate().map(|(idx, item)| {
            match item {
                DiskItem::Empty { size: _ } => 0,
                DiskItem::Item{ size: _, id } => idx * id,
            }
        })
        .sum()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiskItem {
    Empty {
        size: usize
    },
    Item {
        size: usize,
        id: usize,
    }
}
impl Display for DiskItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            DiskItem::Empty { size: _ } => ".".to_string(),
            DiskItem::Item { size: _, id } => id.to_string()
        })
    }
}
#[cfg(test)]
mod test {

}

pub mod parser {
    use super::{Disk, DiskItem};

    enum ParserState {
        Item(usize),
        Empty(usize),
    }

    pub fn parse(input: &str) -> Disk {
        Disk(input.chars()
            .flat_map(|c| c.to_digit(10))
            .map(|item| item as usize)
            .fold(
            (ParserState::Item(0), Vec::new()),
            |(state, mut acc), next| {
                for i in (1..=next).rev() {
                    acc.push(match state {
                        ParserState::Item(id) => DiskItem::Item { size: next, id: id },
                        ParserState::Empty(_) => DiskItem::Empty { size: i }
                    });
                };

                (match state {
                    ParserState::Item(id) => ParserState::Empty(id),
                    ParserState::Empty(id) => ParserState::Item(id + 1),
                }, acc)
            })
            .1)
    }

    #[cfg(test)]
    mod test {
        use test_case::test_case;

        use crate::day9::{parser::parse, DiskItem};

        use crate::day9::{Disk, DiskItem::*};

        #[test_case("3", Disk(vec![DiskItem::Item { id: 0, size: 3 }, DiskItem::Item { id: 0, size: 3}, DiskItem::Item { id: 0, size: 3}]))]
        #[test_case("22", Disk(vec![Item { id: 0, size: 2 }, Item { id: 0, size: 2 }, Empty { size: 2 }, Empty { size: 1 }]))]
        #[test_case("222", Disk(vec![Item { id: 0, size: 2 }, Item { id: 0, size: 2 },Empty { size: 2}, Empty { size: 1} ,Item { id: 1, size: 2 }, Item { id: 1, size: 2 },]))]
        #[test_case("22202", Disk(vec![Item{ id: 0, size: 2 },Item{ id: 0, size: 2 },Empty{ size: 2}, Empty{ size: 1},Item{ id: 1, size: 2 },Item{ id: 1, size: 2 },Item{ id: 2, size: 2 },Item{ id: 2, size: 2 },]))]
        #[test_case("12345", Disk(vec![Item{ id: 0, size: 1 }, Empty{ size: 2}, Empty{ size: 1}, Item{ id: 1, size: 3 }, Item{ id: 1, size: 3 }, Item{ id: 1, size: 3 }, Empty{ size: 4}, Empty{ size: 3}, Empty{ size: 2}, Empty{ size: 1}, Item{ id: 2, size: 5 },Item{ id: 2, size: 5 },Item{ id: 2, size: 5 },Item{ id: 2, size: 5 },Item{ id: 2, size: 5 },]))]
        fn test_parse_compacted(input: &str, expected: Disk) {
            let actual = parse(input);
            assert_eq!(actual, expected);
        }

        #[test_case("12345")]
        #[ignore = "for debugging purposes"]
        fn test_display(input: &str) {
            println!("{}", parse(input));
            assert!(false);
        }

        #[ignore = "for debugging purposes"]
        #[test_case("12345", false)]
        #[test_case("2333133121414131402", false)]
        fn test_defrag(input: &str, expected: bool) {
            let mut disk = parse(input);
            disk.defrag();
            println!("{disk}");
            assert!(expected);
        }

        // #[ignore = "for debugging purposes"]
        #[test_case("12345", false)]
        #[test_case("2333133121414131402", false)]
        fn test_file_defrag(input: &str, expected: bool) {
            let mut disk = parse(input);
            println!("{disk}");
            disk.filewise_defrag();
            println!("{disk}");
            assert!(expected);
        }
    }
}