use std::collections::HashSet;

use nalgebra::Point2;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<String> {
    read_all::<String>("input_9_example")
        .into_iter()
        .map(|rucksack| rucksack.unwrap_infallible())
        .collect()
}

pub fn d9() -> (usize, usize) {
    let values = values();

    let r1 = calc_num_of_visited_positions(&values, 2);
    let r2 = calc_num_of_visited_positions(&values, 10);

    (r1, r2)
}

fn calc_num_of_visited_positions(values: &[String], rope_len: usize) -> usize {
    let mut visited_positions = HashSet::<Point2<i32>>::default();

    let mut rope = vec![Point2::<i32>::origin(); rope_len];

    for (direction, distance) in values.iter().map(|v| v.split_once(' ').unwrap()) {
        let distance = distance.parse::<usize>().unwrap();
        let direction = match direction {
            "L" => Point2::<i32>::new(-1, 0),
            "R" => Point2::<i32>::new(1, 0),
            "U" => Point2::<i32>::new(0, 1),
            "D" => Point2::<i32>::new(0, -1),
            _ => panic!("Invalid direction"),
        };

        for _ in 0..distance {
            let mut i = 1;
            rope[0] += direction.coords;
            while i < rope.len() {
                let abs_dif = (rope[i - 1].coords - rope[i].coords).abs();

                if abs_dif.max() > 1 {
                    if abs_dif.sum() >= 2 {
                        rope[i] += direction.coords;
                    }
                    if abs_dif.sum() == 3 {
                        rope[i] = rope[i - 1] - direction.coords;
                    }
                }

                i += 1;
            }
            visited_positions.insert(*rope.last().unwrap());

            println!("{:?}", rope);
        }
    }

    visited_positions.len()
}
