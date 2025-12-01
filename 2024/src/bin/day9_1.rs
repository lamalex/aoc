use aoc_2024::day9::parser::parse;

pub fn main() {
    let input = include_str!("../../data/day9.txt");
    let res = compute(input);

    println!("{res}");
}

fn compute(input: &str) -> usize {
    let mut disk = parse(input);
    disk.defrag();
    disk.checksum()
}

#[cfg(test)]
mod test {
    use crate::compute;

    #[test]
    fn verify_not_broken() {
        let input = include_str!("../../data/day9.txt");
        let res = compute(input);
        assert_eq!(res, 6448989155953);
    }

    #[test]
    fn test_sample_input() {
        let input = "2333133121414131402";
        let actual = compute(input);

        assert_eq!(actual, 1928);
    }
}