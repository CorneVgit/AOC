use crate::util::read_all;

fn d3values() -> Vec<String> {
    let result_values = read_all::<String>("input_3");
    let mut values: Vec<String> = Vec::new();

    for result_value in result_values {
        let value = match result_value {
            Ok(x) => x,
            _ => continue,
        };
        values.push(String::from(value));
    }

    values
}

pub fn d3a() {
    let values = d3values();

    let binary_num_length = values.first().expect("no value").len();

    let mut gamma_rate_string = String::new();

    for i in 0..binary_num_length {
        let mut zeros = 0;
        let mut ones = 0;

        for value in &values {
            match value.chars().nth(i) {
                Some('0') => zeros += 1,
                Some('1') => ones += 1,
                _ => continue,
            }
        }

        if ones >= zeros {
            gamma_rate_string.push('1');
        } else {
            gamma_rate_string.push('0');
        }
    }

    let gamma_rate = usize::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate = !gamma_rate & (1 << binary_num_length) - 1;

    println!("{}", gamma_rate * epsilon_rate);
}

pub fn d3b() {
    let values = d3values();

    let binary_num_length = values.first().expect("no value").len();

    let mut possible_og_ratings: Vec<String> = values.clone();
    let mut possible_cs_ratings: Vec<String> = values.clone();

    for i in 0..binary_num_length {
        let mut zeros = 0;
        let mut ones = 0;

        for possible_og_rating in &possible_og_ratings {
            match possible_og_rating.chars().nth(i) {
                Some('0') => zeros += 1,
                Some('1') => ones += 1,
                _ => continue,
            }
        }

        if possible_og_ratings.len() == 1 {
            break;
        }

        if ones >= zeros {
            possible_og_ratings.retain(|x| x.chars().nth(i).unwrap() == '0');
        } else {
            possible_og_ratings.retain(|x| x.chars().nth(i).unwrap() == '1');
        }
    }

    for i in 0..binary_num_length {
        let mut zeros = 0;
        let mut ones = 0;

        for possible_cs_rating in &possible_cs_ratings {
            match possible_cs_rating.chars().nth(i) {
                Some('0') => zeros += 1,
                Some('1') => ones += 1,
                _ => continue,
            }
        }

        if possible_cs_ratings.len() == 1 {
            break;
        }

        if ones >= zeros {
            possible_cs_ratings.retain(|x| x.chars().nth(i).unwrap() == '1');
        } else {
            possible_cs_ratings.retain(|x| x.chars().nth(i).unwrap() == '0');
        }
    }

    let oxygen_generator_rating =
        usize::from_str_radix(possible_og_ratings.first().unwrap(), 2).unwrap();
    let co2_scrubber_rating =
        usize::from_str_radix(possible_cs_ratings.first().unwrap(), 2).unwrap();

    println!("{}", oxygen_generator_rating * co2_scrubber_rating);
}
