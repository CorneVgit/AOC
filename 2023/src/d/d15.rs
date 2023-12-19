use ascii::AsciiString;

use crate::util::read_single_ascii_string;

fn get_input() -> Vec<AsciiString> {
    let input = read_single_ascii_string("input_15");

    input
        .split(ascii::AsciiChar::Comma)
        .map(std::borrow::ToOwned::to_owned)
        .collect()
}

#[must_use]
pub fn d15() -> (u64, u64) {
    let input = get_input();

    let r1 = input.iter().map(hash_function).sum();

    (r1, 0)
}

fn hash_function(value: &AsciiString) -> u64 {
    let mut current_value: u64 = 0;

    for character in value.chars() {
        current_value += u64::from(character.as_byte());
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}
