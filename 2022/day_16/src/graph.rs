use std::collections::{HashMap, VecDeque, HashSet};

pub type ValveMap = HashMap<String, Valve>;
pub type DistMatrix  = Vec<Vec<u16>>;

pub struct Valve {
    pub flow: usize,
    pub edges: Vec<String>
}

pub fn bfs(map: &ValveMap, from_key: &str, to_key: &str) -> Option<usize> {
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

pub fn distances(valve_map: &ValveMap, targets: &Vec<String>) -> (DistMatrix, Vec<u16>) {
    let n:usize = targets.len() + 1;

    let mut matrix = vec![vec![0u16; n]; n];
    let mut flows = Vec::from([0u16]);

    for (i, from) in targets.iter().enumerate() {
        let from_origin = bfs(valve_map, "AA", from).unwrap();
        let vm = valve_map.get(from).and_then(|v| Some(v.flow)).unwrap();

        flows.push(vm as u16);
        matrix[0][i+1] = from_origin as u16;
        matrix[i+1][0] = from_origin as u16;

        for (j, to) in targets[i..].iter().enumerate() {
            let d = bfs(valve_map, from, to).unwrap();
            matrix[i+1][j+i+1] = d as u16;
            matrix[j+i+1][i+1] = d as u16;
        }
    }
    (matrix, flows)
}