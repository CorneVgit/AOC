use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    string::String,
};

use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[must_use]
fn get_input() -> Vec<String> {
    read_all::<String>("input_5")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .collect()
}

#[must_use]
pub fn d5() -> (u64, u64) {
    let values = get_input();

    let mut iter = values.split(String::is_empty);

    let rules = iter.next().unwrap();
    let updates: Vec<Vec<u64>> = iter
        .next()
        .unwrap()
        .iter()
        .map(|u| u.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut m: HashMap<u64, HashSet<u64>> = HashMap::new();

    for rule in rules {
        let (left, right) = rule.split_once('|').unwrap();

        let left: u64 = left.parse().unwrap();
        let right: u64 = right.parse().unwrap();

        m.entry(left)
            .and_modify(|set| {
                set.insert(right);
            })
            .or_insert([right].into());
    }

    let (correct_updates, incorrect_updates): (Vec<Vec<u64>>, Vec<Vec<u64>>) =
        updates.into_iter().partition(|u| update_is_ordered(u, &m));

    let r1 = correct_updates.iter().fold(0, |a, u| a + u[u.len() / 2]);
    let r2 = incorrect_updates
        .iter()
        .map(|u| fix_incorrectly_ordered_update(u, &m))
        .fold(0, |a, u| a + u[u.len() / 2]);

    (r1, r2)
}

fn fix_incorrectly_ordered_update<'a>(
    u: &'a [u64],
    m: &'a HashMap<u64, HashSet<u64>>,
) -> Vec<&'a u64> {
    u.iter()
        .sorted_by(|a, b| {
            if m.contains_key(a) && m[a].contains(b) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .collect()
}

fn update_is_ordered(update: &[u64], m: &HashMap<u64, HashSet<u64>>) -> bool {
    for (i, n) in update.iter().enumerate() {
        if update[i..update.len()]
            .iter()
            .any(|x| m.contains_key(x) && m[x].contains(n))
        {
            return false;
        }
    }

    true
}
