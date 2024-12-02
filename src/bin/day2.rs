use aoc_2024::day2::parser::parse_line_safety;

pub fn main() {
    let input = include_str!("../../data/day2.txt");

    let x = input
        .lines()
        .flat_map(|line| parse_line_safety(line, true))
        .filter(|&(_, safe)| safe)
        .count();

    print!("{x}");
}
