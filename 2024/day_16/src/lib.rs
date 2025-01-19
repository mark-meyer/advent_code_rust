use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3
}
impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East
        }
    }
}
pub const DIRECTIONS:[Direction;4] = [Direction::North, Direction::East, Direction::South, Direction::West];

/* 
    HeapNode
    A struct that implements Ord to be used in a min heap.
 */
#[derive(PartialEq, Eq)]
struct HeapNode {cost:u64, point:Point}

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
    Represents each position in the maze along with the lowest cost 
    so for for each direction on that point. 
*/
#[derive(Clone, Copy, Debug)]
pub enum MazeSpot {
    Wall,
    Space,
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

    pub fn least_cost(&self) -> Option<(u64, Vec<Vec<[u64;4]>>)> {        
        let start_node = HeapNode{cost:0, point:self.start};
        let mut heap:BinaryHeap<HeapNode> = BinaryHeap::from([start_node]);

        let mut costs = vec![vec![[u64::MAX; 4]; self.matrix[0].len()]; self.matrix.len()];


        while !heap.is_empty() {
            if let Some(HeapNode{cost, point}) = heap.pop() {
                let (row, col) = point.coords();
                let current_direction = point.dir; 
                
                if cost <=  costs[row][col][current_direction as usize] {
                    costs[row][col][current_direction as usize] = cost;
                } 

                if (row, col) == self.end { return Some((cost, costs)) }
                
                for direction in DIRECTIONS {
                    let neighbor_point = point.step(direction);

                    if let  MazeSpot::Space = self.get(&neighbor_point) {   
                        let previous_cost = costs[neighbor_point.row][neighbor_point.col][direction as usize];
                        let next_cost;
                        let next_point;

                        if direction != current_direction {
                            next_cost = cost + 1000;
                            next_point = point;
                        } else {
                            next_cost = cost +  1;
                            next_point = neighbor_point;
                        }
                        if next_cost <= previous_cost {
                            heap.push(HeapNode{
                                cost:next_cost, 
                                point:Point{dir:direction, ..next_point},
                            });
                        }
                    }
                }
            }
        }
        None
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

