use std::collections::HashMap;

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

    let mut r: HashMap<usize, usize> = HashMap::new();

    all_unique(0, &input, &mut 0, &mut r);

    (r[&4], r[&14])
}

fn all_unique(max_o: usize, input: &[char], i: &mut usize, r: &mut HashMap<usize, usize>) {
    'l: loop {
        for o in 0..max_o {
            if *i + max_o >= input.len() {
                return
            }
            if input[*i + o + 1..*i + max_o].contains(&input[*i + o]) {
                *i += o + 1;
                continue 'l
            }
        }

        r.entry(max_o).or_insert_with(|| *i + max_o);

        all_unique(max_o + 1, input, i, r);
        return
    }
}
