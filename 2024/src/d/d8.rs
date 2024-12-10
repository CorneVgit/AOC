use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nalgebra::Point2;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[must_use]
fn get_input() -> HashMap<Point2<i64>, char> {
    read_all::<String>("input_8")
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

#[must_use]
pub fn d8() -> (usize, usize) {
    let field = get_input();

    let mut antennas: HashMap<char, HashSet<Point2<i64>>> = HashMap::new();

    for (location, frequency) in field.iter().filter(|e| !".#".contains(*e.1)) {
        antennas
            .entry(*frequency)
            .and_modify(|a| {
                a.insert(*location);
            })
            .or_insert(HashSet::from([*location]));
    }

    (
        count_antinodes(&antennas, &field, false),
        count_antinodes(&antennas, &field, true),
    )
}

fn count_antinodes(
    antennas: &HashMap<char, HashSet<nalgebra::OPoint<i64, nalgebra::Const<2>>>>,
    field: &HashMap<nalgebra::OPoint<i64, nalgebra::Const<2>>, char>,
    resonance: bool,
) -> usize {
    let mut antinodes = HashSet::new();

    for locations in antennas.values() {
        for loc_pair in locations.iter().combinations(2) {
            let dx = loc_pair[0].x - loc_pair[1].x;
            let dy = loc_pair[0].y - loc_pair[1].y;

            let mut next_a = loc_pair[0].xy();
            let mut next_b = loc_pair[1].xy();

            if !resonance {
                next_a = Point2::new(loc_pair[0].x + dx, loc_pair[0].y + dy);
                next_b = Point2::new(loc_pair[1].x - dx, loc_pair[1].y - dy);
            }

            loop {
                match (next_a, next_b) {
                    (a, b) if field.contains_key(&a) && field.contains_key(&b) => {
                        antinodes.extend([a, b])
                    }
                    (a, _) if field.contains_key(&a) => {
                        antinodes.insert(a);
                    }
                    (_, b) if field.contains_key(&b) => {
                        antinodes.insert(b);
                    }
                    _ => break,
                }

                if !resonance {
                    break;
                }

                next_a = Point2::new(next_a.x + dx, next_a.y + dy);
                next_b = Point2::new(next_b.x - dx, next_b.y - dy);
            }
        }
    }

    antinodes.len()
}
