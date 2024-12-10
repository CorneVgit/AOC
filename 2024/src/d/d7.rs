use std::collections::HashMap;

use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<(u64, Vec<u64>)> {
    read_all::<String>("input_7")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .map(|s| {
            let (k, v) = s.split_once(':').unwrap();
            (
                k.parse().unwrap(),
                v.split_whitespace().map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

#[must_use]
pub fn d7() -> (u64, u64) {
    let input = get_input();

    (
        count_result_of_valid_equations(&input, &vec![u64::wrapping_mul, u64::wrapping_add]),
        count_result_of_valid_equations(&input, &vec![u64::wrapping_mul, u64::wrapping_add, conc]),
    )
}

fn count_result_of_valid_equations(input: &Vec<(u64, Vec<u64>)>, ops: &OperatorSequence) -> u64 {
    let mut h: HashMap<usize, Vec<OperatorSequence>> = HashMap::new();

    for n in 1..12 {
        h.insert(n, build_operators(n, ops));
    }

    let mut r = 0;

    for (result, components) in input {
        for ops in &h[&(components.len() - 1)] {
            let mut res = components[0];

            for i in 0..ops.len() {
                res = ops[i](res, components[i + 1]);
            }

            if &res == result {
                r += result;
                break;
            }
        }
    }

    r
}

type OperatorSequence = Vec<fn(u64, u64) -> u64>;
fn build_operators(n: usize, ops: &OperatorSequence) -> Vec<OperatorSequence> {
    if ops.is_empty() {
        return vec![Vec::new()];
    }

    if n == 1 {
        return ops.iter().map(|op| vec![op.to_owned()]).collect();
    }

    ops.iter()
        .flat_map(|op| {
            build_operators(n - 1, ops)
                .into_iter()
                .map(|f| f.into_iter().chain(vec![op.to_owned()]).collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

fn conc(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}
