use itertools::Itertools;
use nalgebra::DMatrix;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<Vec<(usize, usize)>> {
    read_all::<String>("input_14")
        .into_iter()
        .map(|value| value.unwrap_infallible())
        .map(|v| {
            v.split(" -> ")
                .map(|s| s.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>()
}

pub fn d14() -> (usize, usize) {
    let paths = values();

    let mmx = paths
        .iter()
        .flatten()
        .map(|p| p.0)
        .minmax()
        .into_option()
        .unwrap();
    let mmy = paths
        .iter()
        .flatten()
        .map(|p| p.1)
        .minmax()
        .into_option()
        .unwrap();
    let (x_min, ncols, nrows): (usize, usize, usize) = (
        mmx.0 - mmy.1,
        (mmx.1 + mmy.1) - (mmx.0 - mmy.1) + 1,
        mmy.1 + 1,
    );

    let mut grid = DMatrix::from_element(nrows, ncols, '.');

    generate_cave(&mut grid, x_min, paths, nrows);

    grid = grid.transpose();

    let mut i = 0;

    fill_cave(x_min, &mut grid, &mut i);
    let r1 = i;

    // println!("{}", grid.slice((mmx.0 - x_min, 0), (mmx.1 - mmx.0, nrows)).transpose());

    grid = grid.insert_column(nrows, '.');
    grid = grid.insert_column(nrows + 1, '#');

    fill_cave(x_min, &mut grid, &mut i);
    let r2 = i;

    (r1, r2)
}

fn generate_cave(
    grid: &mut nalgebra::Matrix<
        char,
        nalgebra::Dynamic,
        nalgebra::Dynamic,
        nalgebra::VecStorage<char, nalgebra::Dynamic, nalgebra::Dynamic>,
    >,
    x_min: usize,
    paths: Vec<Vec<(usize, usize)>>,
    nrows: usize,
) {
    *grid.get_mut(500 - x_min).unwrap() = '+';
    for path in paths {
        for ((x1, y1), (x2, y2)) in path.iter().tuple_windows::<(_, _)>() {
            for i in *x1.min(x2) - x_min..=*x1.max(x2) - x_min {
                for j in *y1.min(y2)..=*y1.max(y2) {
                    *grid.get_mut(j + i * nrows).unwrap() = '#';
                }
            }
        }
    }
}

fn fill_cave(
    x_min: usize,
    grid: &mut nalgebra::Matrix<
        char,
        nalgebra::Dynamic,
        nalgebra::Dynamic,
        nalgebra::VecStorage<char, nalgebra::Dynamic, nalgebra::Dynamic>,
    >,
    i: &mut usize,
) {
    'l: loop {
        let mut column = 500 - x_min;
        let mut row = 0;
        loop {
            if let Some(b) = grid.get_mut(column + (row + 1) * grid.nrows()) {
                match b {
                    '.' => {
                        row += 1;
                        continue;
                    }
                    '#' | 'o' => (),
                    _ => panic!("invalid char"),
                }
            } else {
                break 'l;
            }

            if let Some(l) = grid.get_mut((column - 1) + (row + 1) * grid.nrows()) {
                match l {
                    '.' => {
                        row += 1;
                        column -= 1;
                        continue;
                    }
                    '#' | 'o' => (),
                    _ => panic!("invalid char"),
                }
            } else {
                break 'l;
            }
            if let Some(r) = grid.get_mut((column + 1) + (row + 1) * grid.nrows()) {
                match r {
                    '.' => {
                        row += 1;
                        column += 1;
                        continue;
                    }
                    '#' | 'o' => (),
                    _ => panic!("invalid char"),
                }
            } else {
                break 'l;
            }

            *grid.get_mut(column + row * grid.nrows()).unwrap() = 'o';
            break;
        }

        *i += 1;

        if *grid.get_mut(500 - x_min).unwrap() == 'o' {
            break 'l;
        }
    }
}
