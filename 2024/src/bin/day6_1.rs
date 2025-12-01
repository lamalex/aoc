use std::collections::HashSet;

use aoc_2024::day6::{parser, Position};

pub fn main() {
    let input = include_str!("../../data/day6.txt");
    let res = compute(input);

    println!("{res}");
}

fn compute(input: &str) -> usize {
    let (_, mut world) = parser::world(input).unwrap();
    let mut distinct = HashSet::<Position>::from([
        *world.guard_location()
    ]);

    loop {
        let start = *world.guard_location();
        let res = world.walk_guard();
        let guard_pos = match res {
            Ok(pos) => pos,
            Err(pos) => pos,
        };

        distinct.extend(start.to(&guard_pos));

        if res.is_err() {
            break;
        }
    }

    return distinct.len()
}

#[cfg(test)]
mod test {

    #[test]
    fn test_sample_input() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
        let actual = super::compute(input);
        assert_eq!(actual, 41);
    }
}