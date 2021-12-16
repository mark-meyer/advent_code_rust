use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

type Matrix = Vec<Vec<usize>>;

pub fn neighbors(p:(usize, usize), max_w:isize, max_h:isize) -> Vec<(usize, usize)>{
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
    .iter()
    .map(|(move_x, move_y)| (p.0 as isize + move_x, p.1 as isize + move_y))
    .filter(|(move_x, move_y)|
        (0..max_w).contains(move_x)
        && (0..max_h).contains(move_y)
    )
    .map(|(x, y)| (x as usize, y as usize))
    .collect()
}


#[derive(Debug)]
pub struct Node {
    pub edges: Vec<Edge>
}

#[derive(Debug)]
pub struct Edge {
    pub cost: usize,
    pub dest: (usize, usize)
}

#[derive(Debug, PartialEq, Eq)]
pub struct CostNode {
    pub cost: usize,
    pub node: (usize, usize)
}

impl Ord for CostNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for CostNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<(usize, usize), Node>,
}

impl Graph {
    pub fn new(ref_matrix: &Matrix) -> Self {
        let mut nodes = HashMap::new();

        let width = ref_matrix[0].len() as isize;
        let height = ref_matrix.len() as isize;

        for (y, list) in ref_matrix.iter().enumerate(){
            for (x, _) in list.iter().enumerate(){
                let mut next_node = Node {edges: Vec::new()};
                for dest in neighbors((x, y), width, height) {
                    let cost = ref_matrix[dest.1][dest.0];
                    next_node.edges.push(Edge {cost: cost as usize, dest});
                }
                nodes.insert((x, y), next_node);
            }
        }
        Self { nodes }
    }

    pub fn dijkstra(&self, source: (usize, usize), target: (usize, usize)) -> Option<usize> {
        let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        distances.insert(source,  0);
        heap.push(CostNode{node: source, cost:0});

        while let Some(current_node) = heap.pop() {
            if current_node.node == target {
                return Some(current_node.cost)
            }
            if current_node.cost > *distances.entry(current_node.node.clone()).or_insert(usize::MAX) {
                // we've already found a cheaper way to get there
                continue
            }

            // this is a cheap way to current_node
            // add its children if we havne't already found
            // a cheaper route.
            for edge in &self.nodes.get(&current_node.node).unwrap().edges {
                let next = CostNode {
                    node: edge.dest,
                    cost: edge.cost + current_node.cost
                };
                if next.cost < *distances.entry(edge.dest).or_insert(usize::MAX) {
                    distances.insert(edge.dest, next.cost);
                    heap.push(CostNode{node: edge.dest, cost:next.cost})
                }
            }
        }
        None
    }
}

pub fn make_large_graph(matrix:&Matrix, expand_factor: usize) -> Matrix {
    let mut m:Matrix = Vec::new();
    for y in 0..expand_factor{
        for list in matrix {
            let  mut new_row = Vec::new();
            for x in 0..expand_factor{
                for n in list {
                    new_row.push(((n - 1) + (x + y) ) % 9 + 1)
                }
            }
        m.push(new_row);
        }
    }
    m
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_neighbors(){
        let w = 5;
        let h = 8;
        assert_eq!(neighbors((4,7), w,  h), vec![(3, 7), (4, 6)]);
        assert_eq!(neighbors((4,7), w,  h), vec![(3, 7), (4, 6)]);
        assert_eq!(neighbors((5,8), w,  h), vec![]);
        assert_eq!(neighbors((2,2), w,  h), vec![ (3, 2), (1, 2), (2,3), (2,1)]);



    }
}