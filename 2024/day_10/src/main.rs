use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::collections::HashSet;
use day_10::*;

type Matrix = Vec<Vec<u32>>;

enum Part{
    One,
    Two
}

fn parse(f:File) -> Result<Matrix, Box<dyn Error>> {
    BufReader::new(f)
    .lines()
    .map(|line| {
        let line = line?;
        line.chars()
            .map(|c| c.to_digit(10).ok_or_else(|| "Bad trail data".into()))
        .collect::<Result<Vec<u32>, _>>()
    })
    .collect::<Result<Matrix, _>>()
}

fn find(n:u32, map:&Matrix) -> Vec<Point> {
    map.iter()
    .enumerate()
    .flat_map(|(r, row)| row.iter()
        .enumerate()
        .filter_map(|(c, &m)| if m == n {
            Some(Point::new(r as i32, c as i32))
        } else {
            None
        })
        .collect::<Vec<Point>>()
    )
    .collect()
}

fn count_paths(start:&Point, map:&Matrix, part:Part) -> u32{
    let bounds =  Point::new((map.len() - 1) as i32, (map[0].len() - 1) as i32);

    let mut stack = vec![*start];
    let mut seen = HashSet::new();
    let mut score:u32 = 0;

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        let height = map[current.row as usize][current.col as usize];
        if height == 9 {
            score += 1;
            continue
        }

        for dir in DIRECTIONS {
            let next_p = current + dir;
            if !next_p.within(&bounds) {
                continue
            }

            let next_height = map[next_p.row as usize][next_p.col as usize];
            if next_height == height + 1 {
                match part {
                    Part::One if seen.contains(&next_p) => {},
                    _ => {
                        seen.insert(next_p);
                        stack.push(next_p)
                    }
                }
            }
        }
    }
    score
}

fn main() {
    let p = Path::new("data.txt");
    let f = File::open(p).expect("wut? no file?");
    let map = parse(f).unwrap();
    let zeros = find(0, &map);

    let part_one:u32= zeros
        .iter()
        .map(|z| count_paths(&z, &map, Part::One))
        .sum();
    println!("Part One: {}", part_one);

    let part_two:u32= zeros
        .iter()
        .map(|z| count_paths(&z, &map, Part::Two))
        .sum();

    println!("Part Two: {}", part_two);

}
