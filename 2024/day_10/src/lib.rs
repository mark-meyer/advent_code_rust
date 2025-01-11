pub enum Direction {
    /// A cardinal direction
    /// South and East take upper bound
    /// North and West are bound at 0
    North,
    South(usize),
    East(usize),
    West
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize
}

impl Point {
    pub fn step(self, direction: &Direction) -> Option<Point> {
        let Point{row, col} = self; 
        match direction {
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