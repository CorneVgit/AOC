use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

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
    history: HashMap<&'a str, bool>,
    state: bool,
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

            let l = line.trim_start_matches(|l: char| l.is_ascii_punctuation());
            let (name, destinations) = l.split_once(" -> ").unwrap();
            let destinations = destinations.split(", ").collect_vec();

            (
                name,
                Module {
                    module_type,
                    destinations,
                    history: HashMap::new(),
                    state: false,
                },
            )
        })
        .collect();

    let modules_clone = modules.clone();
    for (name, module) in modules
        .iter_mut()
        .filter(|(_name, m)| m.module_type == ModuleType::Conjuction || m.history.is_empty())
    {
        modules_clone.iter().for_each(|(n, m)| {
            if m.destinations.contains(name) {
                module.history.insert(n, false);
            }
        });
    }

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let mut queue: VecDeque<Command> = VecDeque::new();

        queue.push_back(Command {
            source: "button",
            destination: "broadcaster",
            pulse: false,
        });

        while !queue.is_empty() {
            let command = queue.pop_front().unwrap();

            match command.pulse {
                true => high += 1,
                false => low += 1,
            }

            let destination_module = match modules.get_mut(command.destination) {
                Some(d) => d,
                None => {
                    continue;
                }
            };

            let pulse: bool;
            match destination_module.module_type {
                ModuleType::FlipFlop => match command.pulse {
                    true => continue,
                    false => {
                        destination_module.state = !destination_module.state;
                        pulse = destination_module.state;
                    }
                },
                ModuleType::Conjuction => {
                    destination_module
                        .history
                        .insert(command.source, command.pulse);

                    pulse = !destination_module.history.values().all(bool::to_owned);
                }
                ModuleType::Broadcast => pulse = command.pulse,
            }

            for destination in &destination_module.destinations {
                queue.push_back(Command {
                    source: command.destination,
                    destination,
                    pulse,
                })
            }
        }
    }

    let r1 = low * high;

    (r1, 0)
}
