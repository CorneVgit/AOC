use std::collections::HashMap;

use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<(u128, Vec<u128>)> {
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
pub fn d7() -> (u128, u128) {
    let input = get_input();

    let mut r1 = 0;

    let mut h: HashMap<usize, Vec<OperatorSequence>> = HashMap::new();

    for n in 1..12 {
        h.insert(n, build_operators(n));
    }

    for (result, components) in &input {
        for ops in &h[&(components.len() - 1)] {
            let mut res = components[0];

            for i in 0..ops.len() {
                res = ops[i](res, components[i + 1]);
            }

            if &res == result {
                r1 += result;
                break;
            }
        }
    }

    let mut r2 = 0;

    let mut h: HashMap<usize, Vec<OperatorSequence>> = HashMap::new();

    for n in 1..12 {
        h.insert(n, build_operators_b(n));
    }

    for (result, components) in &input {
        for ops in &h[&(components.len() - 1)] {
            let mut res = components[0];

            for i in 0..ops.len() {
                res = ops[i](res, components[i + 1]);
            }

            if &res == result {
                r2 += result;
                break;
            }
        }
    }

    (r1, r2)
}

type OperatorSequence = Vec<fn(u128, u128) -> u128>;
fn build_operators(n: usize) -> Vec<OperatorSequence> {
    if n == 1 {
        return vec![vec![u128::wrapping_mul], vec![u128::wrapping_add]];
    }

    build_operators(n - 1)
        .into_iter()
        .map(|f| {
            let mut r = f;
            r.push(u128::wrapping_mul);
            r
        })
        .chain(build_operators(n - 1).into_iter().map(|f| {
            let mut r = f;
            r.push(u128::wrapping_add);
            r
        }))
        .collect()
}

fn build_operators_b(n: usize) -> Vec<OperatorSequence> {
    if n == 1 {
        return vec![vec![u128::wrapping_mul], vec![u128::wrapping_add], vec![conc]];
    }

    build_operators_b(n - 1)
        .into_iter()
        .map(|f| {
            let mut r = f;
            r.push(u128::wrapping_mul);
            r
        })
        .chain(build_operators_b(n - 1).into_iter().map(|f| {
            let mut r = f;
            r.push(u128::wrapping_add);
            r
        }))
        .chain(build_operators_b(n - 1).into_iter().map(|f| {
            let mut r = f;
            r.push(conc);
            r
        }))
        .collect()
}

fn conc(a: u128, b: u128) -> u128 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}
