use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<Vec<u32>> {
    read_all::<String>("input_3")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[must_use]
pub fn d3() -> (u64, u64) {
    let banks = get_input();

    (calculate_joltage(&banks, 2), calculate_joltage(&banks, 12))
}

fn calculate_joltage(banks: &[Vec<u32>], amount: u32) -> u64 {
    banks
        .iter()
        .map(|bank| {
            let mut joltage = 0;
            let mut start_offset = 0;

            for n in 0..amount {
                let end_offset = amount - n - 1;

                let position = bank[start_offset..bank.len() - end_offset as usize]
                    .iter()
                    .position_max_first()
                    .unwrap()
                    + start_offset;

                joltage += bank[position] as u64 * 10u64.pow(end_offset);
                start_offset = position + 1;
            }

            joltage
        })
        .sum()
}

impl<T> MyIterator for T where T: Iterator + ?Sized {}

pub trait MyIterator: Iterator {
    fn position_max_first(self) -> Option<usize>
    where
        Self: Sized,
        Self::Item: Ord,
        Self: ExactSizeIterator,
        Self: DoubleEndedIterator,
    {
        self.enumerate()
            .rev()
            .max_by(|x, y| Ord::cmp(&x.1, &y.1))
            .map(|x| x.0)
    }
}
