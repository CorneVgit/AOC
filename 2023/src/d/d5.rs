use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<String> {
    let result_values = read_all::<String>("input_5");

    result_values
        .into_iter()
        .map(|result_values| result_values.unwrap_infallible())
        .collect()
}

pub fn d5() -> (u64, u64) {
    let input = get_input();
    let input = input.split(|p| p.is_empty()).collect_vec();

    let seeds = input[0][0]
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect_vec();

    let input = input[1..]
        .iter()
        .map(|map| {
            map[1..]
                .iter()
                .map(|range| {
                    range
                        .split_ascii_whitespace()
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let mut r1 = u64::MAX;

    for seed in &seeds {
        let location = calc_location(*seed, &input);

        if location < r1 {
            r1 = location;
        }
    }

    let mut r2 = u64::MAX;

    for seed_range in seeds.chunks(2).map(|x| (x[0], x[1])).collect_vec() {
        for seed in seed_range.0..(seed_range.0 + seed_range.1) {
            let location = calc_location(seed, &input);

            if location < r2 {
                r2 = location;
            }
        }
    }

    (r1, r2)
}

fn calc_location(seed: u64, input: &Vec<Vec<Vec<u64>>>) -> u64 {
    let mut location = seed;
    for map in input {
        for range in map {
            if (range[1]..(range[1] + range[2])).contains(&location) {
                let index = location - range[1];
                location = range[0] + index;
                break;
            }
        }
    }

    location
}
