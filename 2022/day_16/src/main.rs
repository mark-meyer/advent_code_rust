use std::fs;
use std::error::Error;
use std::path::Path;
use regex::Regex;
use std::collections::{HashMap, VecDeque, HashSet};

static DATA: &str = "data.txt";

type ValveMap = HashMap<String, Valve>;
type DistMatrix  = Vec<Vec<u32>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    // Valves is be a bit field where 1 represents a value that is off
    pos: u32,
    valves: u32,
    time: u32,
    flow: u32
}

#[derive(Debug)]
struct Valve {
    flow: usize,
    edges: Vec<String>
}

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

fn bfs(map: &ValveMap, from_key: &str, to_key: &str) -> Option<usize> {
    let mut q = VecDeque::new();
    let mut marked:HashSet<&String> = HashSet::new();

    q.push_back((from_key, 0));

    while let Some((current, dist)) = q.pop_front() {
        if current == to_key {
            return Some(dist)
        }
        if let Some(Valve{flow: _, edges}) = map.get(current) {
            for edge in edges {
                if !marked.contains(edge) {
                    q.push_back((edge, dist+1));
                    marked.insert(edge);
                }
            }
        }
    };
    None
}


fn distances(valve_map: &ValveMap, targets: &Vec<String>) -> (DistMatrix, Vec<u32>) {
    let n:usize = targets.len() + 1;

    let mut matrix = vec![vec![0u32; n]; n];
    let mut flows = Vec::from([0u32]);

    for (i, from) in targets.iter().enumerate() {
        let from_origin = bfs(valve_map, "AA", from).unwrap();
        let vm = valve_map.get(from).and_then(|v| Some(v.flow as u32)).unwrap();
        flows.push(vm);
        matrix[0][i+1] = from_origin as u32;
        matrix[i+1][0] = from_origin as u32;

        for (j, to) in targets[i..].iter().enumerate() {
            let d = bfs(valve_map, from, to).unwrap();
            matrix[i+1][j+i+1] = d as u32;
            matrix[j+i+1][i+1] = d as u32;
        }
    }
    (matrix, flows)
}

fn max_tour(distances: &DistMatrix, flows: &Vec<u32>, time:u32) -> u32 {
    let state = State {pos: 0, valves: u32::pow(2, flows.len() as u32) - 2, time:time, flow: 0};

    let mut seen:HashSet<State> = HashSet::new();
    let mut q:VecDeque<State> = VecDeque::from([state]);
    let mut best = 0;

    while let Some(s) = q.pop_front() {
        if s.flow > best {
            best = s.flow
        }
        if s.time <= 0 {
            continue
        }
        for n in 1..=flows.len() {
            if s.valves & (1 << n ) > 0 {
                let d = distances[s.pos as usize][n] + 1;
                if d >= s.time {
                    continue
                }

                let flow = (s.time - d) * flows[n] + s.flow;
                let valves = s.valves & !(1 << n);

                let state = State {pos: n as u32, valves, flow, time: s.time - d };

                if !seen.contains(&state) {
                    q.push_back(state);
                    seen.insert(state);
                }
            }
        }
    }
    best
}


fn main() {
    let p  = Path::new(DATA);
    let (valve_map, targets) = parse_input(p).unwrap();
    let (dist_matrix, flows) = distances(&valve_map, &targets);
    println!("Part One: {}",  max_tour(&dist_matrix, &flows, 30));
}
