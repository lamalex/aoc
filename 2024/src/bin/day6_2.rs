use std::collections::HashSet;

use aoc_2024::day6::{parser, Direction, Position};

pub fn main() {
    let input = include_str!("../../data/day6.txt");
    let res = compute(input);

    println!("{res}");
}

fn compute(input: &str) -> usize {
    let (_, world) = parser::world(input).unwrap();
    let mut candidates = HashSet::<Position>::new();

    let mut candidate_world = world.clone();
    loop {

        let start = *candidate_world.guard_location();
        let res = candidate_world.walk_guard();
        let guard_pos = match res {
            Ok(pos) => pos,
            Err(pos) => pos,
        };

        candidates.extend(start.to(&guard_pos));

        if res.is_err() {
            break;
        }
    }

    let mut count = 0;
    for candidate in candidates {
        let mut distinct = HashSet::<(Position, Direction)>::new();
        
        let mut world = world.clone();
        if world.place_obstacle(candidate).is_err() {
            continue;
        }

        loop {
            let res = world.walk_guard();
            
            // walked off the edge, no loop.
            if res.is_err() {
                break;
            }

            // this indicates a loop!
            if distinct.contains(&(*world.guard_location(), *world.guard_direction())) {
                count += 1;
                break;
            } else {
                distinct.insert((*world.guard_location(), *world.guard_direction()));
            }
        }
    }

    count
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
        assert_eq!(actual, 6);
    }
}