use std::cmp::Reverse;

fn main() {
    let data = include_str!("input.txt");
    let mut data: Vec<u64> = data
        .split("\n\n")
        .map(|e| e.split('\n').map(|e| e.parse::<u64>().unwrap()).sum())
        .collect();

    data.sort_unstable_by_key(|e| Reverse(*e));

    println!("pt 1 {}", data.first().unwrap());
    println!("pt 2 {}", data.iter().take(3).sum::<u64>());
}
