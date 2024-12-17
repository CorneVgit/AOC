use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::digit1, multi::many1};
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[must_use]
fn get_input() -> Vec<(Point, Point, Point)> {
    read_all::<String>("input_13")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .filter(|s| !s.is_empty())
        .tuples()
        .map(|(a, b, p)| {
            let (ax, ay) = parse_a(&a);
            let (bx, by) = parse_b(&b);
            let (px, py) = parse_p(&p);
            (Point::new(ax, ay), Point::new(bx, by), Point::new(px, py))
        })
        .collect()
}

fn parse_a(input: &str) -> (i64, i64) {
    let (s, _) = tag::<&str, &str, nom::error::Error<&str>>("Button A: X+")(input).unwrap();
    let (s, x) = many1(digit1::<&str, nom::error::Error<&str>>)(s).unwrap();
    let (s, _) = tag::<&str, &str, nom::error::Error<&str>>(", Y+")(s).unwrap();
    let (_, y) = many1(digit1::<&str, nom::error::Error<&str>>)(s).unwrap();

    (
        x.first().unwrap().parse().unwrap(),
        y.first().unwrap().parse().unwrap(),
    )
}

fn parse_b(input: &str) -> (i64, i64) {
    let (s, _) = tag::<&str, &str, nom::error::Error<&str>>("Button B: X+")(input).unwrap();
    let (s, x) = many1(digit1::<&str, nom::error::Error<&str>>)(s).unwrap();
    let (s, _) = tag::<&str, &str, nom::error::Error<&str>>(", Y+")(s).unwrap();
    let (_, y) = many1(digit1::<&str, nom::error::Error<&str>>)(s).unwrap();

    (
        x.first().unwrap().parse().unwrap(),
        y.first().unwrap().parse().unwrap(),
    )
}

fn parse_p(input: &str) -> (i64, i64) {
    let (a, _) = tag::<&str, &str, nom::error::Error<&str>>("Prize: X=")(input).unwrap();
    let (a, x) = many1(digit1::<&str, nom::error::Error<&str>>)(a).unwrap();
    let (a, _) = tag::<&str, &str, nom::error::Error<&str>>(", Y=")(a).unwrap();
    let (_, y) = many1(digit1::<&str, nom::error::Error<&str>>)(a).unwrap();

    (
        x.first().unwrap().parse().unwrap(),
        y.first().unwrap().parse().unwrap(),
    )
}

#[must_use]
pub fn d13() -> (i64, usize) {
    let input = get_input();

    let r1 = input.iter().map(|(a, b, p)| solve(a, b, p)).sum();

    (r1, 0)
}

fn solve(button_a: &Point, button_b: &Point, prize: &Point) -> i64 {
    let mut r = (i64::MAX, i64::MAX);

    for num_of_a in 0..100 {
        for num_of_b in 0..100 {
            let x = num_of_a * button_a.x + num_of_b * button_b.x;
            let y = num_of_a * button_a.y + num_of_b * button_b.y;

            if x >= prize.x {
                if x == prize.x
                    && y == prize.y
                    && (r.0.saturating_mul(3).saturating_add(r.1))
                        > (num_of_a.saturating_mul(3).saturating_add(num_of_b))
                {
                    r = (num_of_a, num_of_b);
                }

                break;
            }
        }
    }

    match r {
        (i64::MAX, i64::MAX) => 0,
        _ => r.0 * 3 + r.1,
    }
}
