use itertools::Itertools;
use num::signum;

use crate::util::read_all;

fn get_values() -> Vec<Vec<u32>> {
    let result_values = read_all::<String>("input_2_sample");

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
    let mut error_count = 0;

    let mut sig = signum(report[0] as i64 - report[1] as i64);

    for (v1, v2) in report.iter().tuple_() {
        let new_sig = signum(*v1 as i64 - *v2 as i64);

        if !(1..=3).contains(&v1.abs_diff(*v2)) || new_sig != sig {
            error_count += 1;

            if error_count > tolerance {
                return false;
            }
        }

        sig = new_sig;
    }

    true
}
