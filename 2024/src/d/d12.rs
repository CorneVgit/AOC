use std::collections::{HashMap, HashSet};

use direction::{CardinalDirection, Coord, Direction};
use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;
use Direction::*;

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
    let mut r2 = 0;

    while let Some(p) = left_to_walk.iter().next() {
        let (walked, perimeter) = walk(field.get_key_value(p).unwrap(), HashSet::new(), &field);

        left_to_walk.retain(|p| !walked.contains(p));

        r1 += walked.len() * perimeter;
        r2 += walked.len() * count_sides(&walked);
    }

    (r1, r2)
}

fn walk(
    (position, c): (&Coord, &char),
    mut walked: HashSet<Coord>,
    field: &HashMap<Coord, char>,
) -> (HashSet<Coord>, usize) {
    let mut perimeter = 0;

    if walked.contains(position) {
        return (walked, perimeter);
    }

    walked.insert(*position);

    for direction in CardinalDirection::all() {
        let next_position = position + direction.coord();
        match field.get(&next_position) {
            Some(w) if c == w => {
                let (w, p) = walk((&next_position, c), walked.clone(), field);
                walked.extend(w);
                perimeter += p;
            }
            _ => perimeter += 1,
        };
    }

    (walked, perimeter)
}

fn count_sides(walked: &HashSet<Coord>) -> usize {
    let region = walked
        .iter()
        .sorted_unstable_by_key(|p| (p.y, p.x))
        .collect_vec();

    region.iter().map(|p| lookup(&region, p)).sum()
}

fn lookup(region: &[&Coord], p: &Coord) -> usize {
    let comb = (
        region.contains(&&(*p + NorthWest.coord())),
        region.contains(&&(*p + North.coord())),
        region.contains(&&(*p + NorthEast.coord())),
        region.contains(&&(*p + West.coord())),
        region.contains(&&(*p + East.coord())),
        region.contains(&&(*p + SouthWest.coord())),
        region.contains(&&(*p + South.coord())),
        region.contains(&&(*p + SouthEast.coord())),
    );

    match comb {
        (true, true, true, true, true, true, true, true) => 0,
        (true, true, true, true, true, true, true, false) => 0,
        (true, true, true, true, true, true, false, true) => 1,
        (true, true, true, true, true, true, false, false) => 1,
        (true, true, true, true, true, false, true, true) => 0,
        (true, true, true, true, true, false, true, false) => 0,
        (true, true, true, true, true, false, false, true) => 0,
        (true, true, true, true, true, false, false, false) => 0,
        (true, true, true, true, false, true, true, true) => 1,
        (true, true, true, true, false, true, true, false) => 1,
        (true, true, true, true, false, true, false, true) => 2,
        (true, true, true, true, false, true, false, false) => 2,
        (true, true, true, true, false, false, true, true) => 1,
        (true, true, true, true, false, false, true, false) => 1,
        (true, true, true, true, false, false, false, true) => 1,
        (true, true, true, true, false, false, false, false) => 1,
        (true, true, true, false, true, true, true, true) => 1,
        (true, true, true, false, true, true, true, false) => 1,
        (true, true, true, false, true, true, false, true) => 2,
        (true, true, true, false, true, true, false, false) => 2,
        (true, true, true, false, true, false, true, true) => 1,
        (true, true, true, false, true, false, true, false) => 1,
        (true, true, true, false, true, false, false, true) => 2,
        (true, true, true, false, true, false, false, false) => 2,
        (true, true, true, false, false, true, true, true) => 2,
        (true, true, true, false, false, true, true, false) => 2,
        (true, true, true, false, false, true, false, true) => 3,
        (true, true, true, false, false, true, false, false) => 3,
        (true, true, true, false, false, false, true, true) => 2,
        (true, true, true, false, false, false, true, false) => 2,
        (true, true, true, false, false, false, false, true) => 3,
        (true, true, true, false, false, false, false, false) => 3,
        (true, true, false, true, true, true, true, true) => 0,
        (true, true, false, true, true, true, true, false) => 0,
        (true, true, false, true, true, true, false, true) => 1,
        (true, true, false, true, true, true, false, false) => 1,
        (true, true, false, true, true, false, true, true) => 0,
        (true, true, false, true, true, false, true, false) => 0,
        (true, true, false, true, true, false, false, true) => 0,
        (true, true, false, true, true, false, false, false) => 0,
        (true, true, false, true, false, true, true, true) => 0,
        (true, true, false, true, false, true, true, false) => 0,
        (true, true, false, true, false, true, false, true) => 1,
        (true, true, false, true, false, true, false, false) => 1,
        (true, true, false, true, false, false, true, true) => 0,
        (true, true, false, true, false, false, true, false) => 0,
        (true, true, false, true, false, false, false, true) => 0,
        (true, true, false, true, false, false, false, false) => 0,
        (true, true, false, false, true, true, true, true) => 1,
        (true, true, false, false, true, true, true, false) => 1,
        (true, true, false, false, true, true, false, true) => 2,
        (true, true, false, false, true, true, false, false) => 2,
        (true, true, false, false, true, false, true, true) => 1,
        (true, true, false, false, true, false, true, false) => 1,
        (true, true, false, false, true, false, false, true) => 2,
        (true, true, false, false, true, false, false, false) => 2,
        (true, true, false, false, false, true, true, true) => 1,
        (true, true, false, false, false, true, true, false) => 1,
        (true, true, false, false, false, true, false, true) => 2,
        (true, true, false, false, false, true, false, false) => 2,
        (true, true, false, false, false, false, true, true) => 1,
        (true, true, false, false, false, false, true, false) => 1,
        (true, true, false, false, false, false, false, true) => 2,
        (true, true, false, false, false, false, false, false) => 2,
        (true, false, true, true, true, true, true, true) => 1,
        (true, false, true, true, true, true, true, false) => 1,
        (true, false, true, true, true, true, false, true) => 2,
        (true, false, true, true, true, true, false, false) => 2,
        (true, false, true, true, true, false, true, true) => 1,
        (true, false, true, true, true, false, true, false) => 1,
        (true, false, true, true, true, false, false, true) => 1,
        (true, false, true, true, true, false, false, false) => 1,
        (true, false, true, true, false, true, true, true) => 2,
        (true, false, true, true, false, true, true, false) => 2,
        (true, false, true, true, false, true, false, true) => 3,
        (true, false, true, true, false, true, false, false) => 3,
        (true, false, true, true, false, false, true, true) => 2,
        (true, false, true, true, false, false, true, false) => 2,
        (true, false, true, true, false, false, false, true) => 2,
        (true, false, true, true, false, false, false, false) => 2,
        (true, false, true, false, true, true, true, true) => 2,
        (true, false, true, false, true, true, true, false) => 2,
        (true, false, true, false, true, true, false, true) => 3,
        (true, false, true, false, true, true, false, false) => 3,
        (true, false, true, false, true, false, true, true) => 2,
        (true, false, true, false, true, false, true, false) => 2,
        (true, false, true, false, true, false, false, true) => 3,
        (true, false, true, false, true, false, false, false) => 3,
        (true, false, true, false, false, true, true, true) => 3,
        (true, false, true, false, false, true, true, false) => 3,
        (true, false, true, false, false, true, false, true) => 4,
        (true, false, true, false, false, true, false, false) => 4,
        (true, false, true, false, false, false, true, true) => 3,
        (true, false, true, false, false, false, true, false) => 3,
        (true, false, true, false, false, false, false, true) => 4,
        (true, false, true, false, false, false, false, false) => 4,
        (true, false, false, true, true, true, true, true) => 1,
        (true, false, false, true, true, true, true, false) => 1,
        (true, false, false, true, true, true, false, true) => 2,
        (true, false, false, true, true, true, false, false) => 2,
        (true, false, false, true, true, false, true, true) => 1,
        (true, false, false, true, true, false, true, false) => 1,
        (true, false, false, true, true, false, false, true) => 1,
        (true, false, false, true, true, false, false, false) => 1,
        (true, false, false, true, false, true, true, true) => 2,
        (true, false, false, true, false, true, true, false) => 2,
        (true, false, false, true, false, true, false, true) => 3,
        (true, false, false, true, false, true, false, false) => 3,
        (true, false, false, true, false, false, true, true) => 2,
        (true, false, false, true, false, false, true, false) => 2,
        (true, false, false, true, false, false, false, true) => 2,
        (true, false, false, true, false, false, false, false) => 2,
        (true, false, false, false, true, true, true, true) => 2,
        (true, false, false, false, true, true, true, false) => 2,
        (true, false, false, false, true, true, false, true) => 3,
        (true, false, false, false, true, true, false, false) => 3,
        (true, false, false, false, true, false, true, true) => 2,
        (true, false, false, false, true, false, true, false) => 2,
        (true, false, false, false, true, false, false, true) => 3,
        (true, false, false, false, true, false, false, false) => 3,
        (true, false, false, false, false, true, true, true) => 3,
        (true, false, false, false, false, true, true, false) => 3,
        (true, false, false, false, false, true, false, true) => 4,
        (true, false, false, false, false, true, false, false) => 4,
        (true, false, false, false, false, false, true, true) => 3,
        (true, false, false, false, false, false, true, false) => 3,
        (true, false, false, false, false, false, false, true) => 4,
        (true, false, false, false, false, false, false, false) => 4,
        (false, true, true, true, true, true, true, true) => 0,
        (false, true, true, true, true, true, true, false) => 0,
        (false, true, true, true, true, true, false, true) => 1,
        (false, true, true, true, true, true, false, false) => 1,
        (false, true, true, true, true, false, true, true) => 0,
        (false, true, true, true, true, false, true, false) => 0,
        (false, true, true, true, true, false, false, true) => 0,
        (false, true, true, true, true, false, false, false) => 0,
        (false, true, true, true, false, true, true, true) => 1,
        (false, true, true, true, false, true, true, false) => 1,
        (false, true, true, true, false, true, false, true) => 2,
        (false, true, true, true, false, true, false, false) => 2,
        (false, true, true, true, false, false, true, true) => 1,
        (false, true, true, true, false, false, true, false) => 1,
        (false, true, true, true, false, false, false, true) => 1,
        (false, true, true, true, false, false, false, false) => 1,
        (false, true, true, false, true, true, true, true) => 0,
        (false, true, true, false, true, true, true, false) => 0,
        (false, true, true, false, true, true, false, true) => 1,
        (false, true, true, false, true, true, false, false) => 1,
        (false, true, true, false, true, false, true, true) => 0,
        (false, true, true, false, true, false, true, false) => 0,
        (false, true, true, false, true, false, false, true) => 1,
        (false, true, true, false, true, false, false, false) => 1,
        (false, true, true, false, false, true, true, true) => 1,
        (false, true, true, false, false, true, true, false) => 1,
        (false, true, true, false, false, true, false, true) => 2,
        (false, true, true, false, false, true, false, false) => 2,
        (false, true, true, false, false, false, true, true) => 1,
        (false, true, true, false, false, false, true, false) => 1,
        (false, true, true, false, false, false, false, true) => 2,
        (false, true, true, false, false, false, false, false) => 2,
        (false, true, false, true, true, true, true, true) => 0,
        (false, true, false, true, true, true, true, false) => 0,
        (false, true, false, true, true, true, false, true) => 1,
        (false, true, false, true, true, true, false, false) => 1,
        (false, true, false, true, true, false, true, true) => 0,
        (false, true, false, true, true, false, true, false) => 0,
        (false, true, false, true, true, false, false, true) => 0,
        (false, true, false, true, true, false, false, false) => 0,
        (false, true, false, true, false, true, true, true) => 0,
        (false, true, false, true, false, true, true, false) => 0,
        (false, true, false, true, false, true, false, true) => 1,
        (false, true, false, true, false, true, false, false) => 1,
        (false, true, false, true, false, false, true, true) => 0,
        (false, true, false, true, false, false, true, false) => 0,
        (false, true, false, true, false, false, false, true) => 0,
        (false, true, false, true, false, false, false, false) => 0,
        (false, true, false, false, true, true, true, true) => 0,
        (false, true, false, false, true, true, true, false) => 0,
        (false, true, false, false, true, true, false, true) => 1,
        (false, true, false, false, true, true, false, false) => 1,
        (false, true, false, false, true, false, true, true) => 0,
        (false, true, false, false, true, false, true, false) => 0,
        (false, true, false, false, true, false, false, true) => 1,
        (false, true, false, false, true, false, false, false) => 1,
        (false, true, false, false, false, true, true, true) => 0,
        (false, true, false, false, false, true, true, false) => 0,
        (false, true, false, false, false, true, false, true) => 1,
        (false, true, false, false, false, true, false, false) => 1,
        (false, true, false, false, false, false, true, true) => 0,
        (false, true, false, false, false, false, true, false) => 0,
        (false, true, false, false, false, false, false, true) => 1,
        (false, true, false, false, false, false, false, false) => 1,
        (false, false, true, true, true, true, true, true) => 0,
        (false, false, true, true, true, true, true, false) => 0,
        (false, false, true, true, true, true, false, true) => 1,
        (false, false, true, true, true, true, false, false) => 1,
        (false, false, true, true, true, false, true, true) => 0,
        (false, false, true, true, true, false, true, false) => 0,
        (false, false, true, true, true, false, false, true) => 0,
        (false, false, true, true, true, false, false, false) => 0,
        (false, false, true, true, false, true, true, true) => 1,
        (false, false, true, true, false, true, true, false) => 1,
        (false, false, true, true, false, true, false, true) => 2,
        (false, false, true, true, false, true, false, false) => 2,
        (false, false, true, true, false, false, true, true) => 1,
        (false, false, true, true, false, false, true, false) => 1,
        (false, false, true, true, false, false, false, true) => 1,
        (false, false, true, true, false, false, false, false) => 1,
        (false, false, true, false, true, true, true, true) => 2,
        (false, false, true, false, true, true, true, false) => 2,
        (false, false, true, false, true, true, false, true) => 3,
        (false, false, true, false, true, true, false, false) => 3,
        (false, false, true, false, true, false, true, true) => 2,
        (false, false, true, false, true, false, true, false) => 2,
        (false, false, true, false, true, false, false, true) => 3,
        (false, false, true, false, true, false, false, false) => 3,
        (false, false, true, false, false, true, true, true) => 3,
        (false, false, true, false, false, true, true, false) => 3,
        (false, false, true, false, false, true, false, true) => 4,
        (false, false, true, false, false, true, false, false) => 4,
        (false, false, true, false, false, false, true, true) => 3,
        (false, false, true, false, false, false, true, false) => 3,
        (false, false, true, false, false, false, false, true) => 4,
        (false, false, true, false, false, false, false, false) => 4,
        (false, false, false, true, true, true, true, true) => 0,
        (false, false, false, true, true, true, true, false) => 0,
        (false, false, false, true, true, true, false, true) => 1,
        (false, false, false, true, true, true, false, false) => 1,
        (false, false, false, true, true, false, true, true) => 0,
        (false, false, false, true, true, false, true, false) => 0,
        (false, false, false, true, true, false, false, true) => 0,
        (false, false, false, true, true, false, false, false) => 0,
        (false, false, false, true, false, true, true, true) => 1,
        (false, false, false, true, false, true, true, false) => 1,
        (false, false, false, true, false, true, false, true) => 2,
        (false, false, false, true, false, true, false, false) => 2,
        (false, false, false, true, false, false, true, true) => 1,
        (false, false, false, true, false, false, true, false) => 1,
        (false, false, false, true, false, false, false, true) => 1,
        (false, false, false, true, false, false, false, false) => 1,
        (false, false, false, false, true, true, true, true) => 2,
        (false, false, false, false, true, true, true, false) => 2,
        (false, false, false, false, true, true, false, true) => 3,
        (false, false, false, false, true, true, false, false) => 3,
        (false, false, false, false, true, false, true, true) => 2,
        (false, false, false, false, true, false, true, false) => 2,
        (false, false, false, false, true, false, false, true) => 3,
        (false, false, false, false, true, false, false, false) => 3,
        (false, false, false, false, false, true, true, true) => 3,
        (false, false, false, false, false, true, true, false) => 3,
        (false, false, false, false, false, true, false, true) => 4,
        (false, false, false, false, false, true, false, false) => 4,
        (false, false, false, false, false, false, true, true) => 3,
        (false, false, false, false, false, false, true, false) => 3,
        (false, false, false, false, false, false, false, true) => 4,
        (false, false, false, false, false, false, false, false) => 4,
    }
}
