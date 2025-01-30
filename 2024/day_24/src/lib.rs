use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum Gate {
    And(Box<Gate>, Box<Gate>),
    Xor(Box<Gate>, Box<Gate>),
    Or(Box<Gate>, Box<Gate>),
    Wire(u8)
}

impl Gate {
    pub fn value(&self) -> u8 {
        match self {
            Gate::And(a, b) => a.value() & b.value(),
            Gate::Xor(a, b) => a.value() ^ b.value(),
            Gate::Or(a, b) =>  a.value() | b.value(),
            Gate::Wire(v) => *v
        }
    }
}

pub fn topologic_sort( gates_lines: &Vec<Vec<&str>>, wires: &HashMap<String, u8> ) -> Vec<String> {
    let mut gates:HashMap<&str, HashSet<&str>> = HashMap::new();

    for gate_line in gates_lines.iter() { 
        gates.entry(gate_line[4])
            .or_default()
            .extend([gate_line[0], gate_line[2]]);
    }


    let mut sources:Vec<&str> =  wires.keys().map(|k| k.as_str()).collect(); 
    let mut sorted = vec![];

    while let Some(n) = sources.pop() {
        sorted.push(n.to_string());
        for (k, edges) in gates.iter_mut() {
            if edges.contains(n){
                edges.remove(n);
            if edges.is_empty() {
                sources.push(k);
            }
            }
        }
    }
    sorted
}