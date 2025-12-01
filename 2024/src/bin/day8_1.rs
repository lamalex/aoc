use aoc_2024::day8::PointPair;
use itertools::Itertools;

pub fn main() {
    let input = include_str!("../../data/day8.txt");
    let antinodes = compute(input, (50, 50));

    println!("{antinodes}");
}

fn compute(input: &str, grid_size: (i64, i64)) -> usize {
    aoc_2024::day8::parser::parse(input)
        .into_iter()
        .flat_map(|(_, points)| {
            points.iter()
                .tuple_combinations::<(_, _)>()
                .map(|(a, b)| PointPair(a.clone(), b.clone()))
                .flat_map(|pair| pair.antinodes(grid_size))
                .filter(|point| point.x() >= 0 && point.y() >= 0 && point.x() < grid_size.0 && point.y() < grid_size.1)
                .collect::<Vec<_>>()
        })
        .unique()
        .count()
}

#[cfg(test)]
mod test {
    use crate::compute;

    #[test]
    fn test_solution() {
        let input = include_str!("../../data/day8.txt");
        let antinodes = compute(input, (50, 50));

        assert_eq!(antinodes, 376);
    }

    #[test]
    fn test_sample_input() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        let actual = compute(input, (12, 12));
        assert_eq!(actual, 14);
    }
}