use std::collections::HashMap;

use crate::util::read_all;

const BOARD_ROW_COUNT: usize = 5;
const BOARD_COLUMN_COUNT: usize = 5;

struct BingoCard {
    board: Vec<(u32, bool)>,
    won: bool,
}

fn d4values() -> Vec<String> {
    let result_values = read_all::<String>("input_4");
    let mut values: Vec<String> = Vec::new();

    for result_value in result_values {
        let value = match result_value {
            Ok(x) => x,
            _ => continue,
        };
        values.push(String::from(value));
    }

    values
}

pub fn d4ab() {
    let values = d4values();

    let random_numbers: Vec<u32> = values
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut bingo_cards: HashMap<usize, BingoCard> = HashMap::new();
    let mut results: Vec<u32> = Vec::new();

    initialize_bingo_cards(values, &mut bingo_cards);

    for (random_number_count, random_number) in random_numbers.into_iter().enumerate() {
        for (_, bingo_card) in &mut bingo_cards {
            mark_bingo_card(bingo_card, random_number);

            if random_number_count >= 5 {
                if let Some(r) = check_rows(bingo_card, random_number) {
                    results.push(r);
                } else if let Some(r) = check_columns(bingo_card, random_number) {
                    results.push(r)
                }
            }
        }

        if random_number_count >= 5 {
            bingo_cards.retain(|_, bingo_card| !bingo_card.won == true)
        }
    }

    println!("{}", results.first().unwrap());
    println!("{}", results.last().unwrap());
}

fn mark_bingo_card(bingo_card: &mut BingoCard, random_number: u32) {
    for (n, marked) in bingo_card.board.iter_mut() {
        if *n == random_number {
            *marked = true;
        }
    }
}

fn initialize_bingo_cards(values: Vec<String>, bingo_cards: &mut HashMap<usize, BingoCard>) {
    for bingo_card_index in 0..values[1..].len() / 6 {
        let board: Vec<(u32, bool)> = values[bingo_card_index * 6 + 2..=(bingo_card_index + 1) * 6]
            .join(" ")
            .split_whitespace()
            .map(|n| (n.parse::<u32>().unwrap(), false))
            .collect();

        bingo_cards.insert(bingo_card_index, BingoCard { board, won: false });
    }
}

fn check_rows(bingo_card: &mut BingoCard, random_number: u32) -> Option<u32> {
    for row_index in 0..BOARD_ROW_COUNT {
        if bingo_card.board[row_index..row_index + BOARD_COLUMN_COUNT]
            .iter()
            .filter(|(_, marked)| *marked)
            .collect::<Vec<&(u32, bool)>>()
            .len()
            == 5
        {
            return calculate_result(bingo_card, random_number);
        }
    }

    return None;
}

fn check_columns(bingo_card: &mut BingoCard, random_number: u32) -> Option<u32> {
    for column_index in 0..BOARD_COLUMN_COUNT {
        if bingo_card.board[column_index..]
            .iter()
            .step_by(BOARD_COLUMN_COUNT)
            .filter(|(_, marked)| *marked)
            .collect::<Vec<&(u32, bool)>>()
            .len()
            == 5
        {
            return calculate_result(bingo_card, random_number);
        }
    }

    return None;
}

fn calculate_result(bingo_card: &mut BingoCard, random_number: u32) -> Option<u32> {
    let sum: u32 = bingo_card
        .board
        .iter()
        .filter_map(|(n, marked)| match *marked {
            false => Some(n),
            true => None,
        })
        .sum();

    bingo_card.won = true;
    return Some(random_number * sum);
}
