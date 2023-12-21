use std::collections::HashSet;

use nalgebra::Point2;

use crate::util::read_all;

fn get_input() -> Vec<String> {
    let result_values = read_all::<String>("input_21");

    result_values
        .into_iter()
        .map(unwrap_infallible::UnwrapInfallible::unwrap_infallible)
        .collect()
}

#[must_use]
pub fn d21() -> (usize, usize) {
    let input = get_input();

    let mut garden_plots: HashSet<Point2<i64>> = HashSet::new();
    let mut visited_plots: HashSet<Point2<i64>> = HashSet::new();

    for (y, line) in (0..).zip(&input) {
        for (x, v) in (0..).zip(line.chars()) {
            match v {
                '.' => {
                    garden_plots.insert(Point2::new(x, y));
                } // garden plot
                'S' => {
                    garden_plots.insert(Point2::new(x, y));
                    visited_plots.insert(Point2::new(x, y));
                }
                _ => (), // rock
            }
        }
    }

    let r1 = calc_reach(&visited_plots, &garden_plots, &input, 64);
    let r2 = 0;

    (r1, r2)
}

fn calc_reach(
    visited_plots: &HashSet<nalgebra::OPoint<i64, nalgebra::Const<2>>>,
    garden_plots: &HashSet<nalgebra::OPoint<i64, nalgebra::Const<2>>>,
    input: &[String],
    steps: usize,
) -> usize {
    let mut visited_plots = visited_plots.clone();

    for _ in 1..=steps {
        let mut new_visited_plots: HashSet<Point2<i64>> = HashSet::new();

        for vp in &visited_plots {
            for p in [
                Point2::new(vp.x + 1, vp.y),
                Point2::new(vp.x - 1, vp.y),
                Point2::new(vp.x, vp.y + 1),
                Point2::new(vp.x, vp.y - 1),
            ] {
                if garden_plots.contains(&Point2::new(
                    ((p.x % input.len() as i64) + input.len() as i64) % input.len() as i64,
                    ((p.y % input.len() as i64) + input.len() as i64) % input.len() as i64,
                )) {
                    new_visited_plots.insert(p);
                }
            }
        }

        visited_plots = new_visited_plots;
    }

    visited_plots.len()
}
