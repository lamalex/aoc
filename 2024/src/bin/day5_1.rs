use aoc_2024::day5::{self, Page};

pub fn main() {
    let input = include_str!("../../data/day5.txt");
    let res = compute(input);

    println!("{res}");
}

fn compute(input: &str) -> u32 {
    let (_, (rules, updates)) = day5::parser::input_sections(input).unwrap();
    updates.into_iter()
        .map(|update| {
            (
                update.len(),
                update.into_iter()
                    .map(|pg| Page::new(pg, &rules))
                    .collect::<Vec<_>>()
            )
        })
        .filter(|(_, update)| update.is_sorted())
        .map(|(len, update)| update[len/2])
        .map(|page| page.number())
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use crate::compute;

    #[test]
    fn test() {let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    let v = compute(input);

    assert_eq!(v, 144);
    }
}