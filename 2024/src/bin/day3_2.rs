use aoc_2024::day3::parser;

pub fn main() {
    let input = include_str!("../../data/day3.txt");
    println!("{}", compute(input));
}

fn compute(input: &str) -> i64 {
    parser::parse_and_compute(input).unwrap().1
}

#[cfg(test)]
mod test {
    use crate::compute;

    #[test]
    fn test_compute() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let actual = compute(input);
        assert_eq!(actual, 48);
    }
}
