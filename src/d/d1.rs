use crate::util::read_all;

fn d1values() -> Vec<u64> {
    let result_values = read_all::<u64>("input_1");
    let mut values: Vec<u64> = Vec::new();

    for result_value in result_values {
        let value = match result_value {
            Ok(x) => x,
            _ => continue
        };
        values.push(value);
    }

    return values;
}

pub fn d1a() {
    let values = d1values();

    let mut counter = 0;

    let mut old_value: Option<u64> = None;

    for value in values {
        let old_v = match old_value {
            None => value,
            Some(x) => x
        };

        if value > old_v {
            counter += 1;
        }

        old_value = Some(value);
    }

    println!("{}", counter);
}

pub fn d1b() {
    let values = d1values();

    let mut counter = 0;

    let mut old_window: Vec<u64> = Vec::new();
    old_window.extend(&values[0..=2]);

    for value in &values[3..] {
        let mut new_window: Vec<u64> = Vec::new();
        new_window.extend(&old_window[1..]);
        new_window.push(*value);

        let old_sum: u64 = old_window.iter().sum();
        let new_sum: u64 = new_window.iter().sum();

        if new_sum > old_sum {
            counter += 1;
        }

        old_window = new_window;
    }

    println!("{}", counter);
}
