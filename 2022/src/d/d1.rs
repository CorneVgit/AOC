use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_carried_calories() -> Vec<u32> {
    let result_values = read_all::<String>("input_1");
    let mut carried_calories: Vec<u32> = Vec::new();

    let mut total_calories = 0;
    for result_value in result_values {
        match result_value.unwrap_infallible().parse::<u32>() {
            Ok(calories) => total_calories += calories,
            Err(_) => {
                carried_calories.push(total_calories);
                total_calories = 0;
            }
        };
    }

    carried_calories
}

pub fn d1() -> (u32, u32) {
    let mut carried_calories = get_carried_calories();
    carried_calories.sort();
    let r1 = carried_calories.iter().rev().take(1).sum::<u32>();
    let r2 = carried_calories.iter().rev().take(3).sum::<u32>();
    (r1, r2)
}
