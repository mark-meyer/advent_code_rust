use std::fs::read_to_string;

use day_07::{parse_file, run_transporter};

fn main() {
    let raw_input = read_to_string("data.txt").expect("Couldn't start the tachyon emiiter");
    let manifold = parse_file(&raw_input);

    let (paths, splits) = run_transporter(&manifold);
    println!("Part one: {}", splits);
    println!("Part two: {}", paths);
}
