use num::clamp;
use std::ops;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new() -> Point {
        Point {x: 0, y:0}
    }

    pub fn touching(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    pub fn direction(&self, other: &Point) -> Point {
        Point {
            x: clamp(other.x - self.x, -1, 1),
            y: clamp(other.y - self.y, -1, 1)
        }
    }
}
impl ops::Sub for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

impl ops::Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_touching_true() {
        let p1 = Point::new();
        let p2 = Point{x: 1, y:0};
        assert!(p1.touching(&p2));

        let p1 = Point::new();
        let p2 = Point{x: 1, y:1};
        assert!(p1.touching(&p2));

        let p1 = Point::new();
        let p2 = Point{x: 0, y:-1};
        assert!(p1.touching(&p2));
    }
    #[test]
    fn test_touching_false() {
        let p1 = Point::new();
        let p2 = Point{x: 2, y:0};
        assert_eq!(p1.touching(&p2), false);

        let p1 = Point::new();
        let p2 = Point{x: 1, y:2};
        assert_eq!(p1.touching(&p2), false);

        let p1 = Point::new();
        let p2 = Point{x: -2, y:1};
        assert_eq!(p1.touching(&p2), false);
    }

    #[test]
    fn test_sub(){
        let p1 = Point{x: 2, y:1};
        let p2 = Point{x: 1, y:1};
        assert_eq!(&p1 - &p2, Point{x:1, y:0});

        let p1 = Point{x: 2, y:1};
        assert_eq!(&p1 - &p1, Point{x:0, y:0});

        let p1 = Point{x: 0, y:1};
        let p2 = Point{x: 1, y:-3};
        assert_eq!(&p1 - &p2, Point{x:-1, y:4});

    }


}
