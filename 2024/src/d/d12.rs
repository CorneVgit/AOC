use std::collections::{HashMap, HashSet};

use direction::{CardinalDirection, Coord};
use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[must_use]
fn get_input() -> HashMap<Coord, char> {
    read_all::<String>("input_12")
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
pub fn d12() -> (usize, usize) {
    let field = get_input();

    let mut left_to_walk: HashSet<&Coord> = field.keys().collect();
    let mut r1 = 0;

    while let Some(p) = left_to_walk.iter().next() {
        let walked = walk(field.get_key_value(p).unwrap(), HashMap::new(), &field);

        left_to_walk.retain(|p| !walked.contains_key(p));

        r1 += walked.len() * walked.values().sum::<usize>();
    }

    (r1, 0)
}

fn walk(
    (position, c): (&Coord, &char),
    mut walked: HashMap<Coord, usize>,
    field: &HashMap<Coord, char>,
) -> HashMap<Coord, usize> {
    if walked.keys().contains(position) {
        return walked;
    }

    walked.insert(*position, 0);

    for direction in CardinalDirection::all() {
        let next_position = position + direction.coord();
        match field.get(&next_position) {
            Some(w) if c == w => {
                walked.extend(walk((&next_position, c), walked.clone(), field));
            }
            _ => {
                walked.entry(*position).and_modify(|perimeter| *perimeter += 1);
            }
        };
    }

    walked
}
