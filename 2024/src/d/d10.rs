use std::collections::HashMap;

use direction::{CardinalDirection, Coord};
use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[must_use]
fn get_input() -> HashMap<Coord, u32> {
    read_all::<String>("input_10")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(|(x, c)| (Coord::new(x as i32, y as i32), c.to_digit(10).unwrap()))
                .collect_vec()
        })
        .collect()
}

#[must_use]
pub fn d10() -> (usize, usize) {
    let field = get_input();

    let hiking_trails = field
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(trailhead, _)| calc_score(trailhead, &field))
        .collect_vec();

    (
        hiking_trails.iter().map(|e| e.len()).sum(),
        hiking_trails.iter().map(|e| e.values().sum::<usize>()).sum(),
    )
}

fn calc_score(position: &Coord, field: &HashMap<Coord, u32>) -> HashMap<Coord, usize> {
    let mut hiking_trails = HashMap::new();
    let v = match field.get(position) {
        Some(v) => v,
        None => return hiking_trails,
    };

    if *v == 9 {
        return HashMap::from([(*position, 1)]);
    }

    for direction in CardinalDirection::all() {
        if let Some((p, w)) = field.get_key_value(&(position + direction.coord())) {
            if *w == v + 1 {
                calc_score(p, field).iter().for_each(|(p, v)| {
                    hiking_trails
                        .entry(*p)
                        .and_modify(|e| *e += v)
                        .or_insert(*v);
                });
            }
        }
    }

    hiking_trails
}
