use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

static PATH:&str = "data.txt";


struct Directory {
    parent_index: Option<usize>,
    size: usize,
    // keys will be the path directory name the values will be indices
    // into the vec storing the nodes
    children: HashMap<String, usize>
}

impl Directory {
    fn new() -> Directory {
        Directory {
            parent_index: None,
            size: 0,
            children: HashMap::new() }
    }
}

fn is_numeric(s: &str) -> bool{
    s.chars().all(|c| c.is_numeric())
}

fn cd(path: &str, current_index: usize, nodes: &mut Vec<Directory>) -> usize {
    // Either return the existing index from the current node's children
    // Or create a new Directory, add it to the current node's children
    // and push into the vector, returning the new index.
    match nodes[current_index].children.get(path) {
        Some(child_index) => *child_index,
        None => {
            let mut d = Directory::new();
            d.parent_index = Some(current_index);
            let new_index = nodes.len();
            nodes[current_index].children.insert(path.to_string(),new_index);
            nodes.push(d);
            new_index
        }
    }
}

fn find_directory_totals(nodes: &Vec<Directory>, max_size:usize) -> usize {
    nodes
    .iter()
    .filter(|n| n.size <= max_size )
    .map(|n| n.size)
    .sum()
}

fn find_directory_to_delete(
    nodes: &Vec<Directory>,
    total_space: usize,
    needed_space: usize
) -> usize {
    let available_space = total_space - nodes[0].size;

    nodes
    .iter()
    .filter(|n| n.size >= needed_space - available_space)
    .map(|n| n.size)
    .min()
    .unwrap()
}

fn main() {
    // The basic idea is to keep a vector of directories
    // The directories keep track of their children and parent by storing the
    // indices into the vector. This avoids problems associated with storing
    // references to the children and parents.

    let p = Path::new(PATH);
    let f = File::open(p).expect("Can not open file!");

    let mut nodes = vec![Directory::new()];
    let mut current_index = 0;

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();

        match l.rsplit_once(' ') {
            Some(("$ cd", path)) => {
                current_index = match path {
                    "/"  => 0,
                    ".." => nodes[current_index].parent_index.unwrap(),
                    _ =>  cd(path, current_index, &mut nodes)
                }
            },
            Some((size, _)) if is_numeric(size) => {
                let s:usize = size.parse().unwrap();
                nodes[current_index].size += s;
                let mut i = current_index;
                // Add the size to the parents' sizes as well
                // which will save time later.
                while let Some(n) = nodes[i].parent_index {
                    nodes[n].size += s;
                    i = n;
                }
            },
            // ignore all the other commands...don't really need them
            _ => ()
        }
    }

    println!("Part One: {:?}", find_directory_totals(&nodes, 100_000));
    println!("Part Two: {:?}", find_directory_to_delete(&nodes,  70_000_000, 30_000_000));
}
