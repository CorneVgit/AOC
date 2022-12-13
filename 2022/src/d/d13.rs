use std::str::Chars;

use itertools::Itertools;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<String> {
    read_all::<String>("input_13")
        .into_iter()
        .map(|value| value.unwrap_infallible())
        .filter(|value| !value.is_empty())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Item::Number(n1) => match other {
                Item::Number(n2) => n1.cmp(n2),
                Item::List(_) => Item::List(vec![self.clone()]).cmp(other),
            },
            Item::List(l1) => match other {
                Item::Number(_) => self.cmp(&Item::List(vec![other.clone()])),
                Item::List(l2) => {
                    for i in 0..l1.len() {
                        if i < l2.len() {
                            match l1[i].cmp(&l2[i]) {
                                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                                std::cmp::Ordering::Equal => continue,
                                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                            }
                        } else {
                            return std::cmp::Ordering::Greater;
                        }
                    }

                    if l1.len() == l2.len() {
                        return std::cmp::Ordering::Equal;
                    }

                    std::cmp::Ordering::Less
                }
            },
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Item {
    fn new(input: &str) -> Item {
        fn parse_item_(input: &mut Chars) -> Vec<Item> {
            let mut v: Vec<Item> = Vec::new();

            let mut next_num: String = "".to_string();

            while let Some(c) = input.next() {
                if c.is_ascii_digit() {
                    next_num.push(c)
                } else if c == '[' {
                    v.push(Item::List(parse_item_(input)));
                } else if c == ']' {
                    if !next_num.is_empty() {
                        v.push(Item::Number(next_num.parse().unwrap()));
                        next_num.clear()
                    }
                    break;
                } else if c == ',' {
                    if !next_num.is_empty() {
                        v.push(Item::Number(next_num.parse().unwrap()));
                        next_num.clear()
                    }
                } else {
                    panic!("Unexpected character");
                }
            }

            v
        }

        Item::List(parse_item_(&mut input.chars()))
    }
}

pub fn d13() -> (usize, usize) {
    let mut packets = Vec::new();

    let mut r1 = 0;

    for (i, (p1, p2)) in values().iter().tuples().enumerate() {
        let item1 = Item::new(p1);
        let item2 = Item::new(p2);

        if item1.cmp(&item2) != std::cmp::Ordering::Greater {
            r1 += i + 1;
        }

        packets.push(item1);
        packets.push(item2);
    }

    let markers = vec![
        Item::new("[[2]]"),
        Item::new("[[6]]"),
    ];

    packets.append(&mut markers.clone());
    packets.sort();

    let r2 = markers
        .iter()
        .map(|m| packets.binary_search(m).unwrap() + 1)
        .product();

    (r1, r2)
}
