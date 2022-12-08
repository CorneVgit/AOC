use na::{DMatrix, RowDVector};
use unwrap_infallible::UnwrapInfallible;

extern crate nalgebra as na;

use crate::util::read_all;

fn values() -> DMatrix<u32> {
    let values: Vec<String> = read_all::<String>("input_8")
        .into_iter()
        .map(|value| value.unwrap_infallible())
        .collect();

    let row_count = values.len();
    let col_count = values[0].len();

    let mut grid = DMatrix::zeros(row_count, col_count);

    for (row_i, value) in values.iter().enumerate() {
        let mut row = RowDVector::<u32>::zeros(col_count);

        for (col_i, c) in value.chars().enumerate() {
            row[col_i] = c.to_digit(10).unwrap();
        }

        grid.set_row(row_i, &row);
    }

    grid
}

pub fn d8() -> (usize, usize) {
    let grid = &values();

    let mut r1 = 0;
    let mut r2 = 0;

    for (row_i, row) in grid.row_iter().enumerate() {
        for (col_i, col) in grid.column_iter().enumerate() {
            let selected_tree_height = row[col_i];

            if col_i == 0 || row_i == 0 || col_i == row.len() - 1 || row_i == col.len() - 1 {
                r1 += 1;
                continue;
            }

            let up = &col.slice_range(0..row_i, 0);
            let down = &col.slice_range(row_i + 1.., 0);
            let left = &row.slice_range(0, 0..col_i);
            let right = &row.slice_range(0, col_i + 1..);

            if selected_tree_height > up.max()
                || selected_tree_height > down.max()
                || selected_tree_height > left.max()
                || selected_tree_height > right.max()
            {
                r1 += 1;
            }

            let mut score = 1;
            score *= calculate_distance(up.iter().rev(), selected_tree_height);
            score *= calculate_distance(down.iter(), selected_tree_height);
            score *= calculate_distance(left.iter().rev(), selected_tree_height);
            score *= calculate_distance(right.iter(), selected_tree_height);

            if score > r2 {
                r2 = score
            }
        }
    }

    (r1, r2)
}

fn calculate_distance<'a, I>(iter: I, selected_tree_height: u32) -> usize
where
    I: Iterator<Item = &'a u32>,
{
    let mut distance = 0;

    for tree_height in iter {
        distance += 1;
        if *tree_height >= selected_tree_height {
            break;
        }
    }

    distance
}
