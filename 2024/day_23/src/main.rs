use std::fs;
use std::collections::{HashMap, BTreeSet};

type Graph = HashMap<String, BTreeSet<String>>;

fn parse_input(s:String) -> Graph {
    let mut graph:Graph = HashMap::new();
    s.lines()
    .for_each(|line| {
        let (a, b) = &line.split_once("-").unwrap();
        graph.entry(b.to_string()).or_default().insert(a.to_string());
        graph.entry(a.to_string()).or_default().insert(b.to_string());
    });
    graph
}

/* Part One */
fn find_parties(g:&Graph, node: &str) -> Option<BTreeSet<BTreeSet<String>>> {
    if !node.starts_with("t") {
        return None
    }
    let connected = &g[node];

    connected.iter()
    .zip(connected.iter().skip(1))
    .filter(|(a, b)| g[*a].contains(*b))
    .map(|(a, b)|  Some(BTreeSet::from([a.to_owned(), b.to_owned(), node.to_owned()])))
    .collect()
}

fn all_parties(g:&Graph) -> BTreeSet<BTreeSet<String>> {
    g.keys()
    .flat_map(|n| find_parties(g, n).into_iter().flatten())
    .collect()
}

/* 
Part Two - this produces te correct answer, 
but not sure if it's really correct
*/
fn find_local_complete(g:&Graph, node:&str) -> BTreeSet<String> {
    let connected = &g[node];

    let mut complete = BTreeSet::from([node.to_owned()]);

    for a in connected.iter(){
        if complete.is_subset(&g[a.as_str()]) {
            complete.insert(a.to_string());
        }
    }
    return complete
}

fn all_complete(g:&Graph) -> Option<BTreeSet<String>> {
    g.keys()
    .map(|n| find_local_complete(g, n))
    .max_by_key(|s| s.len())
}

fn main() {
    let f = fs::read_to_string("data.txt").expect("Can't open the file!");
    let graph = parse_input(f);
    println!("{:?}", all_parties(&graph).len());

    let clique = all_complete(&graph).unwrap();
    let joined = clique.iter().cloned().collect::<Vec<String>>().join(",");
    println!("{:?}", joined);
}
