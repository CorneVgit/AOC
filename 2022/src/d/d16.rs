use std::collections::HashMap;

use regex::Regex;
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<String> {
    read_all::<String>("input_16_example")
        .into_iter()
        .map(|value| value.unwrap_infallible())
        .collect()
}

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: usize,
    open_status: bool,
    connected_valves: Vec<String>,
}

impl Valve {
    fn new(flow_rate: usize, connected_valves: Vec<String>) -> Self {
        Valve {
            flow_rate,
            open_status: false,
            connected_valves,
        }
    }
}

pub fn d16() -> (usize, usize) {
    let values = values();

    let mut valves: HashMap<String, Valve> = HashMap::new();

    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
        .unwrap();

    for value in &values {
        match re.captures(value) {
            Some(c) => {
                let id = c.get(1).unwrap().as_str().to_string();
                let flow_rate = c.get(2).unwrap().as_str().parse().unwrap();
                let connected_valves = c
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let valve = Valve::new(flow_rate, connected_valves);
                valves.insert(id, valve);
            }
            None => panic!("Regex failed"),
        }
    }

    let pressure = find_highest_pressure("AA".to_string(), 30, 0, 0, valves);

    let r1 = pressure;
    let r2 = 0;

    (r1, r2)
}

fn find_highest_pressure(
    valve_id: String,
    mut time_left: usize,
    _current_flow: usize,
    _pressure: usize,
    mut valves: HashMap<String, Valve>,
) -> usize {
    let mut p = 0;
    if time_left > 0 {
        for v in valves[&valve_id].connected_valves.clone() {
            let new_pressure = find_highest_pressure(
                v,
                time_left - 1,
                _current_flow,
                _pressure + _current_flow,
                valves.clone(),
            );
            if new_pressure > p {
                p = new_pressure;
            }
        }
    }

    let valve = valves.get_mut(&valve_id).unwrap();
    let mut flow_increase = 0;
    // open valve
    if time_left > 0 && valve.flow_rate > 0 && !valve.open_status {
        valve.open_status = true;
        flow_increase += valve.flow_rate;
        time_left -= 1;
    }

    if time_left > 0 {
        for v in valves[&valve_id].connected_valves.clone() {
            let new_pressure = find_highest_pressure(
                v,
                time_left - 1,
                _current_flow,
                _pressure + _current_flow + flow_increase,
                valves.clone(),
            );
            if new_pressure > p {
                p = new_pressure;
            }
        }
    }

    if time_left > 0 {
        p + _pressure
    } else {
        _pressure
    }
}
