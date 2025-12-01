use aoc_2024::day10::{parser::parse, trailhead_ratings, trailhead_scores};

pub fn main() {
    let input = include_str!("../../data/day10.txt");
    println!("{}", compute(input));
}

fn compute(input: &str) -> u32 {
    let map = parse(input);
    trailhead_ratings(&map).iter().sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn verify_not_broken() {
        let input = include_str!("../../data/day10.txt");
        assert_eq!(928, super::compute(input));
    }

    #[test]
    fn test_sample_input() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        
        assert_eq!(81, super::compute(input));
    }
}