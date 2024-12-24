use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[must_use]
fn get_input() -> Vec<String> {
    read_all::<String>("input_17")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .collect()
}

#[must_use]
pub fn d17() -> (String, u128) {
    let input = get_input();

    let mut it = input.iter();

    let reg_a: u128 = it
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap();
    let reg_b: u128 = it
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap();
    let reg_c: u128 = it
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap();
    let program: Vec<u128> = it
        .nth(1)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let r1 = run_program(reg_a, reg_b, reg_c, &program, usize::MAX)
        .iter()
        .map(u128::to_string)
        .collect_vec()
        .join(",");

    let mut reg_a = 0;

    loop {
        let r = run_program(reg_a, reg_b, reg_c, &program, program.len());

        // 2,4,1,7,7,5,0,3,4,0,1,7,5,5,3,0
        match r[..] {
            [2, 4, 1, 7, 7, 5, 0, 3, 4, 0, 1, 7, 5, 5, 3, 0] => break,
            [.., 4, 1, 7, 7, 5, 0, 3, 4, 0, 1, 7, 5, 5, 3, 0] => reg_a += 8u128.pow(0),
            [.., 1, 7, 7, 5, 0, 3, 4, 0, 1, 7, 5, 5, 3, 0] => reg_a += 8u128.pow(1),
            [.., 7, 7, 5, 0, 3, 4, 0, 1, 7, 5, 5, 3, 0] => reg_a += 8u128.pow(2),
            [.., 7, 5, 0, 3, 4, 0, 1, 7, 5, 5, 3, 0] => reg_a += 8u128.pow(3),
            [.., 5, 0, 3, 4, 0, 1, 7, 5, 5, 3, 0] => reg_a += 8u128.pow(4),
            [.., 0, 3, 4, 0, 1, 7, 5, 5, 3, 0] => reg_a += 8u128.pow(5),
            [.., 3, 4, 0, 1, 7, 5, 5, 3, 0] => reg_a += 8u128.pow(6),
            [.., 4, 0, 1, 7, 5, 5, 3, 0] => reg_a += 8u128.pow(7),
            [.., 0, 1, 7, 5, 5, 3, 0] => reg_a += 8u128.pow(8),
            [.., 1, 7, 5, 5, 3, 0] => reg_a += 8u128.pow(9),
            [.., 7, 5, 5, 3, 0] => reg_a += 8u128.pow(10),
            [.., 5, 5, 3, 0] => reg_a += 8u128.pow(11),
            [.., 5, 3, 0] => reg_a += 8u128.pow(12),
            [.., 3, 0] => reg_a += 8u128.pow(13),
            [.., 0] => reg_a += 8u128.pow(14),
            _ => reg_a += 8u128.pow(15),
        }

        // 0,3,5,4,3,0
        // match r[..] {
        //     [0, 3, 5, 4, 3, 0] => break,
        //     [.., 3, 5, 4, 3, 0] => reg_a += 8u128.pow(1),
        //     [.., 5, 4, 3, 0] => reg_a += 8u128.pow(2),
        //     [.., 4, 3, 0] => reg_a += 8u128.pow(3),
        //     [.., 3, 0] => reg_a += 8u128.pow(4),
        //     [.., 0] => reg_a += 8u128.pow(5),
        //     _ => reg_a += 8u128.pow(0),
        // }
    }

    (r1, reg_a)
}

fn run_program(
    mut reg_a: u128,
    mut reg_b: u128,
    mut reg_c: u128,
    program: &[u128],
    max_out: usize,
) -> Vec<u128> {
    let mut ip = 0;
    let mut out = Vec::new();

    loop {
        if ip + 1 >= program.len() || out.len() >= max_out {
            break;
        }

        let opcode = program[ip];
        let operand = program[ip + 1];

        match opcode {
            0 => reg_a = reg_a / 2u128.pow(get_combo_operand(operand, reg_a, reg_b, reg_c) as u32),
            1 => reg_b ^= operand,
            2 => reg_b = get_combo_operand(operand, reg_a, reg_b, reg_c) % 8,
            3 => {
                if reg_a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => reg_b ^= reg_c,
            5 => out.push(get_combo_operand(operand, reg_a, reg_b, reg_c) % 8),
            6 => reg_b = reg_a / 2u128.pow(get_combo_operand(operand, reg_a, reg_b, reg_c) as u32),
            7 => reg_c = reg_a / 2u128.pow(get_combo_operand(operand, reg_a, reg_b, reg_c) as u32),
            _ => panic!(),
        }

        ip += 2;
    }

    out
}

fn get_combo_operand(operand: u128, reg_a: u128, reg_b: u128, reg_c: u128) -> u128 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => panic!(),
    }
}
