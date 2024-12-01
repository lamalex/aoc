#![feature(binary_heap_into_iter_sorted)]

use aoc_2024::day1::parser::parse_nums;
use std::collections::BinaryHeap;

trait UnzipHeaps<Ta, Tb> {
    fn unzip_heaps(self) -> (BinaryHeap<Ta>, BinaryHeap<Tb>);
}

impl<I, Ta, Tb> UnzipHeaps<Ta, Tb> for I
where
    I: IntoIterator<Item = (Ta, Tb)>,
    Ta: Ord,
    Tb: Ord,
{
    fn unzip_heaps(self) -> (BinaryHeap<Ta>, BinaryHeap<Tb>) {
        let mut heap1 = BinaryHeap::new();
        let mut heap2 = BinaryHeap::new();

        for (a, b) in self {
            heap1.push(a);
            heap2.push(b);
        }

        (heap1, heap2)
    }
}

pub fn main() {
    let input = include_str!("../../data/day1.txt");
    print!("{}", day1_1(input));
}

fn day1_1(input: &str) -> i32 {
    let (l1, l2): (BinaryHeap<_>, BinaryHeap<_>) = input
        .lines()
        .flat_map(|line| parse_nums(line))
        .map(|(_, pair)| pair)
        .unzip_heaps();

    l1.into_iter_sorted()
        .zip(l2.into_iter_sorted())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[cfg(test)]
mod test {
    use super::day1_1;

    #[test]
    fn test_day1_1_sample() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(11, day1_1(input))
    }
}
