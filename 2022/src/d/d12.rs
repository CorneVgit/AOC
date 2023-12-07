use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<String> {
    read_all::<String>("input_12")
        .into_iter()
        .map(|value| value.unwrap_infallible())
        .collect()
}

pub fn d12() -> (usize, usize) {
    let _values = values();

    let r1 = 0;
    let r2 = 0;

    (r1, r2)
}
