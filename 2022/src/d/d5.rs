use std::collections::VecDeque;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<String> {
    read_all::<String>("input_5")
        .into_iter()
        .map(|rucksack| rucksack.unwrap_infallible())
        .collect()
}

pub fn d5() -> (String, String) {
    let values = values();

    let (rearrangement_procedure, initial_container_columns) = init(&values);

    let mut container_columns = initial_container_columns.clone();
    let mut container_columns2 = initial_container_columns;

    for step in rearrangement_procedure {
        let (m, f, t) = parse_step(step);

        for n in 0..m {
            let container = container_columns[f].pop_back().unwrap();
            container_columns[t].push_back(container);

            let container = container_columns2[f].pop_back().unwrap();
            let containers_len = container_columns2[t].len();
            container_columns2[t].insert(containers_len - n, container);
        }
    }

    let r1 = container_columns.iter().fold("".to_string(), |acc, c| {
        acc + &c.back().unwrap().to_string()
    });

    let r2 = container_columns2.iter().fold("".to_string(), |acc, c| {
        acc + &c.back().unwrap().to_string()
    });

    (r1, r2)
}

fn parse_step(step: &str) -> (usize, usize, usize) {
    let mut step_split = step.split_ascii_whitespace();

    step_split.next().unwrap();
    // "move"
    let m = step_split.next().unwrap().parse().unwrap();
    step_split.next().unwrap();
    // "from"
    let f = step_split.next().unwrap().parse::<usize>().unwrap() - 1;
    step_split.next().unwrap();
    // "to"
    let t = step_split.next().unwrap().parse::<usize>().unwrap() - 1;

    (m, f, t)
}

fn init(
    values: &[String],
) -> (
    std::slice::Iter<'_, std::string::String>,
    Vec<VecDeque<char>>,
) {
    let mut value_iter = values.iter();

    let mut container_rows = Vec::new();
    for container_layer in value_iter.by_ref() {
        if container_layer.is_empty() {
            break;
        }

        container_rows.push(container_layer);
    }

    container_rows.reverse();
    let stack_count =
        container_rows[0].chars().fold(
            0,
            |count, c| {
                if c.is_ascii_digit() {
                    count + 1
                } else {
                    count
                }
            },
        );

    let mut container_columns = vec![VecDeque::new(); stack_count];
    for row in &container_rows[1..] {
        let row_iter = &mut row.chars();

        let mut n = 0;
        loop {
            // '[' or ' '
            row_iter.next().unwrap();

            // 'A-Z' or ' '
            let container = row_iter.next().unwrap();
            if container.is_ascii_uppercase() {
                container_columns[n].push_back(container);
            }

            // ']' or ' '
            row_iter.next().unwrap();

            // None or ' '
            if row_iter.next().is_none() {
                break;
            }

            n += 1;
        }
    }

    (value_iter, container_columns)
}
