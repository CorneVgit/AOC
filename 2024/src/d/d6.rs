use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nalgebra::Point2;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[must_use]
fn get_input() -> HashMap<Point2<i64>, char> {
    read_all::<String>("input_6")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .enumerate()
        .flat_map(|(y, v)| {
            v.chars()
                .enumerate()
                .map(|(x, w)| (Point2::new(x as i64, y as i64), w))
                .collect_vec()
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[must_use]
pub fn d6() -> (usize, usize) {
    let field = get_input();

    let mut visited: HashMap<Point2<i64>, Direction> = HashMap::new();
    let mut new_obstacles: HashSet<Point2<i64>> = HashSet::new();
    let start_position = match find_guard(&field) {
        Some(g) => g,
        None => panic!("No guard!"),
    };
    let mut current_position = start_position;
    let mut direction = Direction::North;

    loop {
        visited.insert(current_position, direction);

        let potential_next_position = get_next_position(current_position, &direction);

        match field.get(&potential_next_position) {
            Some(v) => {
                if *v == '#' {
                    direction = get_next_direction(&direction);
                    current_position = get_next_position(current_position, &direction);
                } else {
                    if check_if_loop(current_position, direction, &field, &visited) {
                        // println!("{potential_next_position:?}");
                        new_obstacles.insert(potential_next_position);
                    }

                    current_position = potential_next_position;
                }
            }
            None => break,
        }
    }

    new_obstacles.remove(&start_position);

    let r1 = visited.len();
    let r2 = new_obstacles.len();

    (r1, r2)
}

fn find_guard(field: &HashMap<Point2<i64>, char>) -> Option<Point2<i64>> {
    for (p, v) in field {
        if *v == '^' {
            return Some(*p);
        }
    }

    None
}

fn get_next_direction(dir: &Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn get_next_position(current_position: Point2<i64>, direction: &Direction) -> Point2<i64> {
    match direction {
        Direction::North => (current_position.coords + Point2::new(0, -1).coords).into(),
        Direction::East => (current_position.coords + Point2::new(1, 0).coords).into(),
        Direction::South => (current_position.coords + Point2::new(0, 1).coords).into(),
        Direction::West => (current_position.coords + Point2::new(-1, 0).coords).into(),
    }
}

fn check_if_loop(
    current_position_: Point2<i64>,
    direction_: Direction,
    field: &HashMap<Point2<i64>, char>,
    visited: &HashMap<Point2<i64>, Direction>,
) -> bool {
    let mut direction = get_next_direction(&direction_);
    let mut current_position = current_position_;

    if let Some(v) = field.get(&get_next_position(current_position, &direction)) {
        if *v == '#' {
            return false;
        }
    }

    loop {
        let potential_next_position = get_next_position(current_position, &direction);

        if direction == direction_ {
            return false;
        }

        match field.get(&potential_next_position) {
            Some(v) => {
                if *v == '#' {
                    if visited.contains_key(&current_position)
                        && visited[&current_position] == direction
                    {
                        return true;
                    }
                    direction = get_next_direction(&direction);
                    current_position = get_next_position(current_position, &direction);
                } else {
                    current_position = potential_next_position;
                }
            }
            None => break,
        }

        if current_position == current_position_ {
            return true;
        }
    }


    // loop {
    //     let potential_next_position = get_next_position(current_position, &direction);

    //     match field.get(&potential_next_position) {
    //         Some(v) => {
    //             if *v == '#' {
    //                 return visited.contains_key(&current_position) && visited[&current_position] == direction;
    //             } else {
    //                 current_position = potential_next_position;
    //             }
    //         }
    //         None => break,
    //     }
    // }

    false
}
