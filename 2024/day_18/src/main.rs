use day_18::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(f: File) -> Result<Vec<Point>, Box<dyn Error>> {
    BufReader::new(f)
        .lines()
        .map(|line| {
            let line = line?;
            let (col, row) = line.split_once(",").ok_or("Who took my comma?")?;
            let col = col.parse()?;
            let row = row.parse()?;

            Ok(Point { row, col })
        })
        .collect::<Result<Vec<Point>, Box<dyn Error>>>()
}

fn binary_search(blocks: &[Point]) -> usize {
    // find point where Some becomes None

    let start = Point::new(0, 0);
    let end = Point::new(70, 70);

    let mut lower = 0;
    let mut higher = blocks.len();

    while lower < higher {
        let mid = lower + (higher - lower) / 2;

        let map = Map::new(71, 71, &blocks[0..mid]);

        if map.bfs(&start, &end).is_some() {
            lower = mid + 1;
        } else {
            higher = mid;
        }
    }
    // insertion point is before
    higher - 1
}

fn main() {
    let f = File::open("data.txt").expect("could not find file?");
    let points = parse(f).unwrap();
    let map = Map::new(71, 71, &points[0..1024]);

    let start = Point::new(0, 0);
    let end = Point::new(70, 70);

    if let Some(part_one) = map.bfs(&start, &end) {
        println!("Part one: {}", part_one);
    }

    let n = binary_search(&points);
    println!("part two index: {} block: {:?}", n, points[n]);
}
