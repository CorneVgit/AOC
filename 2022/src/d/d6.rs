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
    let indices: Vec<usize> = vec![4, 9, 14];

    all_unique(1, &input, &indices, &mut 0, &mut r);

    (r[&4], r[&14])
}

fn all_unique(
    indices_index: usize,
    input: &[char],
    indices: &[usize],
    input_index: &mut usize,
    r: &mut HashMap<usize, usize>,
) {
    'l: loop {
        for o in 0..indices[indices_index - 1] {
            if *input_index + r.len() >= input.len() {
                return;
            }
            if input[*input_index + o + 1..*input_index + indices[indices_index - 1]]
                .contains(&input[*input_index + o])
            {
                *input_index += o + 1;

                if indices_index != 1 {
                    return;
                } else {
                    continue 'l;
                }
            }
        }

        r.entry(indices[indices_index - 1])
            .or_insert(*input_index + indices[indices_index - 1]);

        if r.len() == indices.len() {
            return;
        }

        all_unique(indices_index + 1, input, indices, input_index, r);
    }
}
