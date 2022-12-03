use itertools::Itertools;
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref VALS: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
}

fn find_common_pt1(line: &str) -> Option<char> {
    let midpoint = line.len() / 2;
    let compartment_1: HashSet<char> = HashSet::from_iter((&line[0..midpoint]).chars());
    let compartment_2: HashSet<char> = HashSet::from_iter((&line[midpoint..]).chars());

    compartment_1.intersection(&compartment_2).next().cloned()
}

fn find_common_pt2(line_1: &str, line_2: &str, line_3: &str) -> Option<char> {
    let line_1: HashSet<char> = HashSet::from_iter(line_1.chars());
    let line_2: HashSet<char> = HashSet::from_iter(line_2.chars());
    let line_3: HashSet<char> = HashSet::from_iter(line_3.chars());

    line_1
        .intersection(&line_2)
        .cloned()
        .collect::<HashSet<char>>()
        .intersection(&line_3)
        .next()
        .cloned()
}

fn main() {
    let data = include_str!("input.txt");

    let (sum_2, sum_1) = data
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(line_1, line_2, line_3)| {
            (
                find_common_pt2(line_1, line_2, line_3).into_prio_val(),
                [line_1, line_2, line_3]
                    .into_iter()
                    .map(find_common_pt1)
                    .map(Option::into_prio_val)
                    .sum::<usize>(),
            )
        })
        .reduce(|acc, next| (acc.0 + next.0, acc.1 + next.1))
        .unwrap();

    println!("pt1: {sum_1}");
    println!("pt2: {sum_2}");
}
trait IntoPrioVal {
    fn into_prio_val(self) -> usize;
}

impl IntoPrioVal for Option<char> {
    fn into_prio_val(self) -> usize {
        self.map(|c| VALS.iter().position(|e| *e == c).unwrap() + 1)
            .unwrap_or_default()
    }
}
