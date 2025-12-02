use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util::read_single_string;

fn get_input() -> Vec<(u64, u64)> {
    read_single_string("input_2")
        .trim()
        .split(',')
        .map(|s| {
            let split = s.split_once('-').unwrap();
            (
                split.0.parse::<u64>().unwrap(),
                split.1.parse::<u64>().unwrap(),
            )
        })
        .collect_vec()
}

#[must_use]
pub fn d2() -> (u64, u64) {
    let ranges = get_input();

    let mut invalid_ids = Vec::new();
    let mut more_invalid_ids = Vec::new();

    let lut: HashMap<usize, HashSet<usize>> = (1..=20)
        .map(|n| {
            let mut divisors = HashSet::new();

            for v in 2..=((n as f64).sqrt() as usize) {
                if n % v == 0 {
                    divisors.insert(n / v);
                    divisors.insert(v);
                }
            }

            divisors.insert(1);

            (n, divisors)
        })
        .collect();

    for range in ranges {
        for n in range.0..=range.1 {
            let s = n.to_string();

            if s.len() % 2 == 0 {
                let split = s.split_at(s.len() / 2);

                if split.0 == split.1 {
                    invalid_ids.push(n);
                }
            }

            for v in &lut[&s.len()] {
                if s.chars()
                    .chunks(*v)
                    .into_iter()
                    .map(String::from_iter)
                    .all_equal()
                {
                    more_invalid_ids.push(n);
                    break;
                }
            }
        }
    }

    (invalid_ids.iter().sum(), more_invalid_ids.iter().sum())
}
