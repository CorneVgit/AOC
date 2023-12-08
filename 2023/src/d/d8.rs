use std::collections::HashMap;

use itertools::Itertools;
use num::integer::lcm;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn get_input() -> Vec<String> {
    let result_values = read_all::<String>("input_8");

    result_values
        .into_iter()
        .map(|result_values| result_values.unwrap_infallible())
        .collect()
}

pub fn d8() -> (u64, u64) {
    let input = get_input();

    let network = HashMap::from_iter(input[2..].iter().map(|line| {
        let (node, next) = line.split_once(" = ").unwrap();
        let (left, right) = next
            .split(',')
            .map(|x| {
                x.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .collect_tuple()
            .unwrap();

        (node, (left, right))
    }));

    let r1 = calc_steps("AAA", "ZZZ", input[0].chars().cycle(), &network);

    let r2 = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|start_node| calc_steps(start_node, "Z", input[0].chars().cycle(), &network))
        .reduce(lcm)
        .unwrap();

    (r1, r2)
}

fn calc_steps(
    start_node: &str,
    end_node: &str,
    mut navigation: std::iter::Cycle<std::str::Chars<'_>>,
    network: &HashMap<&str, (String, String)>,
) -> u64 {
    let mut node = start_node;
    let mut steps = 0;

    while !node.ends_with(end_node) {
        match navigation.next().unwrap() {
            'L' => node = network.get(node).unwrap().0.as_str(),
            'R' => node = network.get(node).unwrap().1.as_str(),
            _ => panic!(),
        }

        steps += 1;
    }

    steps
}
