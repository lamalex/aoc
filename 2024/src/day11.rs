use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct CacheKey {
    stone_value: u64,
    blink_number: usize,
    target: usize,
}

pub fn memoized_blink(stone_value: u64, target: usize) -> usize {
    let mut cache: HashMap<CacheKey, usize> = HashMap::new();
    blink_with_cache(stone_value, 0, target, &mut cache)
}

fn blink_with_cache(
    stone_value: u64,
    blink_number: usize,
    target: usize,
    cache: &mut HashMap<CacheKey, usize>,
) -> usize {
    let key = CacheKey {
        stone_value,
        blink_number,
        target,
    };

    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let result = match stone_value {
        0 => {
            if blink_number == target {
                1
            } else {
                blink_with_cache(1, blink_number + 1, target, cache)
            }
        }
        _ if format!("{stone_value}").len() % 2 == 0 => {
            if blink_number == target {
                1
            } else {
                let value_str = format!("{stone_value}");
                let left_digits = value_str[0..value_str.len() / 2].parse().unwrap();
                let right_digits = value_str[value_str.len() / 2..].parse().unwrap();
                blink_with_cache(left_digits, blink_number + 1, target, cache)
                    + blink_with_cache(right_digits, blink_number + 1, target, cache)
            }
        }
        _ => {
            if blink_number == target {
                1
            } else {
                blink_with_cache(stone_value * 2024, blink_number + 1, target, cache)
            }
        }
    };

    cache.insert(key, result);
    result
}

pub fn blink(stone_value: u64, blink_number: usize, target: usize) -> usize {
    match stone_value {
        0 => {
            if blink_number == target {
                1
            } else {
                blink(1, blink_number + 1, target)
            }
        },
        _ if format!("{stone_value}").len() % 2 == 0 => {
            if blink_number == target {
                1
            } else {
                let value_str = format!("{stone_value}");
                let left_digits = value_str[0..value_str.len() / 2].parse().unwrap();
                let right_digits = value_str[value_str.len() / 2..].parse().unwrap();
                blink(left_digits, blink_number + 1, target) + blink(right_digits, blink_number + 1, target)
            }
        },
        _ => {
            if blink_number == target {
                1
            } else {
                blink(stone_value * 2024, blink_number + 1, target)
            }
        }
    }
}

pub mod parser {
    pub fn parse(input: &str) -> Vec<u64> {
        input.split_whitespace().into_iter().map(|item| item.parse().unwrap()).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::day11::{blink, memoized_blink};

    #[test]
    fn test_sample_input() {
        let init = vec![125, 17];
        assert_eq!(init.into_iter().map(|stone| blink(stone, 0, 25)).sum::<usize>(), 55312)
    }

    #[test]
    fn test_pt2_sample_input() {
        let init = vec![125, 17];
        assert_eq!(init.into_iter().map(|stone| memoized_blink(stone, 25)).sum::<usize>(), 55312)
    }
}