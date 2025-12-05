use std::collections::HashSet;

use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<Vec<char>> {
    read_all::<String>("input_4")
        .into_iter()
        .map(UnwrapInfallible::unwrap_infallible)
        .map(|s| s.chars().collect())
        .collect()
}

#[must_use]
pub fn d4() -> (usize, usize) {
    let input = get_input();

    let mut paper_locations = HashSet::new();

    for (x, row) in input.iter().enumerate() {
        for (y, location) in row.iter().enumerate() {
            if *location == '@' {
                paper_locations.insert((x as isize, y as isize));
            }
        }
    }

    let mut old_len = paper_locations.len();

    paper_locations = remove_paper(&paper_locations);

    let r1 = old_len - paper_locations.len();

    let mut r2 = r1;

    loop {
        old_len = paper_locations.len();

        paper_locations = remove_paper(&paper_locations);

        let removed = old_len - paper_locations.len();

        if removed == 0 {
            break;
        }

        r2 += removed;
    }

    (r1, r2)
}

fn remove_paper(paper_locations: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    paper_locations
        .iter()
        .filter(|location| check_neighbours(paper_locations, location))
        .copied()
        .collect()
}

fn check_neighbours(paper_locations: &HashSet<(isize, isize)>, location: &(isize, isize)) -> bool {
    let mut count = 0;

    for x in location.0 - 1..=location.0 + 1 {
        for y in location.1 - 1..=location.1 + 1 {
            if paper_locations.contains(&(x, y)) {
                count += 1;
            }
        }
    }

    count > 4
}
