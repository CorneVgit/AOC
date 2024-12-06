use std::collections::HashMap;

use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> (Vec<u32>, Vec<u32>) {
    read_all::<String>("input_1")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .map(|s| {
            let mut split = s.split_ascii_whitespace();
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect()
}

#[must_use]
pub fn d1() -> (u32, u32) {
    let (mut v1, mut v2) = get_input();

    v1.sort_unstable();
    v2.sort_unstable();

    let r1 = v1.iter().zip(&v2).fold(0, |a, (x, y)| a + x.abs_diff(*y));

    let m: HashMap<&u32, usize> = v2
        .iter()
        .dedup_with_count()
        // group by value instead of count
        .into_group_map_by(|km| km.1)
        .into_iter()
        .map(|(value, count)| {
            (
                value,
                // extract count out of the single element vector
                count.into_iter().fold(0, |acc, (count, _)| acc + count),
            )
        })
        .collect();

    let r2 = v1
        .iter()
        .filter(|v| m.contains_key(v))
        .fold(0, |a, v| m[v] as u32 * v + a);

    (r1, r2)
}
