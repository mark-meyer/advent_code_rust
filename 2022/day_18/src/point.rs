use std::hash::Hash;
use std::ops::{Add, Sub};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Point {
    pub fn new(x:i32, y:i32, z:i32) -> Point {
        Point {x, y, z}
    }
    pub fn neighbors(&self) -> HashSet<Point>{
        let offsets = [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
            [-1, 0, 0],
            [0, -1, 0],
            [0, 0, -1]
        ];
        offsets.iter().map(|p| self + &Point { x: p[0], y:p[1], z:p[2]} ).collect()
    }

    pub fn inside(&self, min_extent:&Self, max_extent:&Self) -> bool {
        min_extent.x <= self.x && self.x <= max_extent.x &&
        min_extent.y <= self.y && self.y <= max_extent.y &&
        min_extent.z <= self.z && self.z <= max_extent.z
    }

    pub fn min(&self, other:&Self) -> Point {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z)
        }
    }
    pub fn max(&self, other:&Self) -> Point {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z)
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other:Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
 }

 impl Sub for &Point {
    type Output = Point;

    fn sub(self, other:Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
 }