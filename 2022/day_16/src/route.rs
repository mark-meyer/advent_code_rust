use std::collections::{VecDeque, HashSet, HashMap};
use crate::graph::DistMatrix;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    // Valves is be a bit field where 1 represents a value that is off
    pub pos: u16,
    pub valves: u16,
    pub time: u16,
    pub flow: u16
}

pub fn max_tour(distances: &DistMatrix, state: State, flows: &Vec<u16>) -> HashMap<u16, u16> {

    let mut seen:HashSet<State> = HashSet::new();
    let mut q:VecDeque<State> = VecDeque::from([state]);
    let mut maxes:HashMap<u16, u16> = HashMap::new();

    while let Some(s) = q.pop_front() {
        if s.time <= 0 {
            continue
        }
        for n in 1..flows.len() {
            // is this valve already on?
            if s.valves & (1 << n ) == 0 {
                let d = distances[s.pos as usize][n] + 1;
                if d >= s.time {
                    continue
                }

                let flow = (s.time - d) * flows[n] + s.flow;
                // turn on the valve
                let valves = s.valves | (1 << n);
                let state = State {pos: n as u16, valves, flow, time: s.time - d };

                let m = maxes.get(&state.valves).cloned().unwrap_or_default();
                maxes.insert(state.valves, m.max(flow));

                if !seen.contains(&state) {
                    q.push_back(state);
                    seen.insert(state);
                }
            }
        }
    }
    maxes
}
