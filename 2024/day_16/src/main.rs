use std::fs;
use day_16::*;


fn main() {
    let raw_map = fs::read_to_string("data.txt").expect("The elves lost the course map!");
    let mut maze:Maze = raw_map.into();
    if let Some((cost, len)) = maze.least_cost() {
        println!("Part one: {}", cost);
        println!("Part Two: {}", len);
    }



}
