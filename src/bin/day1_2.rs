use std::collections::HashMap;

use aoc_2024::day1::parser::parse_nums;
use rustc_hash::FxBuildHasher;

pub fn main() {
    let input = include_str!("../../data/day1.txt");
    print!("{}", day1_2(input));
}

fn day1_2(input: &str) -> i32 {
    let mut counts = HashMap::with_capacity_and_hasher(1024, FxBuildHasher::default());

    let l1: Vec<_> = input
        .lines()
        .flat_map(|line| parse_nums(line))
        .map(|(_, pair)| {
            *counts.entry(pair.1).or_insert(0) += 1;
            pair.0
        })
        .fold(Vec::with_capacity(1024), |mut acc, next| {
            acc.push(next);
            acc
        });

    l1.iter()
        .filter_map(|l| counts.get(&l).map(|c| c * l))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day1_2;

    #[test]
    fn test_day1_2_sample() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(31, day1_2(input))
    }
}
