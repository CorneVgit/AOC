use crate::util::read_all;

fn d6values() -> Vec<usize> {
    let result_values = read_all::<String>("input_6");
    let mut values: Vec<String> = Vec::new();

    for result_value in result_values {
        let value = match result_value {
            Ok(x) => x,
            _ => continue,
        };
        values.push(String::from(value));
    }

    values
        .first()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn d6_slow(days: isize) {
    let mut lanternfishes = d6values();

    for _ in 1..=days {
        let mut new_lanternfish_count = 0;
        for lanternfish in &mut lanternfishes {
            match *lanternfish {
                0 => {
                    *lanternfish = 6;
                    new_lanternfish_count += 1;
                }
                _ => *lanternfish -= 1,
            }
        }

        lanternfishes.extend(vec![8; new_lanternfish_count]);
    }

    println!("{}", lanternfishes.len());
}

pub fn d6_fast(days: isize) {
    let lanternfishes = d6values();

    let mut lanternfish_count: Vec<isize> = vec![0; 9];

    for lanternfish in lanternfishes {
        *&mut lanternfish_count[lanternfish] += 1;
    }

    for _ in 1..=days {
        lanternfish_count.rotate_left(1);
        *&mut lanternfish_count[6] += *&mut lanternfish_count[8];
    }

    println!("{}", lanternfish_count.into_iter().sum::<isize>());
}

pub fn d6_slow_recursive(days: isize) {
    let lanternfishes = d6values();

    let mut lanternfish_count = 0;

    for lanternfish in &lanternfishes {
        lanternfish_count += count_created_lanternfish(*lanternfish, days);
    }

    println!("{}", lanternfish_count);
}

fn count_created_lanternfish(lanternfish: usize, days: isize) -> isize {
    let mut c = 1;

    let mut n = days - lanternfish as isize;
    loop {
        if n <= 0 {
            break;
        }

        c += count_created_lanternfish(8, n - 1);

        n -= 7;
    }

    return c;
}
