use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::boxed::Box;
use std::collections::HashSet;
use crate::point::Point;

mod point;

static DATA: &str  = "data.txt";

struct RopeSegment {
    position: Point,
    follower: Option<Box<RopeSegment>>
}

impl RopeSegment {
    fn go(&mut self, direction: Point) -> Option<Point> {
        // Returns the position the last follower
        // is in if it moved, otherwise None
        self.position = &self.position + &direction;
        if let Some(f) = &mut self.follower {
            f.follow(&self.position)
        } else {
            Some(self.position.clone())
        }
    }

    fn follow(&mut self, leader_pos: &Point) -> Option<Point>{
        if !self.position.touching(leader_pos) {
            self.go(self.position.direction(leader_pos))
        } else {
            None
        }
    }
}

fn run_sim(head: &mut RopeSegment, f: File) -> usize {
    let mut seen:HashSet<Point> = HashSet::new();
    // assumes staring position of (0,0)
    seen.insert(Point::new());

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        let (dir, count) = match l.split_once(" ") {
            Some(("U", n)) => (Point {x: 0, y:-1}, n),
            Some(("D", n)) => (Point {x: 0, y:1}, n),
            Some(("R", n)) => (Point {x: 1, y:0}, n),
            Some(("L", n)) => (Point {x: -1, y:0}, n),
            _ => panic!("bad input")
        };
        let count = count.parse().expect("can't parse number!");
        // moving the head will return Some(Position) if the tail moved
        // or None if it didn't. 
        seen.extend((0..count).flat_map(|_| head.go(dir)));
    }
    seen.len()
}

fn main() {
    let p = Path::new(DATA);
    let f = File::open(p).expect("could not open the file!");
    let tail = RopeSegment {position: Point::new(), follower:None};
    let mut head = RopeSegment {position: Point::new(), follower: Some(Box::new(tail))};
    let p_one = run_sim(&mut head, f);

    println!("Part One: {}", p_one);

    let tail = RopeSegment {position: Point::new(), follower:None};
    let mut head = (0..9).fold(tail, |acc, _| RopeSegment {position: Point::new(), follower:Some(Box::new(acc))});
    let f = File::open(p).expect("could not open the file!");
    let p_two = run_sim(&mut head, f);
    
    println!("Part Two: {}", p_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go() {
        let mut head = RopeSegment {position: Point::new(), follower:None};
        head.go(Point {x: 1, y:0});
        assert_eq!(head.position, Point{x: 1, y: 0});

        let mut head = RopeSegment {position: Point::new(), follower:None};
        head.go(Point {x: 1, y:-1});
        assert_eq!(head.position, Point{x: 1, y: -1});
    }
}
