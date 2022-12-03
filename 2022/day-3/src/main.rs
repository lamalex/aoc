use std::collections::HashSet;
fn main() {
    let data = include_str!("input.txt");

    let mut vals: Vec<char> = ('a'..='z').collect::<Vec<char>>();
    vals.extend('A'..='Z');

    let sum = data
        .lines()
        .map(|line| {
            let len = line.len();
            let part_a = &line[0..len / 2];
            let part_b = &line[len / 2..];

            let part_a: HashSet<char> = HashSet::from_iter(part_a.chars());
            let part_b: HashSet<char> = HashSet::from_iter(part_b.chars());
            dbg!(&part_a);
            dbg!(&part_b);
            
            let common = part_a.intersection(&part_b).next();
            dbg!(&common);
            if let Some(common) = common {
                vals.iter().position(|e| e == common).unwrap() + 1
            } else {
                0
            }
                
        })
        .reduce(|acc, next| acc + next)
        .unwrap();

    println!("{sum}");
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn does_it_work() {
        let h: HashSet<char> = HashSet::from_iter("omg".chars());

        let expected: HashSet<char> = vec!['o', 'm', 'g'].into_iter().collect();
        assert_eq!(expected, h);

        let mut vals: Vec<char> = ('a'..'z').collect::<Vec<char>>();
        vals.extend('A'..'Z');
        assert_eq!(vec!['a', 'b'], vals);
    }
}
