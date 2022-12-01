use crate::util::read_all;

fn d8values() -> Vec<(Vec<String>, Vec<String>)> {
    let result_values = read_all::<String>("input_8");
    let mut values: Vec<(Vec<String>, Vec<String>)> = Vec::new();

    for result_value in result_values {
        let value = match result_value {
            Ok(x) => x,
            _ => continue,
        };

        let s = value.split("|").collect::<Vec<&str>>();
        let signals = s.first().unwrap().trim().split_ascii_whitespace().map(|x| String::from(x)).collect();
        let output = s.last().unwrap().trim().split_ascii_whitespace().map(|x| String::from(x)).collect();

        values.push((signals, output));
    }

    values
}

pub fn d8a() {
    let values = d8values();

    let mut count = 0;

    for (_, output) in values {
        count += output.into_iter().filter(|x| (x.len() != 5) && (x.len() != 6)).collect::<Vec<String>>().len();
    }

    println!("{}", count);
}

pub fn d8b() {
    let values = d8values();

}
