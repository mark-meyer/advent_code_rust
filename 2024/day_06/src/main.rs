use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rayon::prelude::*;
use ahash::{AHashSet, AHashMap};

use day_06::*;

fn parse_input(f:File) -> (Point, Point, AHashSet<Point>){
    let mut blocks:AHashSet<Point> = AHashSet::default();
    let mut start:Option<Point> = None;
    let mut h = 0;
    let mut w = 0;
    for (row, line_result) in BufReader::new(f).lines().enumerate() {
        let line = line_result.unwrap();
        for (col, chr) in line.chars().enumerate() {
            if chr == '#' {
                let p = Point{row: row as i32, col: col as i32};
                blocks.insert(p);
            } else if chr == '^' {
                start = Some(Point{row: row as i32, col: col as i32});
            }
            w = col;
        }
        h = row;
    }
    let bounds = Point{
        row: h as i32,
        col: w as i32
    };
    let start = start.expect("Eh? No starting point found!?!");
    (bounds, start, blocks) 
}

fn part_one(mut current: Point, bounds: &Point, blocks: &AHashSet<Point>) -> (usize, AHashMap<Point, usize>) {
    let mut d_index = 0;
    let mut seen:AHashSet<Point> = AHashSet::default();
    let mut seen_dir:AHashMap<Point, usize> = AHashMap::default();

    while current.within(bounds) {
        seen.insert(current);
        if !seen_dir.contains_key(&current) {
            seen_dir.insert(current, d_index);
        }
        let dir = DIRECTIONS[d_index];
        let next = current + dir;

        if blocks.contains(&next) {
            d_index = (d_index + 1) % DIRECTIONS.len();
        } else {
            current = next;
        }
    }
    (seen.len(), seen_dir)
}

fn is_loop(mut current: Point, bounds: &Point, mut d_index: usize, blocks: &AHashSet<Point>, potential_block:&Point) -> bool{
    let mut seen:AHashSet<(Point, usize)> = AHashSet::default();
    
    while current.within(bounds) {
        if seen.contains(&(current, d_index)) {
            return true
        }
        let dir = DIRECTIONS[d_index];
        let next = current + dir;
        if blocks.contains(&next) || next == *potential_block {
            seen.insert((current, d_index));
            d_index = (d_index + 1) % DIRECTIONS.len();
        } else {
            current = next;
        }
    }
    false
}

fn part_two(path: &AHashMap<Point, usize>,  bounds: &Point, blocks: &AHashSet<Point>) -> usize {
    path.par_iter()
    .map(|(potential_block, d_index)| {
        let dir = DIRECTIONS[*d_index];
        let start = *potential_block - dir;
        if is_loop(start, bounds, (d_index + 1) % DIRECTIONS.len(), blocks, potential_block) {
            1
        } else {
            0
        }
    })
    .sum()
}

fn main() {
    let path = Path::new("data.txt");
    let f = File::open(path).expect("Could not open the file!");
    let (bounds, start, blocks) = parse_input(f);
    let (total, visited) = part_one(start, &bounds, &blocks);
    println!("Part one: {:?}", total);

    let loops = part_two(&visited, &bounds, &blocks);
    println!("Part two {:?}", loops);
}