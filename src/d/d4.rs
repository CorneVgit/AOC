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
        for value in &values[bingo_card_index * 6 + 2..=(bingo_card_index + 1) * 6] {
            let row_numbers: Vec<(u32, bool)> = value
                .split_whitespace()
                .map(|x| (x.parse::<u32>().unwrap(), false))
                .collect();

            match bingo_cards.get_mut(&bingo_card_index) {
                Some(x) => x.board.append(&mut row_numbers.clone()),
                None => {
                    bingo_cards.insert(
                        bingo_card_index,
                        BingoCard {
                            board: row_numbers.clone(),
                            won: false,
                        },
                    );
                }
            };
        }
    }

    for (i, random_number) in random_numbers.into_iter().enumerate() {
        for (_, bingo_card) in &mut bingo_cards {
            bingo_card.board.iter_mut().for_each(|(n, marked)| {
                if *n == random_number {
                    *marked = true;
                }
            });

            if i >= 5 {
                let mut marked_count = 0;
                for j in 0..BOARD_ROW_COUNT {
                    for (_, marked) in &bingo_card.board[j..j + 5] {
                        if *marked {
                            marked_count += 1;
                        }
                    }

                    if marked_count == 5 {
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

                    marked_count = 0;
                }

                for j in 0..BOARD_COLUMN_COUNT {
                    for (_, marked) in bingo_card.board[j..].iter().step_by(5) {
                        if *marked {
                            marked_count += 1;
                        }
                    }

                    if marked_count == 5 {
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

                    marked_count = 0;
                }
            }
        }
    }

    println!("{}", results.first().unwrap());
    println!("{}", results.last().unwrap());
}
