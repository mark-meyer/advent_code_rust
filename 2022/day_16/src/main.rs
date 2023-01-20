use std::fs;
use std::error::Error;
use std::path::Path;
use regex::Regex;
use std::collections::{HashMap};
use route::{State, max_tour};
use graph::{ValveMap, Valve, distances};

static DATA: &str = "data.txt";

mod route;
mod graph;

fn parse_input(p: &Path) -> Result<(ValveMap, Vec<String>), Box<dyn Error>> {
    let f = fs::read_to_string(p)?;
    let rx = Regex::new(r"[A-Z]{2}|\d+")?;

    let mut valve_map:HashMap<String, Valve> = HashMap::new();
    let mut targets = vec![];

    for line in f.lines() {
        let valves:Vec<_> = rx.find_iter(&line)
            .map(|valve_name| valve_name.as_str())
            .collect();

        let flow = valves[1].parse()?;
        if flow > 0 {
            targets.push(valves[0].to_string());
        }
        let edges = valves[2..].iter().map(|s| s.to_string()).collect();

        valve_map.insert(valves[0].to_string(), Valve{flow, edges});
    }

     Ok((valve_map, targets))
}

fn main() {
    let p  = Path::new(DATA);
    let (valve_map, targets) = parse_input(p).unwrap();
    let (dist_matrix, flows) = distances(&valve_map, &targets);

    /* Part One */
    let state = State {pos: 0, valves: 0, time:30, flow: 0};
    let maxes = max_tour(&dist_matrix, state, &flows);

    println!("Part One: {:?}",  maxes.values().max().unwrap());

    /* Part Two */
    let state = State {pos: 0, valves:0, time:26, flow: 0};
    let maxes = max_tour(&dist_matrix, state, &flows);

    let mut max = 0;

    // look at combinations of valves that have non in common
    // and find the largest sum
    for (k1, v1) in maxes.iter() {
        for (k2, v2) in maxes.iter() {
            if  k1 & k2 == 0 && v1 + v2 > max {
                max = v1 + v2;
            }
        }
    }
    println!("Part two: {:?}", max);

}
