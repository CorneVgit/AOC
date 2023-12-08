use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<(String, u64)> {
    let result_values = read_all::<String>("input_7");

    result_values
        .into_iter()
        .map(|result_values| result_values.unwrap_infallible())
        .map(|x| {
            let (a, b) = x.split_once(' ').unwrap();
            (a.to_owned(), b.parse::<u64>().unwrap())
        })
        .collect_vec()
}

pub fn d7() -> (u64, u64) {
    let input = get_input();

    let r1 = input
        .iter()
        .sorted_by_key(|a| get_hand_strength(&a.0))
        .enumerate()
        .map(|(i, v)| (i as u64 + 1) * v.1)
        .sum::<u64>();

    let r2 = input
        .iter()
        .sorted_by_key(|a| get_hand_strength_joker(&a.0))
        .enumerate()
        .map(|(i, v)| (i as u64 + 1) * v.1)
        .sum::<u64>();

    (r1, r2)
}

fn get_card_strength(c: char) -> u64 {
    match c {
        c if c.is_ascii_digit() => c.to_digit(10).unwrap().into(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 1,
    }
}

fn get_card_strength_joker(c: char) -> u64 {
    match c {
        c if c.is_ascii_digit() => c.to_digit(10).unwrap().into(),
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 1,
    }
}

fn get_hand_strength(h: &str) -> u64 {
    let m = h.chars().counts();
    let u = m.len();
    let max = m.into_values().max().unwrap();

    let strength: u64 = match u {
        1 => 70000000000000,             // Five of a kind
        2 if max == 4 => 60000000000000, // Four of a kind
        2 if max == 3 => 50000000000000, // Full house
        3 if max == 3 => 40000000000000, // Three of a kind
        3 if max == 2 => 30000000000000, // Two pair
        4 => 20000000000000,             // One pair
        _ => 10000000000000,             // High card
    };

    strength
        + h.chars()
            .rev()
            .enumerate()
            .map(|(i, v)| get_card_strength(v) * 100u64.pow((i + 1) as u32))
            .sum::<u64>()
}

fn get_hand_strength_joker(h: &str) -> u64 {
    let z: String = h.chars().filter(|c| c != &'J').collect();
    let m = z.chars().counts();
    let mut u = m.len();

    let values = m
        .into_iter()
        .sorted_by(|a, b| {
            (b.1, get_card_strength_joker(b.0)).cmp(&(a.1, get_card_strength_joker(a.0)))
        })
        .collect_vec();

    let max;
    if !values.is_empty() {
        max = values[0].1 + h.len() - z.len();
    } else {
        u = 1;
        max = 5;
    }

    let strength: u64 = match u {
        1 => 70000000000000,             // Five of a kind
        2 if max == 4 => 60000000000000, // Four of a kind
        2 if max == 3 => 50000000000000, // Full house
        3 if max == 3 => 40000000000000, // Three of a kind
        3 if max == 2 => 30000000000000, // Two pair
        4 => 20000000000000,             // One pair
        _ => 10000000000000,             // High card
    };

    strength
        + h.chars()
            .rev()
            .enumerate()
            .map(|(i, v)| get_card_strength_joker(v) * 100u64.pow((i + 1) as u32))
            .sum::<u64>()
}
