use core::panic;
use std::{
    collections::{HashMap, HashSet},
    string::String,
};

use direction::{Axis, CardinalDirection, Coord};
use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[must_use]
fn get_input() -> Vec<String> {
    read_all::<String>("input_15")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .collect()
}

#[must_use]
pub fn d15() -> (i32, i32) {
    let values = get_input();

    let (field, moves) = get_normal_field_and_moves(&values);
    let r1 = sum_coordinates_normal_field(field, moves);

    let (field, moves) = get_expanded_field_and_moves(&values);
    let r2 = sum_coordinates_expanded_field(field, moves);

    (r1, r2)
}

fn sum_coordinates_normal_field(mut field: HashMap<Coord, char>, moves: Vec<char>) -> i32 {
    let mut current_position = find_robot(&field);

    for m in &moves {
        match m {
            '^' => update_normal_field(&mut field, &mut current_position, CardinalDirection::North),
            '>' => update_normal_field(&mut field, &mut current_position, CardinalDirection::East),
            'v' => update_normal_field(&mut field, &mut current_position, CardinalDirection::South),
            '<' => update_normal_field(&mut field, &mut current_position, CardinalDirection::West),
            _ => (),
        }
    }

    field
        .iter()
        .filter(|(_, c)| **c == 'O')
        .map(|(p, _)| p.y * 100 + p.x)
        .sum()
}

fn sum_coordinates_expanded_field(mut field: HashMap<Coord, char>, moves: Vec<char>) -> i32 {
    let mut current_position = find_robot(&field);

    for m in &moves {
        match m {
            '^' => {
                update_expanded_field(&mut field, &mut current_position, CardinalDirection::North)
            }
            '>' => {
                update_expanded_field(&mut field, &mut current_position, CardinalDirection::East)
            }
            'v' => {
                update_expanded_field(&mut field, &mut current_position, CardinalDirection::South)
            }
            '<' => {
                update_expanded_field(&mut field, &mut current_position, CardinalDirection::West)
            }
            _ => (),
        }
    }

    field
        .iter()
        .filter(|(_, c)| **c == '[')
        .map(|(p, _)| p.y * 100 + p.x)
        .sum()
}

fn get_normal_field_and_moves(values: &[String]) -> (HashMap<Coord, char>, Vec<char>) {
    let mut iter = values.split(String::is_empty);

    let field: HashMap<Coord, char> = iter
        .next()
        .unwrap()
        .iter()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(|(x, c)| (Coord::new(x as i32, y as i32), c))
                .collect_vec()
        })
        .collect();
    let moves: Vec<char> = iter
        .next()
        .unwrap()
        .iter()
        .flat_map(|s| s.chars().collect_vec())
        .collect();

    (field, moves)
}

fn get_expanded_field_and_moves(values: &[String]) -> (HashMap<Coord, char>, Vec<char>) {
    let mut iter = values.split(String::is_empty);

    let field: HashMap<Coord, char> = iter
        .next()
        .unwrap()
        .iter()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .flat_map(|(x, c)| match c {
                    '#' => [
                        (Coord::new((x * 2) as i32, y as i32), '#'),
                        (Coord::new((x * 2 + 1) as i32, y as i32), '#'),
                    ],
                    '.' => [
                        (Coord::new((x * 2) as i32, y as i32), '.'),
                        (Coord::new((x * 2 + 1) as i32, y as i32), '.'),
                    ],
                    'O' => [
                        (Coord::new((x * 2) as i32, y as i32), '['),
                        (Coord::new((x * 2 + 1) as i32, y as i32), ']'),
                    ],
                    '@' => [
                        (Coord::new((x * 2) as i32, y as i32), '@'),
                        (Coord::new((x * 2 + 1) as i32, y as i32), '.'),
                    ],
                    _ => panic!("This shouldn't happen: match c => {c}"),
                })
                .collect_vec()
        })
        .collect();
    let moves: Vec<char> = iter
        .next()
        .unwrap()
        .iter()
        .flat_map(|s| s.chars().collect_vec())
        .collect();

    (field, moves)
}

fn find_robot(field: &HashMap<Coord, char>) -> Coord {
    for (p, v) in field {
        if *v == '@' {
            return *p;
        }
    }

    panic!("No robot!")
}

fn update_normal_field(
    field: &mut HashMap<Coord, char>,
    robot_position: &mut Coord,
    direction: CardinalDirection,
) {
    let mut current_position = *robot_position;
    loop {
        let next_position = current_position + direction.coord();
        match field.get(&next_position) {
            Some(next) if "O[]".contains(*next) => current_position = next_position,
            Some(next) if *next == '#' => return,
            Some(next) if *next == '.' => {
                let mut current_position = next_position;

                loop {
                    let previous_position = current_position - direction.coord();
                    let p = field[&previous_position];

                    field.entry(current_position).and_modify(|e| *e = p);
                    current_position = previous_position;

                    if current_position == *robot_position {
                        field.entry(*robot_position).and_modify(|e| *e = '.');
                        *robot_position += direction.coord();

                        return;
                    }
                }
            }
            Some(next) => panic!("This shouldn't happen: field.get(&next_position) => {next}"),
            None => panic!("This shouldn't happen: field.get(&next_position) => None"),
        }
    }
}

fn update_expanded_field(
    field: &mut HashMap<Coord, char>,
    robot_position: &mut Coord,
    direction: CardinalDirection,
) {
    if direction.axis() == Axis::X {
        update_normal_field(field, robot_position, direction);
        return;
    }

    let mut begin_positions = vec![*robot_position];
    let mut next_positions = begin_positions.clone();
    let mut positions: HashSet<Coord> = HashSet::new();
    loop {
        if next_positions.is_empty() {
            break;
        }

        let mut current_positions = next_positions.clone();
        positions.extend(&current_positions);
        next_positions.clear();

        for current_position in &mut current_positions {
            let next_position = *current_position + direction.coord();
            match field.get(&next_position) {
                Some(next) if *next == '[' => match field.get(current_position) {
                    Some(current) if *current == '[' => {
                        next_positions.push(next_position);
                    }
                    Some(current) if *current == ']' => {
                        if !begin_positions
                            .iter()
                            .any(|e| e.x == (next_position.x + CardinalDirection::East.coord().x))
                        {
                            begin_positions.push(next_position + CardinalDirection::East.coord());
                        }
                        next_positions.push(next_position);
                        next_positions.push(next_position + CardinalDirection::East.coord());
                    }
                    Some(current) if *current == '@' => {
                        if !begin_positions
                            .iter()
                            .any(|e| e.x == (next_position.x + CardinalDirection::East.coord().x))
                        {
                            begin_positions.push(next_position + CardinalDirection::East.coord());
                        }
                        next_positions.push(next_position);
                        next_positions.push(next_position + CardinalDirection::East.coord());
                    }
                    Some(current) => {
                        panic!("This shouldn't happen: field.get(&position) => {current}")
                    }
                    None => panic!("This shouldn't happen: field.get(&position) => None"),
                },
                Some(next) if *next == ']' => match field.get(current_position) {
                    Some(current) if *current == ']' => {
                        next_positions.push(next_position);
                    }
                    Some(current) if *current == '[' => {
                        if !begin_positions
                            .iter()
                            .any(|e| e.x == (next_position.x + CardinalDirection::West.coord().x))
                        {
                            begin_positions.push(next_position + CardinalDirection::West.coord());
                        }
                        next_positions.push(next_position);
                        next_positions.push(next_position + CardinalDirection::West.coord());
                    }
                    Some(current) if *current == '@' => {
                        if !begin_positions
                            .iter()
                            .any(|e| e.x == (next_position.x + CardinalDirection::West.coord().x))
                        {
                            begin_positions.push(next_position + CardinalDirection::West.coord());
                        }
                        next_positions.push(next_position);
                        next_positions.push(next_position + CardinalDirection::West.coord());
                    }
                    Some(current) => {
                        panic!("This shouldn't happen: field.get(&position) => {current}")
                    }
                    None => panic!("This shouldn't happen: field.get(&position) => None"),
                },
                Some(next) if *next == '#' => return,
                Some(next) if *next == '.' => (),
                Some(next) => panic!("This shouldn't happen: field.get(&next_position) => {next}"),
                None => panic!("This shouldn't happen: field.get(&next_position) => None"),
            }
        }
    }

    let old_field = field.clone();

    for current_position in &positions {
        field.entry(*current_position).and_modify(|e| *e = '.');
    }

    for current_position in &positions {
        let next_position = current_position + direction.coord();
        let current = old_field[current_position];

        field.entry(next_position).and_modify(|e| *e = current);
    }

    *robot_position += direction.coord();
}

#[allow(dead_code)]
fn print_field(field: &HashMap<Coord, char>) {
    let f = field
        .iter()
        .sorted_unstable_by_key(|(p, _)| (p.y, p.x))
        .collect_vec();
    let field_width = f.first().unwrap().0.x.abs_diff(f.last().unwrap().0.x) as usize + 1;

    for (i, (_, c)) in f.iter().enumerate() {
        print!("{c}");
        if (i + 1) % field_width == 0 {
            println!();
        }
    }
}
