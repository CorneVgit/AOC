use itertools::Itertools;

use crate::util::read_all;

fn get_input() -> Vec<Vec<i64>> {
    let result_values = read_all::<String>("input_9");

    result_values
        .into_iter()
        .map(unwrap_infallible::UnwrapInfallible::unwrap_infallible)
        .map(|r| {
            r.split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

#[must_use]
pub fn d9() -> (i64, i64) {
    let patterns = get_input();

    let (r1, r2) = patterns.iter().fold((0, 0), |acc, pattern| {
        let extrapolation = extrapolate_value(pattern);
        (
            acc.0 + pattern.last().unwrap() + extrapolation.0,
            acc.1 + pattern.first().unwrap() - extrapolation.1,
        )
    });

    (r1, r2)
}

fn extrapolate_value(pattern: &[i64]) -> (i64, i64) {
    let diffs = pattern
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    if diffs.iter().all(|v| v == &0) {
        (0, 0)
    } else {
        let extrapolation = extrapolate_value(&diffs);
        (
            diffs.last().unwrap() + extrapolation.0,
            diffs.first().unwrap() - extrapolation.1,
        )
    }
}
