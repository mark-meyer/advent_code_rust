use std::ops::{Add, Sub};


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

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            row: self.row - rhs.row,
            col: self.col - rhs.col
        }
    }
}
pub const DIRECTIONS: [Point; 4] = [
    Point::new(-1, 0),  // Up
    Point::new(0, 1),   // Right
    Point::new(1, 0),   // Down
    Point::new(0, -1),  // Left
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_point(){
        let p = Point {row: 10, col: 20};
        let p2 = Point {row: -1, col: 2};
        let sum = p + p2;
        assert_eq!(sum.row, 9);
        assert_eq!(sum.col, 22);
    }
    #[test]
    fn test_sub_point(){
        let p = Point {row: 10, col: 20};
        let p2 = Point {row: 2, col: 2};
        let sum = p - p2;
        assert_eq!(sum.row, 8);
        assert_eq!(sum.col, 18);
    }
}