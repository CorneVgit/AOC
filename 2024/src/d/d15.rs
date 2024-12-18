use core::panic;
use std::{
    collections::{HashMap, HashSet},
    string::String,
};

use direction::{Axis, CardinalDirection, Coord};
use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Object {
    Robot,
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
}

impl Object {
    fn new(c: &char) -> Self {
        match c {
            '@' => Self::Robot,
            '.' => Self::Empty,
            '#' => Self::Wall,
            'O' => Self::BoxLeft,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            _ => panic!("Invalid object"),
        }
    }
}

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

    let (warehouse, moves) = get_normal_warehouse_and_moves(&values);
    let r1 = sum_coordinates_warehouse(warehouse, moves, false);

    let (warehouse, moves) = get_expanded_warehouse_and_moves(&values);
    let r2 = sum_coordinates_warehouse(warehouse, moves, true);

    (r1, r2)
}

fn find_robot(warehouse: &HashMap<Coord, Object>) -> Coord {
    *warehouse
        .iter()
        .find(|(_, object)| **object == Object::Robot)
        .expect("No robot!")
        .0
}

fn get_normal_warehouse_and_moves(
    values: &[String],
) -> (HashMap<Coord, Object>, Vec<CardinalDirection>) {
    let mut iter = values.split(String::is_empty);

    (
        iter.next()
            .unwrap()
            .iter()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(|(x, c)| (Coord::new(x as i32, y as i32), Object::new(&c)))
                    .collect_vec()
            })
            .collect(),
        iter.next()
            .unwrap()
            .iter()
            .flat_map(|s| {
                s.chars()
                    .map(|c| match c {
                        '^' => CardinalDirection::North,
                        '>' => CardinalDirection::East,
                        'v' => CardinalDirection::South,
                        '<' => CardinalDirection::West,
                        _ => panic!("Invalid move"),
                    })
                    .collect_vec()
            })
            .collect(),
    )
}

fn get_expanded_warehouse_and_moves(
    values: &[String],
) -> (HashMap<Coord, Object>, Vec<CardinalDirection>) {
    let mut iter = values.split(String::is_empty);

    (
        iter.next()
            .unwrap()
            .iter()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .flat_map(|(x, c)| match Object::new(&c) {
                        Object::Wall => [
                            (Coord::new((x * 2) as i32, y as i32), Object::Wall),
                            (Coord::new((x * 2 + 1) as i32, y as i32), Object::Wall),
                        ],
                        Object::Empty => [
                            (Coord::new((x * 2) as i32, y as i32), Object::Empty),
                            (Coord::new((x * 2 + 1) as i32, y as i32), Object::Empty),
                        ],
                        Object::BoxLeft => [
                            (Coord::new((x * 2) as i32, y as i32), Object::BoxLeft),
                            (Coord::new((x * 2 + 1) as i32, y as i32), Object::BoxRight),
                        ],
                        Object::Robot => [
                            (Coord::new((x * 2) as i32, y as i32), Object::Robot),
                            (Coord::new((x * 2 + 1) as i32, y as i32), Object::Empty),
                        ],
                        _ => panic!("This shouldn't happen: match c => {c}"),
                    })
                    .collect_vec()
            })
            .collect(),
        iter.next()
            .unwrap()
            .iter()
            .flat_map(|s| {
                s.chars()
                    .map(|c| match c {
                        '^' => CardinalDirection::North,
                        '>' => CardinalDirection::East,
                        'v' => CardinalDirection::South,
                        '<' => CardinalDirection::West,
                        _ => panic!("Invalid move"),
                    })
                    .collect_vec()
            })
            .collect(),
    )
}

fn sum_coordinates_warehouse(
    mut warehouse: HashMap<Coord, Object>,
    moves: Vec<CardinalDirection>,
    expanded: bool,
) -> i32 {
    let mut current_position = find_robot(&warehouse);

    for m in moves {
        update_warehouse(&mut warehouse, &mut current_position, m, expanded)
    }

    warehouse
        .iter()
        .filter(|(_, c)| **c == Object::BoxLeft)
        .map(|(p, _)| p.y * 100 + p.x)
        .sum()
}

fn update_warehouse(
    warehouse: &mut HashMap<Coord, Object>,
    robot_position: &mut Coord,
    direction: CardinalDirection,
    expanded: bool,
) {
    if !expanded || direction.axis() == Axis::X {
        update_normal_warehouse(warehouse, robot_position, direction);

        return;
    }

    let mut next_positions = HashSet::from([*robot_position]);
    let mut positions: HashSet<Coord> = HashSet::new();

    loop {
        if next_positions.is_empty() {
            break;
        }

        let current_positions = next_positions.clone();
        positions.extend(&current_positions);
        next_positions.clear();

        for current_position in current_positions {
            let next_position = current_position + direction.coord();

            match warehouse.get(&next_position) {
                Some(Object::BoxLeft) => {
                    next_positions.insert(next_position);
                    next_positions.insert(next_position + CardinalDirection::East.coord());
                }
                Some(Object::BoxRight) => {
                    next_positions.insert(next_position);
                    next_positions.insert(next_position + CardinalDirection::West.coord());
                }
                Some(Object::Empty) => continue,
                // Has to be a wall
                _ => return,
            };
        }
    }

    let old_warehouse = warehouse.clone();

    for current_position in &positions {
        warehouse
            .entry(*current_position)
            .and_modify(|current| *current = Object::Empty);
    }

    for current_position in &positions {
        let next_position = current_position + direction.coord();
        let current = old_warehouse[current_position];

        warehouse
            .entry(next_position)
            .and_modify(|next| *next = current);
    }

    *robot_position += direction.coord();
}

fn update_normal_warehouse(
    warehouse: &mut HashMap<Coord, Object>,
    robot_position: &mut Coord,
    direction: CardinalDirection,
) {
    let mut current_position = *robot_position;

    loop {
        let next_position = current_position + direction.coord();

        match warehouse.get(&next_position) {
            Some(Object::Empty) => {
                let mut current_position = next_position;

                loop {
                    let previous_position = current_position - direction.coord();
                    let prev = warehouse[&previous_position];

                    warehouse
                        .entry(current_position)
                        .and_modify(|current| *current = prev);
                    current_position = previous_position;

                    if current_position == *robot_position {
                        warehouse
                            .entry(*robot_position)
                            .and_modify(|current| *current = Object::Empty);
                        *robot_position += direction.coord();

                        return;
                    }
                }
            }
            Some(Object::Wall) => return,
            // Has to be a box
            _ => current_position = next_position,
        }
    }
}
