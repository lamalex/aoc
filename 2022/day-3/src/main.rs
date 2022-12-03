use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let data = include_str!("input.txt");

    let vals: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

    let sum = data
        .lines()
        .tuples::<(_, _, _)>()
        .map(|line| {
            let line_1: HashSet<char> = HashSet::from_iter(line.0.chars());
            let line_2: HashSet<char> = HashSet::from_iter(line.1.chars());
            let line_3: HashSet<char> = HashSet::from_iter(line.2.chars());

            let common = line_1
                .intersection(&line_2)
                .cloned()
                .collect::<HashSet<char>>()
                .intersection(&line_3)
                .next()
                .cloned();

            dbg!(&common);
            if let Some(common) = common {
                vals.iter().position(|e| *e == common).unwrap() + 1
            } else {
                0
            }
        })
        .reduce(|acc, next| acc + next)
        .unwrap();

    println!("{sum}");
}
