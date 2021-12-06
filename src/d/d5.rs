use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

use crate::util::read_all;

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    from: Point,
    to: Point,
}

fn d5values() -> Vec<Line> {
    let result_values = read_all::<String>("input_5");
    let mut lines: Vec<Line> = Vec::new();

    for result_value in result_values {
        let value = match result_value {
            Ok(s) => s,
            _ => continue,
        };

        let points: Vec<&str> = value.split(" -> ").collect();
        let a_as_vec = points.first().unwrap().split(",").collect::<Vec<&str>>();
        let b_as_vec = points.last().unwrap().split(",").collect::<Vec<&str>>();
        let a = Point {
            x: a_as_vec.first().unwrap().parse().unwrap(),
            y: a_as_vec.last().unwrap().parse().unwrap(),
        };
        let b = Point {
            x: b_as_vec.first().unwrap().parse().unwrap(),
            y: b_as_vec.last().unwrap().parse().unwrap(),
        };

        if a >= b {
            lines.push(Line { from: b, to: a });
        } else {
            lines.push(Line { from: a, to: b })
        }
    }

    lines
}

pub fn d5a() {
    let lines = d5values();

    let mut non_diagonal_lines: Vec<Line> = Vec::new();
    let mut diagonal_lines: Vec<Line> = Vec::new();

    for line in &lines {
        if (line.from.x == line.to.x) || (line.from.y == line.to.y) {
            non_diagonal_lines.push(*line);
        } else {
            diagonal_lines.push(*line);
        }
    }

    let mut field: HashMap<Point, usize> = HashMap::new();

    for line in &non_diagonal_lines {
        for x in line.from.x..=line.to.x {
            for y in line.from.y..=line.to.y {
                let point = Point { x, y };

                let field_point = match field.entry(point) {
                    Occupied(entry) => entry.into_mut(),
                    Vacant(entry) => entry.insert(0),
                };

                *field_point += 1;
            }
        }
    }

    println!("{:?}", field.values().filter(|value| *value > &1).count());
}

pub fn d5b() {
    let lines = d5values();

    let mut non_diagonal_lines: Vec<Line> = Vec::new();
    let mut diagonal_lines: Vec<Line> = Vec::new();

    for line in &lines {
        if (line.from.x == line.to.x) || (line.from.y == line.to.y) {
            non_diagonal_lines.push(*line);
        } else {
            diagonal_lines.push(*line);
        }
    }

    let mut field: HashMap<Point, usize> = HashMap::new();

    for line in &non_diagonal_lines {
        for x in line.from.x..=line.to.x {
            for y in line.from.y..=line.to.y {
                let point = Point { x, y };

                let field_point = match field.entry(point) {
                    Occupied(entry) => entry.into_mut(),
                    Vacant(entry) => entry.insert(0),
                };

                *field_point += 1;
            }
        }
    }

    for line in &diagonal_lines {
        for n in 0..=(line.to.x - line.from.x) {
            let mut point: Point = Point::new();
            point.x = line.from.x + n;
            match line.from.y > line.to.y {
                true => point.y = line.from.y - n,
                false => point.y = line.from.y + n,
            }

            let field_point = match field.entry(point) {
                Occupied(entry) => entry.into_mut(),
                Vacant(entry) => entry.insert(0),
            };

            *field_point += 1;
        }
    }

    println!("{:?}", field.values().filter(|value| *value > &1).count());
}
