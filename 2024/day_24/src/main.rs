use std::collections::HashMap;
use std::fs;
use day_24::*;

fn parse_input(s:&str) -> HashMap<String, Gate>{
    let (wires, gates) = s.split_once("\n\n").unwrap();
    
    // wires have the source values
    let wires:HashMap<String, u8> = HashMap::from_iter(
        wires.lines()
        .map(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            (a.to_string(), b.parse().unwrap())
        })
    );
    
    // Make topo sort of gates to we can
    // wire them together in the right order
    let gate_lines = gates.lines().map(|line| line.split_whitespace().collect()).collect();
    let wire_order = topologic_sort(&gate_lines, &wires);

    // gate lookup is a simple adjancy list
    let gate_lookup:HashMap<String, [String; 3]> =  gates.lines()
    .map(|line| {
        let gate_line:Vec<_> = line.split_whitespace().collect();
        (
            gate_line[4].to_owned(),
            [gate_line[0].to_owned(), gate_line[2].to_owned(),  gate_line[1].to_owned()]
        )
    }).collect();
    
    // components will be the final wired up 
    // gates. 
    let mut components:HashMap<String, Gate> = wires
        .into_iter()
        .map(|(name, value)| (name, Gate::Wire(value)))
        .collect();
    
    for name in &wire_order {
        if !gate_lookup.contains_key(name) {
            continue;
        }
        let g = &gate_lookup[name];
        let a = &g[0];
        let b = &g[1];
        let op = &g[2];
        
        let gate = match op.as_str() {
            "AND" => Gate::And(Box::new(components[a].clone()), Box::new(components[b].clone())),
            "XOR" => Gate::Xor(Box::new(components[a].clone()), Box::new(components[b].clone())),
            "OR" => Gate::Or(Box::new(components[a].clone()), Box::new(components[b].clone())),
            bad_gate => panic!("Bad gate type: {}", bad_gate)
        };
        components.insert(name.to_string(), gate);       
    }

    return components
}

fn part_one(gates: &HashMap<String, Gate>) -> u64 {
    gates
        .iter()
        .filter(|(n, _v)| n.starts_with("z"))
        .fold(0_u64, |acc,(k, v)| {
            let n:u8 = k.chars().skip(1).collect::<String>().parse().unwrap();
            acc + ((v.value() as u64) << n )
        })
}

fn main() {
    let s = fs::read_to_string("data.txt").expect("Could not open the");
    let gates = parse_input(&s);

    println!("Part one: {}", part_one(&gates));
}
