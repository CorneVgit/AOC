use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use num::integer::lcm;

use crate::util::read_all;

fn get_input() -> Vec<String> {
    let result_values = read_all::<String>("input_20");

    result_values
        .into_iter()
        .map(unwrap_infallible::UnwrapInfallible::unwrap_infallible)
        .collect()
}

#[derive(PartialEq, Clone)]
enum ModuleType {
    FlipFlop,
    Conjuction,
    Broadcast,
}

#[derive(Clone)]
struct Module<'a> {
    module_type: ModuleType,
    destinations: Vec<&'a str>,
    sources: Vec<&'a str>,
    state: bool,
    last_pulse: bool,
}

struct Command<'a> {
    source: &'a str,
    destination: &'a str,
    pulse: bool,
}

#[must_use]
pub fn d20() -> (u64, u64) {
    let input = get_input();

    let mut modules: HashMap<&str, Module> = input
        .iter()
        .map(|line| {
            let module_type = match line.chars().next().unwrap() {
                '%' => ModuleType::FlipFlop,
                '&' => ModuleType::Conjuction,
                'b' => ModuleType::Broadcast,
                _ => panic!(),
            };

            let (name, destinations) = line
                .trim_start_matches(|l: char| l.is_ascii_punctuation())
                .split_once(" -> ")
                .unwrap();
            let destinations = destinations.split(", ").collect_vec();

            (
                name,
                Module {
                    module_type,
                    destinations,
                    sources: Vec::new(),
                    state: false,
                    last_pulse: false,
                },
            )
        })
        .collect();

    for (source_name, source_module) in &modules.clone() {
        for (name, module) in &mut modules {
            if source_module.destinations.contains(name) {
                module.sources.push(source_name);
            }
        }
    }

    let rx_source = modules
        .iter()
        .find(|x| x.1.destinations.contains(&"rx"))
        .unwrap()
        .0
        .to_owned();
    let mut occ: HashMap<&str, u64> = HashMap::new();

    let mut low = 0;
    let mut high = 0;

    let mut r1 = 0;

    for i in 1..u64::MAX {
        let mut queue: VecDeque<Command> = VecDeque::new();

        queue.push_back(Command {
            source: "button",
            destination: "broadcaster",
            pulse: false,
        });

        if occ.len() == modules.get(rx_source).unwrap().sources.len() && i > 1000 {
            break;
        }

        while !queue.is_empty() {
            let command = queue.pop_front().unwrap();

            if modules
                .get(rx_source)
                .unwrap()
                .sources
                .contains(&command.source)
                && command.pulse
            {
                occ.entry(command.source).or_insert(i);
            }

            match command.pulse {
                true => high += 1,
                false => low += 1,
            }

            let mut destination_module = match modules.get(command.destination).cloned() {
                Some(d) => d,
                None => {
                    continue;
                }
            };

            match destination_module.module_type {
                ModuleType::FlipFlop => match command.pulse {
                    true => continue,
                    false => {
                        destination_module.state = !destination_module.state;
                        destination_module.last_pulse = destination_module.state;
                    }
                },
                ModuleType::Conjuction => {
                    destination_module.last_pulse = !destination_module
                        .sources
                        .iter()
                        .all(|s| modules.get(s).unwrap().last_pulse);
                }
                ModuleType::Broadcast => destination_module.last_pulse = command.pulse,
            }

            for destination in &destination_module.destinations {
                queue.push_back(Command {
                    source: command.destination,
                    destination,
                    pulse: destination_module.last_pulse,
                })
            }

            modules.insert(command.destination, destination_module);
        }

        if i == 1000 {
            r1 = low * high;
        }
    }

    let r2 = occ.into_values().reduce(lcm).unwrap();

    (r1, r2)
}
