use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<String> {
    let result_values = read_all::<String>("input_3");

    result_values
        .into_iter()
        .map(|result_values| result_values.unwrap_infallible())
        .collect()
}

pub fn d3() -> (i32, i32) {
    let input = get_input();

    let mut field: Vec<Vec<char>> = Vec::new();

    let mut region = (-1, -1, -1);

    let mut regions: Vec<(i32, i32, i32)> = Vec::new();

    for line in input {
        field.push(line.chars().collect_vec());
    }

    let mut r1 = 0;

    for (i, row) in field.clone().into_iter().enumerate() {
        for (j, value) in row.into_iter().enumerate() {
            if value.is_ascii_digit() {
                if region == (-1, -1, -1) {
                    region = (i as i32, j as i32, j as i32)
                } else {
                    region = (i as i32, region.1, j as i32)
                }
            } else {
                process_part_number(&mut region, &field, &mut r1, &mut regions);
            }
        }

        process_part_number(&mut region, &field, &mut r1, &mut regions)
    }

    let mut r2 = 0;

    for (i, row) in field.clone().into_iter().enumerate() {
        for (j, value) in row.into_iter().enumerate() {
            let mut reg: Vec<(i32, i32, i32)> = Vec::new();
            if value == '*' {
                for k in (i as i32 - 1)..=(i as i32 + 1) {
                    if k == -1 {
                        continue;
                    }
                    for l in (j as i32 - 1)..=(j as i32 + 1) {
                        if l == -1 {
                            continue;
                        }

                        for region in &regions {
                            if region.0 == k && l >= region.1 && l <= region.2 {
                                reg.push(*region);
                            }
                        }
                    }
                }

                reg.dedup();
                if reg.len() == 2 {
                    let mut buffer: Vec<char> = Vec::new();
                    let reg1 = reg.first().unwrap();
                    for k in reg1.1..=reg1.2 {
                        let r = field.get(reg1.0 as usize).unwrap();
                        let v = r.get(k as usize).unwrap();
                        buffer.push(*v);
                    }

                    let part_number_s: String = buffer.into_iter().collect();
                    let part_number1 = part_number_s.parse::<i32>().unwrap();

                    let mut buffer: Vec<char> = Vec::new();
                    let reg2 = reg.last().unwrap();
                    for k in reg2.1..=reg2.2 {
                        let r = field.get(reg2.0 as usize).unwrap();
                        let v = r.get(k as usize).unwrap();
                        buffer.push(*v);
                    }

                    let part_number_s: String = buffer.into_iter().collect();
                    let part_number2 = part_number_s.parse::<i32>().unwrap();

                    r2 += part_number1 * part_number2;
                }
            }
        }
    }

    (r1, r2)
}

fn process_part_number(region: &mut (i32, i32, i32), field: &[Vec<char>], r1: &mut i32, regions: &mut Vec<(i32, i32, i32)>) {
    let mut is_part_num = false;

    if *region != (-1, -1, -1) {
        for k in (region.0 - 1)..=(region.0 + 1) {
            if k == -1 {
                continue;
            }
            for l in (region.1 - 1)..=(region.2 + 1) {
                if l == -1 {
                    continue;
                }
                if let Some(r) = field.get(k as usize) {
                    if let Some(v) = r.get(l as usize) {
                        if !v.is_ascii_digit() && *v != '.' {
                            is_part_num = true;
                        }
                    }
                }
            }
        }

        if is_part_num {
            let mut buffer: Vec<char> = Vec::new();
            for k in region.1..=region.2 {
                let r = field.get(region.0 as usize).unwrap();
                let v = r.get(k as usize).unwrap();
                buffer.push(*v);
            }

            let part_number_s: String = buffer.into_iter().collect();
            let part_number = part_number_s.parse::<i32>().unwrap();
            *r1 += part_number;
            regions.push(*region);
        }
    }

    *region = (-1, -1, -1);
}
