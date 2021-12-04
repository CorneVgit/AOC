use std::collections::HashMap;

use crate::util::read_all;

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

    const BOARD_ROW_COUNT: usize = 5;
    const BOARD_COLUMN_COUNT: usize = 5;

    struct BingoCard {
        board: Vec<(u32, bool)>,
        won: bool,
    }

    let mut bingo_cards: HashMap<usize, BingoCard> = HashMap::new();
    let mut results: Vec<u32> = Vec::new();

    for bingo_card_index in 0..values[1..].len() / 6 {
        let board: Vec<(u32, bool)> = values[bingo_card_index * 6 + 2..=(bingo_card_index + 1) * 6]
            .join(" ")
            .split_whitespace()
            .map(|n| (n.parse::<u32>().unwrap(), false))
            .collect();

        bingo_cards.insert(bingo_card_index, BingoCard { board, won: false });
    }

    for (i, random_number) in random_numbers.into_iter().enumerate() {
        for (_, bingo_card) in &mut bingo_cards {
            for (n, marked) in bingo_card.board.iter_mut() {
                if *n == random_number {
                    *marked = true;
                }
            }

            if i >= 5 {
                for row_index in 0..BOARD_ROW_COUNT {
                    if bingo_card.board[row_index..row_index + BOARD_COLUMN_COUNT]
                        .iter()
                        .filter(|(_, marked)| *marked)
                        .collect::<Vec<&(u32, bool)>>()
                        .len()
                        == 5
                    {
                        let sum: u32 = bingo_card
                            .board
                            .iter()
                            .filter_map(|(n, marked)| match *marked {
                                false => Some(n),
                                true => None,
                            })
                            .sum();

                        if bingo_card.won == false {
                            results.push(random_number * sum);
                        }

                        bingo_card.won = true;
                        break;
                    }
                }

                for column_index in 0..BOARD_COLUMN_COUNT {
                    if bingo_card.board[column_index..]
                        .iter()
                        .step_by(BOARD_COLUMN_COUNT)
                        .filter(|(_, marked)| *marked)
                        .collect::<Vec<&(u32, bool)>>()
                        .len()
                        == 5
                    {
                        let sum: u32 = bingo_card
                            .board
                            .iter()
                            .filter_map(|(n, marked)| match *marked {
                                false => Some(n),
                                true => None,
                            })
                            .sum();

                        if bingo_card.won == false {
                            results.push(random_number * sum);
                        }

                        bingo_card.won = true;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", results.first().unwrap());
    println!("{}", results.last().unwrap());
}
