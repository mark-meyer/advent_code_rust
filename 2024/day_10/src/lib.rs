#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize
}


pub enum Direction {
    North,
    South(usize),
    East(usize),
    West
}

impl Direction {
    pub fn step(&self, Point{row, col}: Point) -> Option<Point> {
        match self {
            Direction::North if row > 0  => 
                Some(Point{row: row - 1, col}),
            Direction::West if col > 0  => 
                Some(Point{row, col:col - 1}),
            Direction::East(bound) if col < *bound  => 
                Some(Point{row, col:col + 1}),
            Direction::South(bound) if row < *bound => 
                Some(Point{row: row + 1,col}),
            _ => None
        }
    }
}
