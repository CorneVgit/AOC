use crate::util::read_all;

fn d7values() -> Vec<usize> {
    let result_values = read_all::<String>("input_7");
    let mut values: Vec<String> = Vec::new();

    for result_value in result_values {
        let value = match result_value {
            Ok(x) => x,
            _ => continue,
        };
        values.push(String::from(value));
    }

    values
        .first()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn d7a() {
    let mut values = d7values();

    values.sort();
    let median = values[values.len() / 2];

    let mut new_values = values.clone();

    for new_value in &mut new_values {
        if *new_value < median {
            *new_value = *new_value + (median - *new_value) * 2;
        }
    }

    println!(
        "{}",
        new_values.iter().sum::<usize>() - (median * values.len())
    );
}

pub fn d7b() {
    let values = d7values();
    let mut results: Vec<usize> = Vec::new();

    let average = values.iter().sum::<usize>() as f64 / values.len() as f64;

    for n in average.floor() as usize..=average.ceil() as usize {
        let mut new_values = Vec::new();

        for value in &values {
            let x = value.abs_diff(n);
            new_values.push((x * (x + 1)) / 2);
        }

        results.push(new_values.iter().sum::<usize>());
    }

    println!("{:?}", results.iter().min().unwrap());
}
