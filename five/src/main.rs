extern crate regex;

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;
use LineType::*;

static PATH: &'static str = "./data.txt";

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y:i32) -> Point{
        Point {x, y}
    }
}

#[derive(Debug, PartialEq)]
enum LineType {
    Horizontal,
    Vertical,
    Diagonal(i32),
    Other
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
    direction: LineType
}

impl Line {
    fn new(mut p:Vec<Point>)  -> Line{
        p.sort_by_key(|p| (p.x, p.y));
        let p1 = p[0];
        let p2 = p[1];
  
        let direction = match (p1.x - p2.x, p2.y - p1.y) {
            (_, 0) => Horizontal,
            (0, _) => Vertical,
            (m, n) if m.abs() == n.abs()  => Diagonal(n.signum()),
            _ => Other
        };
        Line {p1, p2, direction}
    }

    fn points(&self) -> Vec<Point>{
        // No negative step_by on Rust ranges ... wtf?
        match self.direction {
            Horizontal  => (self.p1.x..self.p2.x+1).map(|x| Point::new(x, self.p1.y)).collect(),
            Vertical    => (self.p1.y..self.p2.y+1).map(|y| Point::new(self.p1.x, y)).collect(),
            Diagonal(m) => (self.p1.x..self.p2.x+1)
                            .enumerate()
                            .map(|(i, x)| Point::new(x, self.p1.y + (i as i32 * m)))
                            .collect(),                                  
            _ => Vec::new()
        }
    }
}

fn get_data(data:&str) -> Vec<Point>{
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    re.captures_iter(data)
        .map(|n| Point::new(
            n[1].parse::<i32>().unwrap(), 
            n[2].parse::<i32>().unwrap() )
        )
        .collect::<Vec<Point>>()
}



fn solution_one(data: &Vec<Line>) -> i32 {
    let mut point_counts = HashMap::new();
    for line in data {
        if matches!(line.direction, Horizontal) || matches!(line.direction, Vertical) {
            for point in line.points() {
                *point_counts.entry((point.x, point.y)).or_insert(0) += 1;
            };
        }
    };
    point_counts.values()
        .filter(|&n| *n > 1)
        .fold(0, |a, _| a + 1)
}

fn solution_two(data: &Vec<Line>) -> i32 {
    let mut point_counts = HashMap::new();
    for line in data {
        for point in line.points() {
            *point_counts.entry((point.x, point.y)).or_insert(0) += 1;
        };
    };
    point_counts.values()
        .filter(|&n| *n > 1)
        .fold(0, |a, _| a + 1)
}


fn main() {
    let path = Path::new(PATH);
    let data = fs::read_to_string(path).expect("Can't read file. We've fallen into a hyperthermic vent!");
    let points = get_data(&data);
    let lines:Vec<Line> = points.chunks(2).map(|ps| Line::new(ps.to_vec())).collect();

    let solution1 = solution_one(&lines);
    println!("solution 1: {}", solution1);

    let solution2 = solution_two(&lines);
    println!("solution 2: {}", solution2);

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_points(){
        let horiz_points = vec![Point {x:0,y:0}, Point {x:0,y:1}, Point {x:0,y:2}];
        let line:Line = Line::new(vec![Point::new(0, 0), Point::new(0, 2)]);
        assert_eq!(line.points(), horiz_points);

        let line:Line = Line::new(vec![Point::new(0, 2), Point::new(0, 0)]);
        assert_eq!(line.points(), horiz_points);

        let vert_points = vec![Point {x:3,y:2}, Point {x:4,y:2}, Point {x:5,y:2}];
        let line:Line = Line::new(vec![Point::new(3, 2), Point::new(5, 2)]);
        assert_eq!(line.points(), vert_points);

        let line:Line = Line::new(vec![Point::new(5, 2), Point::new(3, 2)]);
        assert_eq!(line.points(), vert_points);
    }
    #[test]
    fn test_get_diag_points(){
        let diag_points = vec![Point {x:0,y:0}, Point {x:1,y:1}, Point {x:2,y:2}];
        let line:Line = Line::new(vec![Point::new(2, 2), Point::new(0, 0)]);
        println!("{:?}", line);
        assert_eq!(line.points(), diag_points);

        let line:Line = Line::new(vec![Point::new(0, 0), Point::new(2, 2)]);
        assert_eq!(line.points(), diag_points);

        let diag_points = vec![Point {x:0,y:2}, Point {x:1,y:1}, Point {x:2,y:0}];
        let line:Line = Line::new(vec![Point::new(0, 2), Point::new(2, 0)]);
        println!("{:?}", line);

        assert_eq!(line.points(), diag_points);
    }
    #[test]
    fn parse_input_string_problem_1(){
        let s = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

        let data = get_data(s);
        assert_eq!(data.len(), 20);
        assert_eq!(data[1], Point::new(5, 9));
        assert_eq!(data[19], Point::new(8, 2));
        let lines:Vec<Line> = data.chunks(2).map(|ps| Line::new(ps.to_vec())).collect();
        let solution1 = solution_one(&lines);
        assert_eq!(solution1, 5)
    }
    #[test]
    fn parse_input_string_problem_2(){
        let s = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

        let data = get_data(s);
        assert_eq!(data.len(), 20);
        assert_eq!(data[1], Point::new(5, 9));
        assert_eq!(data[19], Point::new(8, 2));

        let lines:Vec<Line> = data.chunks(2).map(|ps| Line::new(ps.to_vec())).collect();
        let solution2 = solution_two(&lines);
        assert_eq!(solution2, 12)
    }
}