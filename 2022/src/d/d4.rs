use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<String> {
    read_all::<String>("input_4")
        .into_iter()
        .map(|rucksack| rucksack.unwrap_infallible())
        .collect()
}

pub fn d4() -> (u32, u32) {
    let pairs = values();

    let mut r1 = 0;
    let mut r2 = 0;

    for pair in pairs {
        let (e1, e2) = pair.split_once(',').unwrap();
        let (e1_lb, e1_ub) = e1.split_once('-').unwrap();
        let (e2_lb, e2_ub) = e2.split_once('-').unwrap();

        let e1_lb: i32 = e1_lb.parse().unwrap(); // elf 1 lower bound
        let e1_ub: i32 = e1_ub.parse().unwrap(); // elf 1 upper bound
        let e2_lb: i32 = e2_lb.parse().unwrap(); // elf 2 lower bound
        let e2_ub: i32 = e2_ub.parse().unwrap(); // elf 2 upper bound

        if e1_lb <= e2_lb && e1_ub >= e2_lb || e1_lb >= e2_lb && e1_lb <= e2_ub {
            // Partly contains
            r2 += 1;

            if e1_lb <= e2_lb && e1_ub >= e2_ub || e1_lb >= e2_lb && e1_ub <= e2_ub {
                // Fully contains
                r1 += 1;
            }
        }
    }

    (r1, r2)
}
