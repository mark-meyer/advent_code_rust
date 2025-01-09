use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use day_08::*;

fn parse_input(f:File) -> (Point, HashMap<char, Vec<Point>>){
    let mut antennas:HashMap<char, Vec<Point>> = HashMap::new();
    let mut h = 0;
    let mut w = 0;
    for (row, line_result) in BufReader::new(f).lines().enumerate() {
        let line = line_result.unwrap();
        for (col, chr) in line.chars().enumerate() {
            if chr != '.' {
                let p = Point::new(row as i32, col as i32);
                antennas.entry(chr).or_insert(Vec::new()).push(p);
            }
            w = col;
        }
        h = row;
    }
    let bounds = Point{
        row: h as i32,
        col: w as i32
    };
    (bounds, antennas) 
}

fn antinodes(antenna:&Vec<Point>, bounds: &Point) -> HashSet<Point> {
    antenna.iter().tuple_combinations()
    .flat_map(|(&a, &b)| {
        let delta = a - b;
        [a + delta, b - delta]
    })
    .filter(|p| p.within(bounds))
    .collect()
}

fn resonant_antinodes(antenna:&Vec<Point>, bounds: &Point) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    antinodes.extend(antenna);

    antenna
    .iter()
    .tuple_combinations()
    .for_each(|(&a, &b)| {
        let delta = a - b;
        let mut n1 = a + delta;
        let mut n2 = b - delta;

        while n1.within(bounds) {
            antinodes.insert(n1);
            n1 = n1 + delta;
        }
        while n2.within(bounds) {
            antinodes.insert(n2);
            n2 = n2 - delta;
        }
    });
    antinodes

}

fn part_one(antennas:&HashMap<char, Vec<Point>>, bounds: &Point) -> usize {
    antennas
    .iter()
    .map(|(_, v)| antinodes(v, bounds))
    .fold(HashSet::new(), |mut a:HashSet<Point>, s| {
        a.extend(&s);
        a
    })
    .len()
}

fn part_two(antennas:&HashMap<char, Vec<Point>>, bounds: &Point) -> usize {
    antennas
    .iter()
    .map(|(_, v)| resonant_antinodes(v, bounds))
    .fold(HashSet::new(), |mut a:HashSet<Point>, s| {
        a.extend(&s);
        a
    })
    .len()
}
fn main() {
    let path = Path::new("data.txt");
    let f = File::open(path).expect("Could not open the file!");

    let (bounds, antennas) = parse_input(f);
    println!("{:?}", part_one(&antennas, &bounds));
    println!("{:?}", part_two(&antennas, &bounds));
}

