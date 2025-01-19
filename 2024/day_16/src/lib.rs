use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3
}
const DIRECTIONS:[Direction;4] = [Direction::North, Direction::East, Direction::South, Direction::West];

/* 
    HeapNode
    A struct that implements Ord 
    to be used in a min heap.
    It also does some housekeeping to track the path
    used up to this point for part two.
 */
#[derive(PartialEq, Eq)]
struct HeapNode {cost:u64, point:Point, path:Vec<(usize, usize)>}

impl Ord for HeapNode {
    fn cmp(&self, other:&HeapNode) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

/*
    MazeSpot
    Represents each position in the
    maze along with the lowest cost 
    so for for each direction on that
    point. 
*/
#[derive(Clone, Copy, Debug)]
pub enum MazeSpot {
    Wall,
    Space, // min cost seen so far
}

pub struct Maze {
    pub matrix: Vec<Vec<MazeSpot>>,
    pub start: Point,
    pub end: (usize, usize)
}

impl Maze {
    fn get(&self, p:&Point) -> &MazeSpot {
        &self.matrix[p.row][p.col]
    }

    pub fn least_cost(&mut self) -> Option<(u64, usize)> {
        // Keeps track of the min seen in the maze
        // which means this mutates the maze
        let start_node = HeapNode{cost:0, point:self.start, path:vec![self.start.coords()]};
        let mut heap:BinaryHeap<HeapNode> = BinaryHeap::from([start_node]);
        let mut costs = vec![vec![[u64::MAX; 4]; self.matrix[0].len()]; self.matrix.len()];

        let mut global_min = u64::MAX;
        let mut track_positions = HashSet::new();

        while !heap.is_empty() {
            if let Some(HeapNode{cost, point, path}) = heap.pop() {
                let (row, col) = point.coords();

                if (row, col) == self.end {
                    if cost <= global_min {
                        track_positions.extend(path);
                        global_min = cost;
                    } else {
                        return Some((global_min, track_positions.len()))
                    }
                    continue
                }
                
                let current_direction = point.dir;    
                for direction in DIRECTIONS {
                    let neighbor_point = point.step(direction);
                    let  neighbor_spot = self.get(&neighbor_point);

                    match neighbor_spot {
                        MazeSpot::Wall => {},
                        
                        MazeSpot::Space  => {
                            let previous_cost = costs[neighbor_point.row][neighbor_point.col][direction as usize];
                            let mut next_cost = cost;
                            let next_point;
                            let mut new_path = path.clone();

                            if direction != current_direction {
                                next_cost += 1000;
                                next_point = point;
                                
                            } else {
                                next_cost += 1;
                                next_point = neighbor_point;
                                new_path.push(next_point.coords())
                            }
                            if next_cost <= previous_cost {
                                heap.push(HeapNode{
                                    cost:next_cost, 
                                    point:Point{ dir:direction, ..next_point},
                                    path: new_path
                                });
                                let (row, col) = next_point.coords();
                                costs[row][col][direction as usize] = next_cost
                            }
                        }
                    }
                }
            }
        }
        None
    }
    pub fn back_track(&self) {
        let (row, col) = self.end;
        let spot = self.matrix[row+1][col];
        println!{"{:?}, {}-{}", spot, row, col};
    }
}
impl From<String> for Maze {
    fn from(s:String) -> Self {
        let mut start = None;
        let mut end = None;
        let matrix = s.split("\n")
        .enumerate()
        .map(|(row, line)| line
            .chars()
            .enumerate()
            .map(|(col, c)| {
                match c {
                    '#' => MazeSpot::Wall,
                    '.' => MazeSpot::Space,
                    'E' => {
                        end = Some((row, col));
                        MazeSpot::Space
                    }
                    'S' => {
                        // race starts facing east
                        start = Some(Point{row, col, dir:Direction::East});
                        MazeSpot::Space
                    },
                    _ => {
                        panic!("Debris on the race course!");
                    }
                }
            }).collect()
        ).collect();
        Maze{matrix, start: start.expect("No start?!"), end: end.expect("No End?")}
    }  
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize,
    pub dir: Direction
}

impl Point {
    pub fn step(&self,dir:Direction) -> Point {
        let &Point{row, col, dir:_} = self; 
        match dir {
            Direction::North => Point{row: row - 1, col, dir},
            Direction::West =>  Point{row, col:col - 1, dir},
            Direction::South => Point{row: row + 1, col, dir},
            Direction::East => Point{row, col:col + 1, dir},
        }
    }
    pub fn coords(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

