use std::collections::VecDeque;

// use unwrap_infallible::UnwrapInfallible;

// use crate::util::read_all;

// fn values() -> Vec<String> {
//     read_all::<String>("input_11")
//         .into_iter()
//         .map(|value| value.unwrap_infallible())
//         .collect()
// }

#[derive (Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    test_div: u64,
    test_true: usize,
    test_false: usize,
    count: u64,
}

impl Monkey {
    fn test(&self, val: u64) -> usize {
        match val % self.test_div == 0 {
            true => self.test_true,
            false => self.test_false,
        }
    }
}

pub fn d11() -> (u64, u64) {
    // let values = values();

    let mut monkeys: Vec<Monkey> = vec![
        Monkey {
            items: VecDeque::from([78, 53, 89, 51, 52, 59, 58, 85]),
            operation: |x| x * 3,
            test_div: 5,
            test_true: 2,
            test_false: 7,
            count: 0,
        },
        Monkey {
            items: VecDeque::from([64]),
            operation: |x| x + 7,
            test_div: 2,
            test_true: 3,
            test_false: 6,
            count: 0,
        },
        Monkey {
            items: VecDeque::from([71, 93, 65, 82]),
            operation: |x| x + 5,
            test_div: 13,
            test_true: 5,
            test_false: 4,
            count: 0,
        },
        Monkey {
            items: VecDeque::from([67, 73, 95, 75, 56, 74]),
            operation: |x| x + 8,
            test_div: 19,
            test_true: 6,
            test_false: 0,
            count: 0,
        },
        Monkey {
            items: VecDeque::from([85, 91, 90]),
            operation: |x| x + 4,
            test_div: 11,
            test_true: 3,
            test_false: 1,
            count: 0,
        },
        Monkey {
            items: VecDeque::from([67, 96, 69, 55, 70, 83, 62]),
            operation: |x| x * 2,
            test_div: 3,
            test_true: 4,
            test_false: 1,
            count: 0,
        },
        Monkey {
            items: VecDeque::from([53, 86, 98, 70, 64]),
            operation: |x| x + 6,
            test_div: 7,
            test_true: 7,
            test_false: 0,
            count: 0,
        },
        Monkey {
            items: VecDeque::from([88, 64]),
            operation: |x| x * x,
            test_div: 17,
            test_true: 2,
            test_false: 5,
            count: 0,
        },
    ];

    let mut monkeys2 = monkeys.clone();

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            while !monkeys[index].items.is_empty() {
                let mut item = monkeys[index].items.pop_front().unwrap();
                item = (monkeys[index].operation)(item) / 3;
                let i = monkeys[index].test(item);
                monkeys[index].count += 1;
                monkeys[i].items.push_back(item);
            }
        }
    }

    for _ in 0..10000 {
        for index in 0..monkeys2.len() {
            while !monkeys2[index].items.is_empty() {
                let mut item = monkeys2[index].items.pop_front().unwrap();
                item = (monkeys[index].operation)(item);
                item %= monkeys2.iter().map(|x| x.test_div).product::<u64>();
                let i = monkeys2[index].test(item);
                monkeys2[index].count += 1;
                monkeys2[i].items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.count.cmp(&a.count));
    monkeys2.sort_by(|a, b| b.count.cmp(&a.count));

    let r1 = monkeys.iter().map(|x| x.count).collect::<Vec<u64>>()[0..=1].iter().product();
    let r2 = monkeys2.iter().map(|x| x.count).collect::<Vec<u64>>()[0..=1].iter().product();

    (r1, r2)
}
