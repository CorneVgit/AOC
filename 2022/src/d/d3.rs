use itertools::Itertools;
use sets::{SType, Set};
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<String> {
    let result_values = read_all::<String>("input_3");
    let mut rucksacks: Vec<String> = Vec::new();

    for result_value in result_values {
        let rucksack = result_value.unwrap_infallible();
        rucksacks.push(rucksack);
    }

    rucksacks
}

pub fn d3() -> (u32, u32) {
    let rucksacks = values();

    let mut total_priority: u32 = 0;

    for rucksack in &rucksacks {
        let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);

        let intersection: Set<u8> = Set::new(SType::Unordered, comp1.as_bytes(), true)
            .intersection(&Set::new(SType::Unordered, comp2.as_bytes(), true));

        total_priority += calculate_priority(intersection.data.first().unwrap()) as u32;
    }

    let r1 = total_priority;

    let mut total_priority: u32 = 0;

    for chunk in &rucksacks.into_iter().chunks(3) {
        let sets = chunk
            .map(|rucksack| Set::new(SType::Unordered, rucksack.as_bytes(), true))
            .collect::<Vec<Set<u8>>>();

        let intersection = sets
            .iter()
            .fold(sets[0].to_owned(), |acc, s| acc.intersection(s));

        total_priority += calculate_priority(intersection.data.first().unwrap()) as u32;
    }

    let r2 = total_priority;

    (r1, r2)
}

fn calculate_priority(c: &u8) -> u8 {
    if c.is_ascii_uppercase() {
        c - 64u8 + 26u8
    } else if c.is_ascii_lowercase() {
        c - 96u8
    } else {
        panic!("Invalid character");
    }
}
