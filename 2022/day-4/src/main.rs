use std::ops::RangeInclusive;

use itertools::Itertools;

trait RangeInclusiveExt {
    fn overlaps_with(&self, other: &Self) -> bool;
    fn is_superset_or_subset_of(&self, other: &Self) -> bool;
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

    fn is_superset_or_subset_of(&self, other: &Self) -> bool {
        return (other.start() >= self.start() && other.end() <= self.end())
            || (self.start() >= other.start() && self.end() <= other.end());
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
        .fold(OverlapCounter::default(), |acc, next| acc + next);

    println!("{count:?}");
}

#[derive(Clone, Copy, Debug, Default)]
struct OverlapCounter {
    full_overlap_count: usize,
    simple_overlap_count: usize,
}

impl<T> std::ops::Add<(RangeInclusive<T>, RangeInclusive<T>)> for OverlapCounter
where
    T: PartialOrd,
{
    type Output = Self;

    fn add(self, rhs: (RangeInclusive<T>, RangeInclusive<T>)) -> Self::Output {
        Self {
            full_overlap_count: self.full_overlap_count
                + rhs.0.is_superset_or_subset_of(&rhs.1) as usize,
            simple_overlap_count: self.simple_overlap_count + rhs.0.overlaps_with(&rhs.1) as usize,
        }
    }
}
