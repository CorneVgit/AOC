use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<String> {
    let result_values = read_all::<String>("input_6");

    result_values
        .into_iter()
        .map(|result_values| result_values.unwrap_infallible())
        .collect()
}

pub fn d6() -> (u64, u64) {
    let input = get_input();

    let times = input[0]
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();
    let distances = input[1]
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    let mut r1 = 1;

    for i in 0..4 {
        let time = times[i];
        let distance = distances[i];

        let mut r = 0;

        for t in 1..(time / 2) {
            let res = (time - t) * t;
            if res > distance {
                r = (time + 1) - t * 2;
                break;
            }
        }

        r1 *= r;
    }

    let time = input[0]
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = input[1]
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut r2 = 0;

    for t in 1..(time / 2) {
        let res = (time - t) * t;
        if res > distance {
            r2 = (time + 1) - t * 2;
            break;
        }
    }

    (r1, r2)
}
