use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{VecDeque, HashSet};
use crate::grid::Grid;

mod grid;

static DATA:&str = "data.txt";

type Map = Vec<Vec<u8>>;
type Point = (usize, usize);

fn traverse(
    map: &Map, 
    start: Point,
    end: Point,
    edge_predicate: fn(Point, Point, &Map) -> bool,
    end_predicate: fn(Point, Point, &Map) -> bool
) -> Option<u64> {
    /*
        The edge_predicate should return true if there is an edge between the two tuples.
        The end_predicate should return true when we've reached the goal.
    */

    let mut seen = HashSet::<Point>::new();  
    let mut q = VecDeque::<(usize, usize, u64)>::new();
    
    q.push_back((start.0, start.1, 0));

    let grid = Grid::new(map.len(), map[0].len());
    
    while let Some((row, col, steps)) = q.pop_front() {
        if end_predicate((row, col), end, map){
            return Some(steps)
        }
       
        for p in grid.neighbors((row, col)) {
            if edge_predicate((row, col), p, map)  && !seen.contains(&p) {
                q.push_back((p.0, p.1, steps+1));
                seen.insert(p);
            }
        }
    }
    None
    
}

fn main() {
    let p = Path::new(DATA);
    let f = File::open(p).expect("Could not open file");

    let start = 'S';
    let end = 'E';
    let mut start_stop:VecDeque<(usize, usize)> = VecDeque::new();

    let lines: Map = BufReader::new(f)
        .lines()
        .enumerate()
        .map(|(row, line)| line.unwrap().chars().enumerate().map(|(col, c)| {
            match c {
                n if n == start => {
                    start_stop.push_front((row, col));
                    'a' as u8
                },
                n if n == end => {
                    start_stop.push_back((row, col));
                    'z' as u8
                },
                _ => c as u8
            }
            
        }).collect())
        .collect();
        

    let start = start_stop.pop_front().unwrap();
    let end = start_stop.pop_back().unwrap();
    
    /* Part One */
    fn part_one_edges(curr: Point, neighbor: Point, m: &Map) -> bool {
        m[neighbor.0][neighbor.1] <= m[curr.0][curr.1] + 1
    }
    fn part_one_end(current: Point, end:Point, _: &Map) -> bool {
        current == end
    }

    let steps = traverse( &lines,start,end, part_one_edges,part_one_end); 
    println!("Part One: {:?}", steps);


    /* Part Two */
    fn part_two_edges(current: Point, neighbor: Point, m: &Map) -> bool {
        m[neighbor.0][neighbor.1] >= m[current.0][current.1] - 1
    }
    fn part_two_end(current: Point, _: Point, m: &Map) -> bool {
        m[current.0][current.1] == 'a' as u8
    }

    let steps = traverse(&lines, end, start, part_two_edges, part_two_end); 
    println!("Part Two: {:?}", steps);


}
