use std::collections::HashSet;
use std::collections::VecDeque;

pub fn trailhead_ratings(map: &[Vec<u32>]) -> Vec<u32> {
    let start_nodes: VecDeque<(usize, usize)> = map.into_iter().enumerate()
        .flat_map(|(y, line)| line.into_iter().enumerate()
            .filter_map(|(x, value)| if *value == 0 { Some((y, x)) } else { None })
            .collect::<Vec<_>>()
        )
        .collect();

    start_nodes.into_iter().map(|next_start| {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        count_paths(map, next_start, &mut visited)
    })
    .collect::<Vec<u32>>()
}

fn count_paths(map: &[Vec<u32>], node: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> u32 {
    let node_value = map[node.0][node.1];

    if node_value == 9 {
        return 1;
    }

    let mut path_count = 0;
    visited.insert(node);

    if node.0 > 0 && map[node.0 - 1][node.1].checked_sub(node_value) == Some(1) && !visited.contains(&(node.0 - 1, node.1)) {
        path_count += count_paths(map, (node.0 - 1, node.1), visited)
    }

    if node.0 < map.len() - 1 && map[node.0 + 1][node.1].checked_sub(node_value) == Some(1) && !visited.contains(&(node.0 + 1, node.1)) {
        path_count += count_paths(map, (node.0 + 1, node.1), visited)
    }

    if node.1 > 0 && map[node.0][node.1 - 1].checked_sub(node_value) == Some(1) && !visited.contains(&(node.0, node.1 - 1)) {
        path_count += count_paths(map, (node.0, node.1 - 1), visited)
    }

    if node.1 < map[node.0].len() - 1 && map[node.0][node.1 + 1].checked_sub(node_value) == Some(1) && !visited.contains(&(node.0, node.1 + 1)) {
        path_count += count_paths(map, (node.0, node.1 + 1), visited)
    }

    visited.remove(&node);
    path_count
}

pub fn trailhead_scores(map: &[Vec<u32>]) -> Vec<u32> {
    let start_nodes: VecDeque<(usize, usize)> = map.into_iter().enumerate()
        .flat_map(|(y, line)| line.into_iter().enumerate()
            .filter_map(|(x, value)| if *value == 0 { Some((y, x)) } else { None })
            .collect::<Vec<_>>()
        )
        .collect();

    start_nodes.into_iter().map(|next_start| {
            let mut count = 0;
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            let mut queue: VecDeque<(usize, usize)> = VecDeque::from([
                next_start
            ]);

            while let Some(node) = queue.pop_front() {
                let node_value = map[node.0][node.1];
                if visited.contains(&node) {
                    continue;
                }

                visited.insert(node);

                if node_value == 9 {
                    count += 1;
                    continue;
                }

                if node.0 > 0 && map[node.0 - 1][node.1].checked_sub(node_value) == Some(1) {
                    queue.push_back((node.0 - 1, node.1));
                }

                if node.0 < map.len() - 1 && map[node.0 + 1][node.1].checked_sub(node_value) == Some(1) {
                    queue.push_back((node.0 + 1, node.1));
                }

                if node.1 > 0 && map[node.0][node.1 - 1].checked_sub(node_value) == Some(1) {
                    queue.push_back((node.0, node.1 - 1));
                }

                if node.1 < map[node.0].len() - 1 && map[node.0][node.1 + 1].checked_sub(node_value) == Some(1) {
                    queue.push_back((node.0, node.1 + 1));
                }
            }

            count
        }
    )
    .collect()

}

#[cfg(test)]
mod test {
    use crate::day10::trailhead_ratings;

    use super::{trailhead_scores, parser::parse};

    #[test]
    fn test_sample_input_pt2() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        let map = parse(input);
        let trailhead_ratings = trailhead_ratings(&map[..]);

        println!("{trailhead_ratings:?}");
        assert_eq!(trailhead_ratings.len(), 9);
        assert_eq!(trailhead_ratings, vec![20, 24, 10, 4, 1, 4, 5, 8, 5]);
    }

    #[test]
    fn test_sample_input_pt1() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        let map = parse(input);
        let trailhead_scores = trailhead_scores(&map[..]);

        println!("{trailhead_scores:?}");
        assert_eq!(trailhead_scores.len(), 9);
        assert_eq!(trailhead_scores, vec![5, 6, 5, 3, 1, 3, 5, 3, 5]);
        assert_eq!(36u32, trailhead_scores.into_iter().sum());
    }
}


pub mod parser {
    pub fn parse(input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .map(|line| line.chars().flat_map(|c| c.to_digit(10)).collect())
            .collect()
    }
    #[cfg(test)]
    mod test {}
}
