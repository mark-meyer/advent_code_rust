use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::collections::HashSet;
use day_10::*;

type Matrix = Vec<Vec<usize>>;

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
            .map(|c| c
                .to_digit(10)
                .map(|n| n as usize)
                .ok_or_else(|| "Bad trail data".into()))
        .collect::<Result<Vec<usize>, _>>()
    })
    .collect::<Result<Matrix, _>>()
}

fn find(n:usize, map:&Matrix) -> Vec<Point> {
    map.iter()
    .enumerate()
    .flat_map(|(r, row)| row.iter()
        .enumerate()
        .filter_map(|(c, &m)| if m == n {
            Some(Point{row: r, col:c})
        } else {
            None
        })
        .collect::<Vec<Point>>()
    )
    .collect()
}

fn count_paths(start:&Point, map:&Matrix, part:Part) -> u32{
    let h = map.len() - 1;
    let w = map[0].len() - 1;

    let directions = [
        Direction::North,
        Direction::South(h),
        Direction::East(w),
        Direction::West
    ];

    let mut stack = vec![*start];
    let mut seen = HashSet::new();
    let mut score:u32 = 0;

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        let height = map[current.row][current.col];
        if height == 9 {
            score += 1;
            continue
        }

        for dir in &directions {
            if let Some(next_p) = current.step(dir) {
                let next_height = map[next_p.row][next_p.col];
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
