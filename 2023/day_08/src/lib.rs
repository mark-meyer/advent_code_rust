use regex::Regex;
use std::collections::HashMap;

pub type Graph = HashMap<String, Node>;

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(c: char) -> Instruction {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Bad Instruction"),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub left: String,
    pub right: String,
}

pub fn get_path_length<'a, I>(
    filter_function: fn(&'a str) -> bool,
    g: &'a Graph,
    mut start: &'a str,
    mut instructions: I,
) -> u64
where
    I: Iterator<Item = &'a Instruction>,
{
    let mut steps = 0;
    while filter_function(start) {
        let node = g
            .get(start)
            .expect(&format!("Could not find node {}!", start));
        let i = instructions.next().unwrap();
        start = match i {
            Instruction::Left => &node.left,
            Instruction::Right => &node.right,
        };
        steps += 1;
    }
    steps
}

pub fn create_graph<I>(lines: I) -> Result<Graph, std::io::Error>
where
    I: Iterator<Item = Result<String, std::io::Error>>,
{
    let rx = Regex::new(r"\w+").unwrap();

    let mut label_lookup: HashMap<String, Node> = HashMap::new();

    for line in lines {
        let line = line?;
        let mut matches = rx.find_iter(&line);

        let label = matches.next().unwrap().as_str().to_owned();
        let left = matches.next().unwrap().as_str().to_owned();
        let right = matches.next().unwrap().as_str().to_owned();

        let node = Node {
            left: left,
            right: right,
        };

        label_lookup.insert(label, node);
    }

    Ok(label_lookup)
}
