use aoc_2024::day4::{count_xmas, parser::parse_to_matrix};

pub fn main() {
    let input = include_str!("../../data/day4.txt");
    let matrix = parse_to_matrix(input);
    let answer = count_xmas(&matrix);

    println!("{answer}");
}

#[cfg(test)]
mod test {
    use aoc_2024::day4::{count_xmas, parser::parse_to_matrix};

    #[test]
    fn test_count_xmas() {
        let input = include_str!("../../data/day4.txt");
        let matrix = parse_to_matrix(input);
        let answer = count_xmas(&matrix);

        assert_eq!(answer, 2545);
    }
}
