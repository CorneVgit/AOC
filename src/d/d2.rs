use crate::util::read_all;

fn d2values() -> Vec<(String, i64)> {
    let result_values = read_all::<String>("input_2");
    let mut values: Vec<(String, i64)> = Vec::new();

    for result_value in result_values {
        match result_value {
            Ok(x) => {
                let mut s = x.split_ascii_whitespace();
                let instruction = String::from(s.next().expect("no instruction"));
                let value = s.next().expect("no value").parse().unwrap();

                values.push((instruction, value));
            }
            _ => continue,
        };
    }

    values
}

pub fn d2a() {
    let values = d2values();

    let mut distance = 0;
    let mut depth = 0;

    for value in values {
        match value.0.as_str() {
            "forward" => distance += value.1,
            "down" => depth += value.1,
            "up" => depth -= value.1,
            _ => continue,
        }
    }

    println!("{}", distance * depth);
}

pub fn d2b() {
    let values = d2values();

    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;

    for value in values {
        match value.0.as_str() {
            "forward" => {
                distance += value.1;
                depth += aim * value.1;
            }
            "down" => aim += value.1,
            "up" => aim -= value.1,
            _ => continue,
        }
    }

    println!("{}", distance * depth);
}
