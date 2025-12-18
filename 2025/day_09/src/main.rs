use std::fs::File;

use day_09::*;


fn main() {
    let f = File::open("data.txt").expect("Couldn't open the map!");
    let coords = parse_input(f).expect("Could not parse input");
    let p1_result = part_one(&coords);
    println!("Part One: {}", p1_result);

    let p2_result = solve_compression(&coords);
    println!("Part Two: {}", p2_result);
}



