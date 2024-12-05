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
    let input = read_single_string("input_3");

    let r1 = total_result_of_real_mul_operations(&input, false);
    let r2 = total_result_of_real_mul_operations(&input, true);

    (r1, r2)
}

fn total_result_of_real_mul_operations(input_: &str, do_dont: bool) -> u64 {
    let mut pairs: Vec<(u64, u64)> = Vec::new();

    let mut input = input_.to_string();

    loop {
        // I love ownership
        let inp = input.clone();

        let Ok(mul) = take_until::<&str, &str, Error<&str>>("mul(")(&inp) else {
            break;
        };

        if do_dont {
            if let Ok(o) = take_until::<&str, &str, Error<&str>>("don't()")(&inp) {
                // Is there is a don't instruction before the next mul instruction
                if mul.0.len() < o.0.len() {
                    // Take until the next do instruction
                    if let Ok(o) = take_until::<&str, &str, Error<&str>>("do()")(o.0) {
                        // Start looking for more mul instructions
                        input = o.0.to_string();
                        continue;
                    }

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

    pairs.iter().fold(0, |a, p| a + p.0 * p.1)
}

fn parse_mul(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        tag("mul("),
        separated_pair(digit1, tag(","), digit1),
        tag(")"),
    )(input)
}
