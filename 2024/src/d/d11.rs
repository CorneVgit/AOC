use crate::util::read_single_string;

#[must_use]
fn get_input() -> Vec<u64> {
    read_single_string("input_11")
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[must_use]
pub fn d11() -> (usize, usize) {
    let mut stones = get_input();

    for i in 0..15 {
        stones = stones
            .iter()
            .flat_map(|stone| match stone {
                0 => Vec::from([1]),
                _ if stone.to_string().len() % 2 == 0 => {
                    let s = stone.to_string();
                    let (v1, v2) = s.split_at(s.len() / 2);
                    Vec::from([v1.parse().unwrap(), v2.parse().unwrap()])
                }
                _ => Vec::from([stone * 2024]),
            })
            .collect();
        // println!("{:?}", stones);
        println!("{}: {}", i + 1, stones.len());
    }

    // let stones = get_input();

    // println!("{}", stones.iter().map(|s| rec(*s, 25)).sum::<usize>());

    (0, 0)
}

// fn rec(stone: u64, n: u32) -> usize {
//     if n == 0 {
//         return 1
//     }
//     match stone {
//         0 => rec(1, n - 1),
//         _ if stone.ilog10() % 2 == 0 => {
//             let s = stone.to_string();
//             let (v1, v2) = s.split_at(s.len() / 2);
//             rec(v1.parse().unwrap(), n - 1) + rec(v2.parse().unwrap(), n - 1)
//         }
//         _ =>
//         rec(stone * 2024, n - 1),
//     }
// }
