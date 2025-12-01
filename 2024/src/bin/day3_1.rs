use aoc_2024::day3::parser;


pub fn main() {
    let input = include_str!("../../data/day3.txt");
    println!("{}", compute(input));
}

fn compute(input: &str) -> i64 {
    parser::parse_muls(input)
        .unwrap()
        .1
        .iter()
        .map(|(a, b)| a * b)
        .sum::<i64>()
}

#[cfg(test)]
mod test {
    use crate::compute;

    #[test]
    fn test_compute() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let actual = compute(input);
        assert_eq!(actual, 161);
    }
}