use itertools::Itertools;

use crate::util::read_all;

fn get_values() -> Vec<Vec<u32>> {
    let result_values = read_all::<String>("input_2");

    result_values
        .into_iter()
        .map(unwrap_infallible::UnwrapInfallible::unwrap_infallible)
        .map(|v| {
            v.split_ascii_whitespace()
                .map(|x| str::parse(x).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[must_use]
pub fn d2() -> (usize, usize) {
    let reports = get_values();

    let r1 = reports.iter().filter(|report| is_safe(report, 0)).count();
    let r2 = reports.iter().filter(|report| is_safe(report, 1)).count();

    (r1, r2)
}

fn is_safe(report: &[u32], tolerance: usize) -> bool {
    'outer: for i in 0..report.len() {
        let r: Vec<&u32> = report[0..i]
            .into_iter()
            .chain(report[(i + tolerance)..report.len()].into_iter())
            .collect();

        if !r.is_sorted() && !r.iter().rev().is_sorted() {
            continue;
        }

        for (v1, v2) in r.into_iter().tuple_windows() {
            if !(1..=3).contains(&v1.abs_diff(*v2)) {
                continue 'outer;
            }
        }

        return true;
    }

    return false;
}
