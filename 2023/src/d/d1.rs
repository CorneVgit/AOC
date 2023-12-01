use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_calibration_values() -> Vec<String> {
    let result_values = read_all::<String>("input_1");

    result_values
        .into_iter()
        .map(|result_values| result_values.unwrap_infallible())
        .collect()
}

pub fn d1() -> (i64, i64) {
    let calibration_values = get_calibration_values();

    let mut r1 = 0;

    for calibration_value in &calibration_values {
        let mut first_val: i64 = i64::MIN;
        let mut last_val: i64 = i64::MAX;

        for c in calibration_value.chars() {
            if c.is_ascii_digit() {
                if first_val == i64::MIN {
                    first_val = c.to_digit(10).unwrap().into();
                }

                last_val = c.to_digit(10).unwrap().into();
            }
        }

        let val = first_val.to_string() + &last_val.to_string();
        r1 += val.parse::<i64>().unwrap();
    }

    let mut r2 = 0;

    for calibration_value in &calibration_values {
        let mut first_val = 0;
        let mut last_val = 0;

        let mut first_loc = i64::MAX;
        let mut last_loc = i64::MIN;

        let lut: Vec<&str> = vec![
            "ඞ", "1", "2", "3", "4", "5", "6", "7", "8", "9", "ඞ", "one", "two", "three", "four",
            "five", "six", "seven", "eight", "nine",
        ];

        for (i, v) in lut.into_iter().enumerate() {
            if let Some(m) = calibration_value.find(v) {
                if (m as i64) < first_loc {
                    first_loc = m as i64;
                    first_val = i % 10;
                }
            }
            if let Some(m) = calibration_value.rfind(v) {
                if (m as i64) > last_loc {
                    last_loc = m as i64;
                    last_val = i % 10;
                }
            }
        }

        let val = first_val.to_string() + &last_val.to_string();
        r2 += val.parse::<i64>().unwrap();
    }

    (r1, r2)
}
