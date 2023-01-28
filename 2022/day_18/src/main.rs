#![allow(unused)]
use std::fs;
use std::collections::{HashSet, VecDeque};
use point::Point;

mod point;

fn get_extents(cubes: &HashSet<Point>) -> (Point, Point) {
    let min = cubes
        .iter()
        .fold(Point{x:0, y:0, z:0}, |min_p, p| min_p.min(p) );

    let max = cubes
        .iter()
        .fold(Point{x:0, y:0, z:0}, |min_p, p| min_p.max(p) );

    let offset = Point{x:1, y:1, z:1};
    (&min - &offset, &max + &offset)
}


fn bfs(min: &Point, max: &Point, cubes: &HashSet<Point>) -> usize {
    let mut q = VecDeque::from([min.clone()]);
    let mut seen:HashSet<Point> = HashSet::new();
    let mut face_count = 0;

    while let Some(current) = q.pop_front() {
        for n in current.neighbors() {
            if n.inside(&min, &max) & !seen.contains(&n) {
                if cubes.contains(&n) {
                    face_count += 1
                } else {
                    q.push_back(n);
                    seen.insert(n);
                }
            }
        }
    }

    face_count
}

fn main() {
    let data = fs::read_to_string("data.txt").expect("no scan data");
    let cubes:HashSet<_> = data.lines().map(|l| {
        let coords:Vec<_> = l.split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        Point::new(coords[0], coords[1], coords[2])
    }).collect();

    let total:usize = cubes.iter().map(|c| c.neighbors().difference(&cubes).count()).sum();
    println!("Part one: {}", total);

    let (min, max)  = get_extents(&cubes);
    let spaces = bfs(&min, &max, &cubes);
    println!("Part two: {}", spaces);


}
