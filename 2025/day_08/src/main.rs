use day_08::*;
use std::fs::File;


fn main() {
    let f = File::open("data.txt").expect("could not open file");
    let data = parse_file(f).expect("Could not parse file!");
    let kd_tree = make_kd_tree(&data);
    let res = part_one(&kd_tree, 1000);
    println!("part one {}", res);

    let res = part_two(&kd_tree, 1000);
    println!("part one {}", res);
}
