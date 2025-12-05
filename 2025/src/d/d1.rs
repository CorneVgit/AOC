use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<(char, i32)> {
    read_all::<String>("input_1")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .map(|s| {
            let split = s.split_at(1);
            (
                split.0.chars().next().unwrap(),
                split.1.parse::<i32>().unwrap(),
            )
        })
        .collect_vec()
}

#[must_use]
pub fn d1() -> (i32, i32) {
    let rotations = get_input();

    let mut zeroes = 0;
    let mut zeroes_pos = 0;
    let mut pos = 50;

    for rotation in rotations {
        match rotation.0 {
            'L' => {
                if pos == 0 {
                    pos = 100;
                }
                zeroes += (100 - pos + rotation.1) / 100;
                pos -= rotation.1;
            }
            'R' => {
                zeroes += (pos + rotation.1) / 100;
                pos += rotation.1;
            }
            _ => (),
        }

        pos = pos.rem_euclid(100);

        if pos == 0 {
            zeroes_pos += 1;
        }
    }

    (zeroes_pos, zeroes)
}
