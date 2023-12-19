use itertools::Itertools;
use nalgebra::{DMatrix, Point2};

use crate::util::read_all;

fn get_input() -> DMatrix<bool> {
    let values: Vec<String> = read_all::<String>("input_11")
        .into_iter()
        .map(unwrap_infallible::UnwrapInfallible::unwrap_infallible)
        .collect();

    let row_count = values.len();
    let col_count = values[0].len();

    let values = values
        .iter()
        .flat_map(|s| {
            s.chars().map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!(),
            })
        })
        .collect_vec();

    let mut grid = DMatrix::from_vec(row_count, col_count, values);

    let mut i = 0;
    for row in grid.clone().row_iter() {
        if row.iter().all(|v| v == &false) {
            grid = grid.insert_row(i, false);
            i += 1;
        }

        i += 1;
    }

    i = 0;
    for col in grid.clone().column_iter() {
        if col.iter().all(|v| v == &false) {
            grid = grid.insert_column(i, false);
            i += 1;
        }

        i += 1;
    }

    grid.transpose()
}

#[must_use]
pub fn d11() -> (i64, u64) {
    let input = get_input();

    let mut galaxies = Vec::new();

    for (i, row) in input.row_iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if *v {
                galaxies.push(Point2::new(i64::try_from(i).unwrap(), j as i64));
            }
        }
    }

    let r1 = galaxies
        .iter()
        .combinations(2)
        .fold(0, |acc, g| acc + (g[0] - g[1]).abs().sum());

    (r1, 0)
}
