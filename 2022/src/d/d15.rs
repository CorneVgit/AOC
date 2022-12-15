use std::collections::HashSet;

use nalgebra::Point2;
use regex::Regex;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<String> {
    read_all::<String>("input_15")
        .into_iter()
        .map(|value| value.unwrap_infallible())
        .collect()
}

pub fn d15() -> (usize, i64) {
    let values = values();

    let re = Regex::new(r"[^-\d]*(-?\d+)[^-\d]*(-?\d+)[^-\d]*(-?\d+)[^-\d]*(-?\d+)").unwrap();

    let mut sensors = Vec::<(i32, i32, i32)>::new();

    for value in &values {
        match re.captures(value) {
            Some(c) => {
                let sensor_x: i32 = c.get(1).unwrap().as_str().parse().unwrap();
                let sensor_y: i32 = c.get(2).unwrap().as_str().parse().unwrap();
                let beacon_x: i32 = c.get(3).unwrap().as_str().parse().unwrap();
                let beacon_y: i32 = c.get(4).unwrap().as_str().parse().unwrap();
                let distance: i32 = TryInto::<i32>::try_into(
                    sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y),
                )
                .unwrap();

                sensors.push((sensor_x, sensor_y, distance));
            }
            None => panic!("Regex failed"),
        }
    }

    let mut set: HashSet<Point2<i32>> = HashSet::new();

    let row = 2_000_000;
    for (sensor_x, sensor_y, distance) in &sensors {
        if row >= sensor_y - distance && row <= sensor_y + distance {
            for x in (sensor_x - (distance - sensor_y.abs_diff(row) as i32))
                ..(sensor_x + (distance - sensor_y.abs_diff(row) as i32))
            {
                set.insert(Point2::<i32>::new(x, row));
            }
        }
    }

    let r1 = set.len();
    let mut r2: i64 = 0;
    'l: for (sensor_x, sensor_y, distance) in &sensors {
        for y in sensor_y - distance..sensor_y + distance {
            for x in [
                sensor_x - (distance - sensor_y.abs_diff(y) as i32) - 1,
                sensor_x + (distance - sensor_y.abs_diff(y) as i32) + 1,
            ] {
                if (0..=4_000_000).contains(&x)
                    && (0..=4_000_000).contains(&y)
                    && sensors.iter().all(|(other_x, other_y, other_distance)| {
                        TryInto::<i32>::try_into(x.abs_diff(*other_x) + y.abs_diff(*other_y))
                            .unwrap()
                            > *other_distance
                    })
                {
                    r2 = x as i64 * 4_000_000 + y as i64;
                    break 'l;
                }
            }
        }
    }

    (r1, r2)
}
