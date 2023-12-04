use std::collections::HashSet;

use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<(String, u32)> {
    let result_values = read_all::<String>("input_4");

    result_values
        .into_iter()
        .map(|result_values| (result_values.unwrap_infallible(), 1))
        .collect()
}

pub fn d4() -> (u32, u32) {
    let mut input = get_input();

    let mut counts: Vec<usize> = Vec::new();

    let mut r1 = 0;

    for line in &input {
        let (winning_numbers, numbers) = line
            .0
            .split_once(": ")
            .unwrap()
            .1
            .split_once(" | ")
            .unwrap();
        let numbers: HashSet<i32> = HashSet::from_iter(
            numbers
                .split_ascii_whitespace()
                .map(|n| n.trim().parse::<i32>().unwrap()),
        );
        let winning_numbers: HashSet<i32> = HashSet::from_iter(
            winning_numbers
                .split_ascii_whitespace()
                .map(|n| n.trim().parse::<i32>().unwrap()),
        );

        let count = winning_numbers.intersection(&numbers).count();
        counts.push(count);
        if count > 0 {
            r1 += 2u32.pow(count as u32 - 1);
        }
    }

    for i in 0..input.len() {
        for _ in 0..input[i].1 {
            input
                .iter_mut()
                .skip(i + 1)
                .take(counts[i])
                .for_each(|x| x.1 += 1);
        }
    }

    let r2 = input.into_iter().fold(0, |acc, e| acc + e.1);

    (r1, r2)
}
