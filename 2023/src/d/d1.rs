use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_calibration_values() -> Vec<String> {
    let result_values = read_all::<String>("input_1");

    result_values
        .into_iter()
        .map(|result_values| result_values.unwrap_infallible())
        .collect()
}

pub fn d1() -> (i64, usize) {
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

        r1 += first_val * 10 + last_val;
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
            if let Some(loc) = calibration_value.find(v) {
                if (loc as i64) < first_loc {
                    first_loc = loc as i64;
                    first_val = i % 10;
                }
            }
            if let Some(loc) = calibration_value.rfind(v) {
                if (loc as i64) > last_loc {
                    last_loc = loc as i64;
                    last_val = i % 10;
                }
            }
        }

        r2 += first_val * 10 + last_val;
    }

    (r1, r2)
}
