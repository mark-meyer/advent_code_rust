use std::fs::File;
use std::io::{BufRead, BufReader};
use day_13::*;

fn parse_input(f: File) -> Vec<Machine>{
    let lines:Vec<_> = BufReader::new(f).lines().filter_map(Result::ok).collect();
    
    lines
    .chunks(4)
    .map(|lines| {
        Machine{
            button_a: lines[0].as_str().into(),
            button_b: lines[1].as_str().into(),
            prize: lines[2].as_str().into()
        }
    })
    .collect()
}

fn main() {
    let f = File::open("data.txt").expect("where'd my claw machine go?");
    let machines = parse_input(f);
    let part_one:i64 = machines.iter().flat_map(|m| m.min_route()).sum();

    let part_two:i64 = machines.iter()
    .map(|m| {
        let (x, y) = (m.prize.x + 10000000000000, m.prize.y + 10000000000000);
        Machine {prize: Button{x, y}, ..*m}
        })
    .flat_map(|m| m.min_route())
    .sum();

    println!("Part One: {:?}", part_one);
    println!("Part Two: {:?}", part_two);

}
