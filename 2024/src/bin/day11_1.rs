use aoc_2024::day11::{memoized_blink, parser::parse};
use iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::*;

pub fn main() {
    let input = include_str!("../../data/day11.txt");
    println!("{}", compute(input));
}

fn compute(input: &str) -> usize {
    const TARGET_BLINKS: usize = 75;

    let init = parse(input);
    
    init.par_iter().map(|stone| memoized_blink(*stone,  TARGET_BLINKS)).sum()
}

#[cfg(test)]
mod test {
    use crate::compute;

    #[test]
    fn test_sample_input() {
        let input = "125 17";
        assert_eq!(compute(input), 55312);
    }
}