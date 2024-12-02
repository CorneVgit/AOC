use std::ops::{Div, Mul};

use euclid::default::{Point3D, Vector3D};
use itertools::Itertools;

use crate::util::read_all;

fn get_input() -> Vec<String> {
    let result_values = read_all::<String>("input_24");

    result_values
        .into_iter()
        .map(unwrap_infallible::UnwrapInfallible::unwrap_infallible)
        .collect()
}

#[derive(Clone)]
struct Line {
    initial_pos: Point3D<f64>,
    velocity: Vector3D<f64>,
    slope: f64,
    intercept: f64,
}

#[must_use]
pub fn d24() -> (usize, u64) {
    let input = get_input();

    let lines = input
        .into_iter()
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            let initial_pos = Point3D::from(
                pos.split(',')
                    .map(|v| v.trim().parse().unwrap())
                    .collect_tuple::<(f64, f64, f64)>()
                    .unwrap(),
            );
            let velocity = Vector3D::from(
                vel.split(',')
                    .map(|v| v.trim().parse().unwrap())
                    .collect_tuple::<(f64, f64, f64)>()
                    .unwrap(),
            );

            let slope = velocity.y / velocity.x;
            let intercept = initial_pos.y - (slope * initial_pos.x);

            Line {
                initial_pos,
                velocity,
                slope,
                intercept,
            }
        })
        .collect_vec();

    let mut r1 = 0;
    for l in lines.iter().combinations(2) {
        let a = l.first().unwrap();
        let b = l.last().unwrap();

        let x = (b.intercept - a.intercept) / (a.slope - b.slope);
        let y = a.slope * x + a.intercept;

        // println!(
        //     "Hailstone A: {:?} @ {:?} | {} + {}",
        //     a.initial_pos, a.velocity, a.slope, a.intercept
        // );
        // println!(
        //     "Hailstone B: {:?} @ {:?} | {} + {}",
        //     b.initial_pos, b.velocity, b.slope, b.intercept
        // );

        // let mut res = x > 7.0 && x < 27.0 && y > 7.0 && y < 27.0;
        let mut res = x > 200000000000000.0
            && x < 400000000000000.0
            && y > 200000000000000.0
            && y < 400000000000000.0;
        res &= (x < a.initial_pos.x && a.velocity.x.signum() < 0.0)
            || (x > a.initial_pos.x && a.velocity.x.signum() > 0.0);
        res &= (x < b.initial_pos.x && b.velocity.x.signum() < 0.0)
            || (x > b.initial_pos.x && b.velocity.x.signum() > 0.0);

        // println!("{x}, {y}, {res}");

        if res {
            r1 += 1;
        }
    }

    // for line_pair in lines.iter().combinations(2) {
        // let a = l.first().unwrap();
        // let b = l.last().unwrap();

        let a = Line {
            initial_pos: (6.0, 8.0, 4.0).into(),
            velocity: (6.0, 7.0, 0.0).into(),
            slope: 0.0,
            intercept: 0.0,
        };
        let b = Line {
            initial_pos: (6.0, 8.0, 2.0).into(),
            velocity: (6.0, 7.0, 4.0).into(),
            slope: 0.0,
            intercept: 0.0,
        };

        let c = a.initial_pos.to_vector();
        let d = b.initial_pos.to_vector();
        let e = a.velocity;
        let f = b.velocity;

        let g = d - c;
        let hc = f.cross(g);
        let h = hc.dot(hc);
        let kc = f.cross(e);
        let k = kc.dot(kc);

        if h != 0.0 && k != 0.0 {
            let l = e * (h.div(k));

            let m_min = c - l;
            let m_plus = c + l;

            let intersect = c + e.div(2.0);

            println!("{:?}", m_min);
            println!("{:?}", m_plus);
            println!("{:?}", intersect);
        } else {
            println!("No intersect");
        }
    // }

    (r1, 0)
}
