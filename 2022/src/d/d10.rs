use na::{Matrix, SMatrix};
use unwrap_infallible::UnwrapInfallible;

extern crate nalgebra as na;

use crate::util::read_all;

fn operations() -> Vec<String> {
    read_all::<String>("input_10")
        .into_iter()
        .map(|rucksack| rucksack.unwrap_infallible())
        .collect()
}

enum Operation {
    Addx(i32),
    Noop,
}

pub fn d10() -> (
    i32,
    Matrix<char, na::Const<6>, na::Const<40>, na::ArrayStorage<char, 6, 40>>,
) {
    let operations = operations();

    let mut x = 1;
    let mut cycle = 0;
    let mut cycle_time;
    let mut current_operation;

    let mut r1 = 0;
    let mut r2 = SMatrix::<char, 40, 6>::from_element('.');

    let mut crt_iter = r2.iter_mut().enumerate();

    // Fetch
    for operation in operations {
        // Decode
        if operation.starts_with("addx") {
            let v = operation.split(' ').last().unwrap();

            current_operation = Operation::Addx(v.parse::<i32>().unwrap());
            cycle_time = 2;
        } else {
            current_operation = Operation::Noop;
            cycle_time = 1;
        }

        // Execute
        while cycle_time > 0 {
            cycle += 1;

            if (cycle - 20) % 40 == 0 {
                r1 += cycle * x;
            }

            let (a, b) = crt_iter.next().unwrap();
            if a as i32 % 40 >= (x - 1) && a as i32 % 40 <= (x + 1) {
                *b = '#';
            }

            cycle_time -= 1;
        }

        // Write-back
        match current_operation {
            Operation::Addx(v) => x += v,
            Operation::Noop => (),
        }
    }

    (r1, r2.transpose())
}
