use std::collections::{HashMap, HashSet};

use direction::{CardinalDirection, Coord};
use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[must_use]
fn get_input() -> HashMap<Coord, char> {
    read_all::<String>("input_6")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(|(x, c)| (Coord::new(x as i32, y as i32), c))
                .collect_vec()
        })
        .collect()
}

#[must_use]
pub fn d6() -> (usize, usize) {
    let field = get_input();

    let mut visited: HashMap<Coord, CardinalDirection> = HashMap::new();
    let mut new_obstacles: HashSet<Coord> = HashSet::new();
    let start_position = match find_guard(&field) {
        Some(g) => g,
        None => panic!("No guard!"),
    };
    let mut current_position = start_position;
    let mut direction = CardinalDirection::North;

    loop {
        visited.insert(current_position, direction);

        let potential_next_position = current_position + direction.coord();

        match field.get(&potential_next_position) {
            Some(v) => {
                if *v == '#' {
                    direction = direction.right90();
                    current_position += direction.coord();
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

fn find_guard(field: &HashMap<Coord, char>) -> Option<Coord> {
    for (p, v) in field {
        if *v == '^' {
            return Some(*p);
        }
    }

    None
}

fn check_if_loop(
    current_position_: Coord,
    direction_: CardinalDirection,
    field: &HashMap<Coord, char>,
    visited: &HashMap<Coord, CardinalDirection>,
) -> bool {
    let mut direction = direction_.right90();
    let mut current_position = current_position_;

    if let Some(v) = field.get(&(current_position + direction.coord())) {
        if *v == '#' {
            return false;
        }
    }

    loop {
        let potential_next_position = current_position + direction.coord();

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
                    direction = direction.right90();
                    current_position += direction.coord();
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
