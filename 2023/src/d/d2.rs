use itertools::Itertools;

use crate::util::read_all;

fn get_games() -> Vec<String> {
    let result_values = read_all::<String>("input_2");

    result_values
        .into_iter()
        .map(unwrap_infallible::UnwrapInfallible::unwrap_infallible)
        .collect()
}

#[must_use] pub fn d2() -> (i32, i32) {
    const BAG_RED: i32 = 12;
    const BAG_GREEN: i32 = 13;
    const BAG_BLUE: i32 = 14;

    let games = get_games();

    let mut r1 = 0;

    'games: for game in &games {
        let (id, sets) = game.split_once(": ").unwrap();
        let id = id
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let sets = sets.replace(';', ",");

        for cubes in sets.split(", ").collect_vec() {
            let n = cubes
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            if cubes.contains("red") && n > BAG_RED
                || cubes.contains("green") && n > BAG_GREEN
                || cubes.contains("blue") && n > BAG_BLUE
            {
                continue 'games;
            }
        }

        r1 += id;
    }

    let mut r2 = 0;

    for game in &games {
        let sets = game.split_once(": ").unwrap().1;

        let mut red = i32::MIN;
        let mut green = i32::MIN;
        let mut blue = i32::MIN;

        let sets = sets.replace(';', ",");

        for cubes in sets.split(", ").collect_vec() {
            let n = cubes
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            if cubes.contains("red") && n > red {
                red = n;
            } else if cubes.contains("green") && n > green {
                green = n;
            } else if cubes.contains("blue") && n > blue {
                blue = n;
            }
        }

        r2 += red * green * blue;
    }

    (r1, r2)
}
