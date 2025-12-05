use std::fs::File;

use day_05::*;




fn main() {
    let f = File::open("data.txt").expect("Could not open ingredients database!");
    let (ranges, ingredients) = parse_file(f).expect("Could not parse this file!");

    println!("{:?}", part_one(&ranges, &ingredients));
    println!("{:?}", part_two(&ranges));

}
