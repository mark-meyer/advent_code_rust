use std::ops::Add;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub row: i32,
    pub col: i32
}

impl Point {
    pub const fn new(row: i32, col: i32) -> Self {
        Point {row, col}
    }
    pub fn within(&self, bounds:&Point) -> bool {
        self.row >= 0 && self.row <= bounds.row
        && self.col >= 0 && self.col <= bounds.col
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            row: self.row + rhs.row,
            col: self.col + rhs.col
        }
    }
}

pub const DIRECTIONS: [Point; 4] = [
    Point::new(-1, 0),  // Up
    Point::new(0, 1),   // Right
    Point::new(1, 0),   // Down
    Point::new(0, -1),  // Left
];
