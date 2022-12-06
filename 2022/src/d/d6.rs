use itertools::Itertools;

use crate::util::read_single_string;

pub fn d6() -> (usize, usize) {
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
