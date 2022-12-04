use std::ops::RangeInclusive;

use itertools::Itertools;

trait RangeInclusiveExt {
    fn overlaps_with(&self, other: &Self) -> bool;
}

impl<T> RangeInclusiveExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn overlaps_with(&self, other: &Self) -> bool {
        // 👇🏻 this could definitely be cleaned up with some methods on RangeInclusivex
        (self.start() <= other.start() && self.end() >= other.start())
            || (other.start() <= self.start() && other.end() >= self.start())
    }
}

fn main() {
    let data = include_str!("input.txt");

    let count = data
        .lines()
        .flat_map(|line| {
            line.split(',')
                .flat_map(|range| {
                    range
                        .split('-')
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect_tuple::<(_, _)>()
                        .map(|pair| pair.0..=pair.1)
                })
                .collect_tuple::<(_, _)>()
        })
        .filter(|(a, b)| a.overlaps_with(b))
        .count();

    println!("{count}");
}
