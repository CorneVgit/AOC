use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<String> {
    let result_values = read_all::<String>("input_2");
    let mut games: Vec<String> = Vec::new();

    for result_value in result_values {
        let game = result_value.unwrap_infallible();
        games.push(game);
    }

    games
}

pub fn d2() -> (u32, u32) {
    let games = values();

    let mut score = 0;
    for game in &games {
        score += match game.as_str() {
            // Loss
            "B X" => 1,
            "C Y" => 2,
            "A Z" => 3,
            // Tie
            "A X" => 4,
            "B Y" => 5,
            "C Z" => 6,
            // Win
            "C X" => 7,
            "A Y" => 8,
            "B Z" => 9,
            _ => panic!("Invalid input"),
        }
    }

    let r1 = score;

    let mut score = 0;
    for game in &games {
        score += match game.as_str() {
            // Loss
            "A X" => 3,
            "B X" => 1,
            "C X" => 2,
            // Tie
            "A Y" => 4,
            "B Y" => 5,
            "C Y" => 6,
            // Win
            "A Z" => 8,
            "B Z" => 9,
            "C Z" => 7,
            _ => panic!("Invalid input"),
        }
    }

    let r2 = score;

    (r1, r2)
}
