use aoc_2024::day3::parser;

pub fn main() {
    let input = include_str!("../../data/day3.txt");

    let res = parser::parse_muls(input)
        .unwrap()
        .1
        .iter()
        .map(|(a, b)| a * b)
        .sum::<i64>();

    println!("{res}");
}