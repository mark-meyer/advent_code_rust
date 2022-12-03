use std::collections::{HashMap, HashSet, BTreeSet};

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

// fn is_upper(s: &str) -> bool {
//     !s.chars().any(|c| c.is_lowercase())
// }

fn is_lower(s: &str) -> bool {
    !s.chars().any(|c| c.is_uppercase())
}

pub fn parse_input(s: &str) -> Graph {
    let mut g = HashMap::new();
    for line in s.lines() {
        let mut pair = line.split('-');
        let source = pair.next().unwrap();
        let dest = pair.next().unwrap();
        if dest != "start" && source != "end" {
            g.entry(source).or_insert(HashSet::new()).insert(dest);
        }

        if dest != "end" && source != "start" {
            g.entry(dest).or_insert(HashSet::new()).insert(source);
        }
    }
    g
}

pub fn count_paths_one(graph: &Graph, start: &str, seen: Option<HashSet<&str>>) -> usize {
    let seen = seen.unwrap_or_default();
    if start == "end" {
        return 1
    }
    let mut total = 0;
    for &val in graph.get(start).unwrap() {
        if seen.contains(val){
            continue
        }
        let mut local_seen:HashSet<&str> = HashSet::new();
        if is_lower(val) {
            local_seen.insert(val);
        }
        local_seen.extend(&seen);
        total += count_paths_one(graph, val, Some(local_seen));
    }
    total
} 

pub fn count_paths_two(
    graph: &Graph, 
    start: &str, 
    seen: Option<BTreeSet<&str>>, 
    twice:bool,
) -> usize {
  
    let seen = seen.unwrap_or_default();

    if start == "end" {
        return 1
    }
    let mut total = 0;
    for &val in graph.get(start).unwrap() {
        let mut local_twice = twice;
        let mut local_seen:BTreeSet<&str> = BTreeSet::new();
        if is_lower(val) {
            local_seen.insert(val);
            if seen.contains(val) {
                if local_twice {
                    continue
                }
                local_twice = true;
            }
        }
        local_seen.extend(&seen);
        let subsum = count_paths_two(graph, val, Some(local_seen), local_twice);
        total += subsum;
    }
    
    total
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_parse(){
        let inp = "start-A\n\
        start-b\n\
        A-c\n\
        A-b\n\
        b-d\n\
        A-end\n\
        b-end";

        let g = parse_input(inp);
        assert_eq!(g.get("start"), Some(&HashSet::from(["A", "b"])));
        assert_eq!(g.get("A"), Some(&HashSet::from(["c", "b", "end"])));
        assert_eq!(g.get("d"), Some(&HashSet::from(["b"])));
    }
    #[test]
    fn test_path_count_one(){
        let inp = "start-A\n\
        start-b\n\
        A-c\n\
        A-b\n\
        b-d\n\
        A-end\n\
        b-end";

        let g = parse_input(inp);
        assert_eq!(count_paths_one(&g, "start", None), 10);
    }

    #[test]
    fn test_path_count_two(){
        let inp = "dc-end\n\
        HN-start\n\
        start-kj\n\
        dc-start\n\
        dc-HN\n\
        LN-dc\n\
        HN-end\n\
        kj-sa\n\
        kj-HN\n\
        kj-dc";

        let g = parse_input(inp);
        assert_eq!(count_paths_two(&g, "start", None, false), 103);
    }
}