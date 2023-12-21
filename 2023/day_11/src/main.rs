use std::path::Path;
use std::fs::File;

use day_11::*;

fn main() {
    
    let path = Path::new("data.txt");
    let file = File::open(path).expect("Cannot open file");

    let universe = Universe::from(file);
    // let g1 =  &universe.galaxies[0];
    // let g2 =  &universe.galaxies[6];
    let d = universe.pair_distances(2);
    println!("Part One {}", d);

    let d = universe.pair_distances(1000000);
    println!("Part two {}", d);
}
