use std::collections::{HashMap, HashSet};

use direction::{CardinalDirection, Coord, Direction};
use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;
use Direction::*;

use crate::util::read_all;

#[must_use]
fn get_input() -> HashMap<Coord, char> {
    read_all::<String>("input_12_sample")
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

    let mut sides = 0;

    for p in &region {
        if !region.contains(&&(*p + North.coord()))
            && !region.contains(&&(*p + East.coord()))
            && !region.contains(&&(*p + South.coord()))
            && !region.contains(&&(*p + West.coord()))
        {
            sides += 4;
        }
        if !region.contains(&&(*p + North.coord()))
            && region.contains(&&(*p + East.coord()))
            && !region.contains(&&(*p + South.coord()))
            && !region.contains(&&(*p + West.coord()))
        {
            sides += 3;
        }
        if !region.contains(&&(*p + North.coord()))
            && !region.contains(&&(*p + East.coord()))
            && region.contains(&&(*p + South.coord()))
            && !region.contains(&&(*p + West.coord()))
        {
            sides += 3;
        }
        if !region.contains(&&(*p + North.coord()))
            && region.contains(&&(*p + East.coord()))
            && region.contains(&&(*p + South.coord()))
            && !region.contains(&&(*p + West.coord()))
        {
            sides += 2;
        }
        if !region.contains(&&(*p + South.coord()))
            && region.contains(&&(*p + West.coord()))
            && region.contains(&&(*p + SouthWest.coord()))
        {
            sides += 1;
        }
        if !region.contains(&&(*p + East.coord())) && !region.contains(&&(*p + North.coord())) {
            sides += 1;
        }
        if !region.contains(&&(*p + West.coord()))
            && !region.contains(&&(*p + East.coord()))
            && region.contains(&&(*p + SouthWest.coord()))
        {
            sides += 1;
        }
        if region.contains(&&(*p + North.coord()))
            && region.contains(&&(*p + East.coord()))
            && !region.contains(&&(*p + South.coord()))
            && !region.contains(&&(*p + West.coord()))
        {
            sides += 1;
        }
        if region.contains(&&(*p + North.coord()))
            && !region.contains(&&(*p + East.coord()))
            && !region.contains(&&(*p + South.coord()))
            && !region.contains(&&(*p + West.coord()))
        {
            sides += 2;
        }
        if region.contains(&&(*p + North.coord()))
            && region.contains(&&(*p + NorthEast.coord()))
            && !region.contains(&&(*p + East.coord()))
        {
            sides += 1;
        }
        if !region.contains(&&(*p + North.coord()))
            && region.contains(&&(*p + NorthWest.coord()))
            && region.contains(&&(*p + West.coord()))
            && region.contains(&&(*p + SouthWest.coord()))
            && !region.contains(&&(*p + South.coord()))
        {
            sides += 2;
        }
        if !region.contains(&&(*p + North.coord()))
            && region.contains(&&(*p + NorthWest.coord()))
            && region.contains(&&(*p + West.coord()))
            && region.contains(&&(*p + South.coord()))
        {
            sides += 1;
        }
    }

    sides
}
