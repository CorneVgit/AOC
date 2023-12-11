use std::collections::HashSet;

use itertools::Itertools;

use crate::util::read_all;

fn get_input() -> Vec<String> {
    let result_values = read_all::<String>("input_10_sample");

    result_values
        .into_iter()
        .map(unwrap_infallible::UnwrapInfallible::unwrap_infallible)
        .collect()
}

#[must_use]
pub fn d10() -> (u64, u64) {
    let input = get_input();

    let input = input
        .into_iter()
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    let mut start: (usize, usize) = (0, 0);

    'outer: for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'S' {
                start = (i, j);
                break 'outer;
            }
        }
    }

    let (r1, r2) = walk_path(start, &input);

    (r1, r2)
}

fn walk_path(start: (usize, usize), grid: &[Vec<char>]) -> (u64, u64) {
    let mut steps = 0;
    let mut count = 0;
    let mut pos = start;
    let mut dir = Dir::Unknown;

    let mut main_loop = HashSet::new();

    #[derive(PartialEq)]
    enum Dir {
        North,
        East,
        South,
        West,
        Unknown,
    }

    'init: {
        if pos.0 != 0 {
            if let Some(row) = grid.get(pos.0 - 1) {
                if let Some(v) = row.get(pos.1) {
                    if matches!(v, '|' | '7' | 'F') {
                        dir = Dir::South;
                        pos = (pos.0 - 1, pos.1);
                        break 'init;
                    }
                }
            }
        }
        if let Some(row) = grid.get(pos.0) {
            if let Some(v) = row.get(pos.1 + 1) {
                if matches!(v, '-' | '7' | 'J') {
                    dir = Dir::West;
                    pos = (pos.0, pos.1 + 1);
                    break 'init;
                }
            }
        }
        if let Some(row) = grid.get(pos.0 + 1) {
            if let Some(v) = row.get(pos.1) {
                if matches!(v, '|' | 'L' | 'J') {
                    dir = Dir::North;
                    pos = (pos.0 + 1, pos.1);
                    break 'init;
                }
            }
        }
        if pos.1 != 0 {
            if let Some(row) = grid.get(pos.0) {
                if let Some(v) = row.get(pos.1 - 1) {
                    if matches!(v, '-' | 'L' | 'F') {
                        dir = Dir::East;
                        pos = (pos.0, pos.1 - 1);
                        break 'init;
                    }
                }
            }
        }
    }

    loop {
        steps += 1;

        main_loop.insert(pos);

        if let Some(row) = grid.get(pos.0) {
            if let Some(v) = row.get(pos.1) {
                match v {
                    '-' if dir == Dir::East => {
                        pos = (pos.0, pos.1 - 1);
                        dir = Dir::East
                    }
                    '-' if dir == Dir::West => {
                        pos = (pos.0, pos.1 + 1);
                        dir = Dir::West
                    }
                    '|' if dir == Dir::North => {
                        pos = (pos.0 + 1, pos.1);
                        dir = Dir::North
                    }
                    '|' if dir == Dir::South => {
                        pos = (pos.0 - 1, pos.1);
                        dir = Dir::South
                    }
                    'F' if dir == Dir::East => {
                        pos = (pos.0 + 1, pos.1);
                        dir = Dir::North
                    }
                    'F' if dir == Dir::South => {
                        pos = (pos.0, pos.1 + 1);
                        dir = Dir::West
                    }
                    'J' if dir == Dir::North => {
                        pos = (pos.0, pos.1 - 1);
                        dir = Dir::East
                    }
                    'J' if dir == Dir::West => {
                        pos = (pos.0 - 1, pos.1);
                        dir = Dir::South
                    }
                    'L' if dir == Dir::North => {
                        pos = (pos.0, pos.1 + 1);
                        dir = Dir::West
                    }
                    'L' if dir == Dir::East => {
                        pos = (pos.0 - 1, pos.1);
                        dir = Dir::South
                    }
                    '7' if dir == Dir::South => {
                        pos = (pos.0, pos.1 - 1);
                        dir = Dir::East
                    }
                    '7' if dir == Dir::West => {
                        pos = (pos.0 + 1, pos.1);
                        dir = Dir::North
                    }
                    'S' => break,
                    _ => (),
                }
            }
        }
    }

    (steps / 2, count)
}
