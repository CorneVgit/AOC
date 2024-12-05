use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

#[must_use]
fn get_values() -> Vec<Vec<char>> {
    let result_values = read_all::<String>("input_4");

    result_values
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect()
}

#[must_use]
pub fn d4() -> (usize, usize) {
    let field = get_values();

    let r1 = find_all_xmas(&field);
    let r2 = find_all_x_mas(&field);

    (r1, r2)
}

fn find_all_xmas(field: &[Vec<char>]) -> usize {
    let mut count = 0;

    for x in 0..field.len() as u32 {
        for y in 0..field[x as usize].len() as u32 {
            if field[x as usize][y as usize] == 'X' {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if search_direction(x, y, dx, dy, field, "MAS") {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

fn find_all_x_mas(field: &[Vec<char>]) -> usize {
    let mut count = 0;

    for x in 0..field.len() as u32 {
        for y in 0..field[x as usize].len() as u32 {
            if field[x as usize][y as usize] == 'A'
                && (
                    // Check if first diagonal == MAS
                    search_direction(x, y, 1, 1, field, "M") && search_direction(x, y, -1, -1, field, "S")
                    // If first diagonal != MAS, check if first diagonal == SAM
                    || search_direction(x, y, -1, -1, field, "M") && search_direction(x, y, 1, 1, field, "S")
                )
                && (
                    // Check if second diagonal == MAS
                    search_direction(x, y, 1, -1, field, "M") && search_direction(x, y, -1, 1, field, "S")
                    // If second diagonal != MAS, check if second diagonal == SAM
                    || search_direction(x, y, -1, 1, field, "M") && search_direction(x, y, 1, -1, field, "S")
                )
            {
                count += 1;
            }
        }
    }

    count
}

fn search_direction(x: u32, y: u32, dx: i64, dy: i64, field: &[Vec<char>], pattern: &str) -> bool {
    let mut cx = i64::from(x) + dx;
    let mut cy = i64::from(y) + dy;

    for ch in pattern.chars() {
        if cx < 0 || cy < 0 || cx >= field.len() as i64 || cy >= field.len() as i64 {
            return false;
        }

        if field[cx as usize][cy as usize] == ch {
            cx += dx;
            cy += dy;
        } else {
            return false;
        }
    }

    true
}
