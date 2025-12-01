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
                .collect::<Vec<_>>()
        })
        .unique()
        .sorted()
        // .sorted_by(|p1, p2| p1.y().cmp(&p2.y()))
        // .inspect(|p| { println!("{p:?}"); })
        .count()
}

#[cfg(test)]
mod test {
    use crate::compute;
    use test_case::test_case;


    #[test_case("00...", (5,1), 5)]
    #[test_case("0\n0\n.\n.\n.", (1, 5), 5)]
    #[test_case(".\n.\na\na\n.", (1, 5), 5)]
    #[test_case("0....\n.0...", (5, 5), 5)]
    #[test_case("....a\n...a.", (5, 5), 5)]
    fn test_contrinved_input(input: &str, grid_size: (i64, i64), expected: usize) {
        let actual = compute(input, grid_size);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_real_input_bound() {
        let input = include_str!("../../data/day8.txt");
        let antinodes = compute(input, (50, 50));
        assert_eq!(antinodes, 1352);
    }

     #[test]
        fn test_small_sample() {
            let input = r#"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
.........."#;

            let actual = compute(input, (10,10));
            assert_eq!(actual, 9);
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
        assert_eq!(actual, 34);

        r#"
$....$*...
...$......
.$....$...
.........$
..$.......
..........
...$......
..........
....$.....
..........
"#;
    }
}