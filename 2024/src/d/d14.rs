use direction::Coord;
use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[must_use]
fn get_input() -> Vec<(Coord, Coord)> {
    read_all::<String>("input_14")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .map(|s| {
            let (p, v) = s.split_ascii_whitespace().collect_tuple().unwrap();
            let p = p[2..].split_once(',').unwrap();
            let v = v[2..].split_once(',').unwrap();

            (
                Coord::new(p.0.parse().unwrap(), p.1.parse().unwrap()),
                Coord::new(v.0.parse().unwrap(), v.1.parse().unwrap()),
            )
        })
        .collect()
}

const ROOM_SIZE: Coord = Coord { x: 101, y: 103 };

#[must_use]
pub fn d14() -> (usize, usize) {
    let mut input = get_input();

    for i in 0..100 {
        for (p, v) in input.iter_mut() {
            p.x = (p.x + v.x).rem_euclid(ROOM_SIZE.x);
            p.y = (p.y + v.y).rem_euclid(ROOM_SIZE.y);
        }
    }

    let r1 = input
        .iter()
        .filter(|(p, _)| p.x < ROOM_SIZE.x / 2 && p.y < ROOM_SIZE.y / 2)
        .count()
        * input
            .iter()
            .filter(|(p, _)| p.x < ROOM_SIZE.x / 2 && p.y > ROOM_SIZE.y / 2)
            .count()
        * input
            .iter()
            .filter(|(p, _)| p.x > ROOM_SIZE.x / 2 && p.y < ROOM_SIZE.y / 2)
            .count()
        * input
            .iter()
            .filter(|(p, _)| p.x > ROOM_SIZE.x / 2 && p.y > ROOM_SIZE.y / 2)
            .count();

    (r1, 0)
}

