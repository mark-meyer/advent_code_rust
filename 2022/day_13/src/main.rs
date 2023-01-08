use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::packet::Packet;

mod packet; 

static  DATA:&str = "data.txt";

fn main() {
    let p = Path::new(DATA);
    let f = File::open(p).expect("could not find our packets!");

    let mut lines:Vec<Packet> = BufReader::new(f).lines()
        .map(|line| line.unwrap())
        .filter(|line| line != "")
        .map(|line| line.parse().expect("Can't parse packet"))
        .collect();

    let p1:usize = lines
        .chunks(2)
        .enumerate()
        .filter(|(_, pair)| pair[0] < pair[1])
        .map(|(i, _)| i + 1)
        .sum();

    println!("Part One: {}", p1);

    let d1:Packet = "[[2]]".parse().expect("Can't parse packet");
    let d2:Packet = "[[6]]".parse().expect("Can't parse packet");
    lines.push(d1.clone());
    lines.push(d2.clone());

    lines.sort();

    let d1_pos = lines.iter().position(|d| d == &d1).unwrap() + 1;
    let d2_pos = lines.iter().position(|d| d == &d2).unwrap() + 1;

    println!("Part Two: {}", d1_pos * d2_pos);

}
