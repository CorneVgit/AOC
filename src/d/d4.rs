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

    let random_numbers: Vec<usize> = values
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    const BOARD_ROW_COUNT: usize = 5;
    const BOARD_COLUMN_COUNT: usize = 5;

    let mut boards: HashMap<usize, (Vec<(usize, bool)>, bool)> = HashMap::new();
    let mut results: Vec<usize> = Vec::new();

    for board_index in 0..values[1..].len() / 6 {
        for value in &values[board_index * 6 + 2..=(board_index + 1) * 6] {
            let row_numbers: Vec<(usize, bool)> = value
                .split_whitespace()
                .map(|x| (x.parse::<usize>().unwrap(), false))
                .collect();

            match boards.get_mut(&board_index) {
                Some(x) => x.0.append(&mut row_numbers.clone()),
                None => {
                    boards.insert(board_index, (row_numbers.clone(), false));
                }
            };
        }
    }

    for (i, random_number) in random_numbers.into_iter().enumerate() {
        for (_, board) in &mut boards {
            board.0.iter_mut().for_each(|(n, marked)| {
                if *n == random_number {
                    *marked = true;
                }
            });

            if i >= 5 {
                let mut marked_count = 0;
                for j in 0..BOARD_ROW_COUNT {
                    for (_, marked) in &board.0[j..j + 5] {
                        if *marked {
                            marked_count += 1;
                        }
                    }

                    if marked_count == 5 {
                        let s = board
                            .0
                            .iter()
                            .filter_map(|(n, marked)| match *marked {
                                false => Some(n),
                                true => None,
                            })
                            .sum::<usize>();

                        if board.1 == false {
                            results.push(random_number * s);
                        }

                        board.1 = true;
                        break;
                    }

                    marked_count = 0;
                }

                for j in 0..BOARD_COLUMN_COUNT {
                    for (_, marked) in board.0[j..].iter().step_by(5) {
                        if *marked {
                            marked_count += 1;
                        }
                    }

                    if marked_count == 5 {
                        let s = board
                            .0
                            .iter()
                            .filter_map(|(n, marked)| match *marked {
                                false => Some(n),
                                true => None,
                            })
                            .sum::<usize>();

                        if board.1 == false {
                            results.push(random_number * s);
                        }

                        board.1 = true;
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
