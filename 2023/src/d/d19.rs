use std::collections::HashMap;

use itertools::Itertools;

use crate::util::read_all;

fn get_input() -> Vec<String> {
    let result_values = read_all::<String>("input_19");

    result_values
        .into_iter()
        .map(unwrap_infallible::UnwrapInfallible::unwrap_infallible)
        .collect()
}

#[must_use]
pub fn d19() -> (i64, u64) {
    let input = get_input();

    let (workflows, parts) = input.splitn(2, String::is_empty).collect_tuple().unwrap();

    let parts: Vec<HashMap<char, i64>> = parts
        .iter()
        .map(|part| {
            part.split(|c: char| c.is_ascii_punctuation())
                .filter(|s| !s.is_empty())
                .tuples::<(&str, &str)>()
                .map(|(key, value)| (key.chars().next().unwrap(), value.parse::<i64>().unwrap()))
                .collect()
        })
        .collect();

    let workflows: HashMap<&str, Vec<(&str, &str)>> = workflows
        .iter()
        .map(|workflow| {
            let (name, rules) = workflow
                .trim_end_matches(|c: char| c.is_ascii_punctuation())
                .split('{')
                .collect_tuple()
                .unwrap();

            let rules = rules
                .split(',')
                .map(|rule| match rule.split_once(':') {
                    None => ("", rule),
                    Some((condition, destination)) => (condition, destination),
                })
                .collect();

            (name, rules)
        })
        .collect();

    let r1 = parts
        .into_iter()
        .filter(|part| {
            let mut current_workflow_name = "in";

            loop {
                match current_workflow_name {
                    "A" => return true,
                    "R" => return false,
                    _ => (),
                }

                for (condition, destination) in workflows.get(current_workflow_name).unwrap() {
                    if let [key, o, ..] = condition.chars().collect_vec().as_slice() {
                        let part_v = part.get(key).unwrap();
                        let rule_v = &condition[2..].parse().unwrap();
                        if !(o == &'>' && part_v > rule_v || o == &'<' && part_v < rule_v) {
                            continue;
                        }
                    }

                    current_workflow_name = destination;
                    break;
                }
            }
        })
        .fold(0, |acc, part| acc + part.values().sum::<i64>());

    (r1, 0)
}
