use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    error::Error,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::util::read_single_string;

#[must_use]
pub fn d3() -> (u64, u64) {
    let mut input = read_single_string("input_3");
    let mut pairs: Vec<(u64, u64)> = Vec::new();

    loop {
        // I love ownership
        let inp = input.clone();

        let mul = match take_until::<&str, &str, Error<&str>>("mul(")(&inp) {
            // Possible mul instruction found
            Ok(o) => o,
            // No mul instruction left
            Err(_) => break,
        };

        if let Ok(o) = take_until::<&str, &str, Error<&str>>("don't()")(&inp) {
            // Is there is a don't instruction before the next mul instruction
            if mul.0.len() < o.0.len() {
                // Take until the next do instruction
                if let Ok(o) = take_until::<&str, &str, Error<&str>>("do()")(o.0) {
                    // Start looking for more mul instructions
                    input = o.0.to_string();
                    continue;
                } else {
                    // No next do instruction, so no more mul instructions to process
                    break;
                }
            }
        }

        match parse_mul(mul.0) {
            Ok(o) => {
                // Real mul instruction, add to valid pairs
                pairs.push((o.1 .0.parse().unwrap(), o.1 .1.parse().unwrap()));
                input = o.0.to_string();
            }
            Err(x) => {
                // Corrupted mul instruction, skip
                x.map(|e| input = e.input.to_string());
            }
        };
    }

    let r1 = pairs.iter().fold(0, |a, p| a + p.0 * p.1);

    (r1, 0)
}

fn parse_mul(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        tag("mul("),
        separated_pair(digit1, tag(","), digit1),
        tag(")"),
    )(input)
}
