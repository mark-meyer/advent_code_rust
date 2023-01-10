use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use point::Point;

mod point;

static DATA:&str = "data.txt";

fn parse_file(p: &Path) -> HashSet<Point> {
    let f = File::open(p).expect("Scanner is not working!");
    BufReader::new(f)
        .lines()
        .flat_map(|line| Point::parse_line(&line.unwrap()))
        .collect()
}

fn drop_sand(
    mut pos: Point,
    part1: bool,
    rocks: &HashSet<Point>,
    last_positions: &mut Vec<Point>,
    max_y: usize
) -> Option<Point> {
    loop {
        if pos.y +1 > max_y {
            if part1 {
                break None
            } else {
                break Some(pos)
            }
        }
        match pos.neighbors().find(|p| !rocks.contains(p)) {
            Some(p) => {
                last_positions.push(pos);
                pos = p;
            }
            None => break Some(pos)
        }
    }
}

fn main() {
    let p = Path::new(DATA);
    
    /* 
        Part One 
    */

    let mut rocks:HashSet<Point> = parse_file(p);
    let max_y = rocks.iter().map(|r| r.y).max().unwrap();
    let mut count: u32 = 0;
    let mut last_positions:Vec<Point> = Vec::new();
    let mut start_pos = Point::new(500, 0);

    while let Some(p) = drop_sand(start_pos, true, &rocks, &mut last_positions, max_y) {
        rocks.insert(p);
        start_pos = last_positions.pop().unwrap();
        count += 1; 
    }
    println!("Part One: {}", count);

    /* 
        Part Two 
    */
    let mut rocks:HashSet<Point> = parse_file(p);
    let mut last_positions = vec![Point::new(500, 0)];
    let mut count: u32 = 0;
    let max_y = max_y + 1;

    while let Some(start_pos) = last_positions.pop() {
        let s = drop_sand(start_pos, false, &rocks, &mut last_positions, max_y).unwrap();
        rocks.insert(s);
        count += 1; // move floor down one
    }
    println!("Part Two: {}", count);

}
