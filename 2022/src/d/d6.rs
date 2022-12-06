use itertools::Itertools;

use crate::util::read_single_string;

pub fn d6_slow() -> (usize, usize) {
    let input = read_single_string("input_6").chars().collect::<Vec<char>>();

    let mut r1 = 0;
    let r2;

    let mut i = 0;
    loop {
        if input[i..i + 4].iter().all_unique() {
            if r1 == 0 {
                r1 = i + 4;
            }

            if input[i..i + 14].iter().all_unique() {
                r2 = i + 14;
                break;
            }
        }
        i += 1;
    }

    (r1, r2)
}

pub fn d6_fast() -> (usize, usize) {
    let input = read_single_string("input_6").chars().collect::<Vec<char>>();

    let mut r1 = 0;
    let r2;

    let mut i = 0;

    loop {
        if all_unique(4, &input, &mut i) {
            if r1 == 0 {
                r1 = i + 4;
            }
            if all_unique(7, &input, &mut i)
                && all_unique(11, &input, &mut i)
                && all_unique(14, &input, &mut i)
            {
                r2 = i + 14;
                break;
            }
        }
    }

    (r1, r2)
}

fn all_unique(max_o: usize, input: &[char], i: &mut usize) -> bool {
    for o in 0..max_o {
        if input[*i + o + 1..*i + max_o].contains(&input[*i + o]) {
            *i += o + 1;
            return false;
        }
    }

    true
}
