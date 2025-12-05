use std::collections::HashSet;

use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> (HashSet<(usize, usize)>, Vec<usize>) {
    let input = read_all::<String>("input_5")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .collect_vec();

    let (ranges, ids) = input.splitn(2, String::is_empty).next_tuple().unwrap();

    (
        optimize_ranges(
            ranges
                .iter()
                .map(|s| {
                    let split = s.split_once('-').unwrap();
                    (split.0.parse().unwrap(), split.1.parse().unwrap())
                })
                .collect(),
        ),
        ids.iter().map(|s| s.parse().unwrap()).collect(),
    )
}

#[must_use]
pub fn d5() -> (usize, usize) {
    let (ranges, ids) = get_input();

    let r1 = ids
        .iter()
        .filter(|id| {
            ranges
                .iter()
                .any(|range| range.0 <= **id && **id <= range.1)
        })
        .count();

    let r2 = ranges.iter().map(|range| range.1 - range.0 + 1).sum();

    (r1, r2)
}

fn optimize_ranges(mut ranges: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    loop {
        let mut new_ranges = ranges.clone();

        for r1 in &ranges {
            for r2 in &ranges {
                if r1 != r2 {
                    if r1.0 >= r2.0 && r1.1 <= r2.1 {
                        new_ranges.remove(r1);
                        break;
                    } else if r1.0 <= r2.0 && r1.1 >= r2.1 {
                        new_ranges.remove(r2);
                        break;
                    } else if (r1.0 >= r2.0 && r1.0 <= r2.1) || r2.1 + 1 == r1.0 {
                        new_ranges.remove(r1);
                        new_ranges.remove(r2);
                        new_ranges.insert((r2.0, r1.1));
                        break;
                    } else if (r1.0 <= r2.0 && r1.1 >= r2.0) || r1.1 + 1 == r2.0 {
                        new_ranges.remove(r1);
                        new_ranges.remove(r2);
                        new_ranges.insert((r1.0, r2.1));
                        break;
                    }
                }
            }
        }

        if ranges.len() == new_ranges.len() {
            return ranges;
        }

        ranges = new_ranges;
    }
}
