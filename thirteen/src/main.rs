use std::fs;
use std::path::Path;
use std::collections::HashSet;
use thirteen::{parse_data, Point, make_strings};

const PATH:&str = "./data.txt";


fn main() {
    let path = Path::new(PATH);
    let raw_data = fs::read_to_string(path).expect("could not read file");
    let (points, folds) = parse_data(&raw_data);

    let points:Vec<Point> = points.into_iter()
    .map(|point| point.transform_on_axis(&folds[0])).collect();
    
    let count:HashSet<&Point> = HashSet::from_iter(points.iter());
    println!("Solution 1: {}", count.len());

    let points:Vec<Point> = points.into_iter()
        .map(|point| folds[1..].iter()
            .fold(point, |p, fold| p.transform_on_axis(fold))).collect();

    println!("Solution 2: \n{}", make_strings(&points).join("\n"));
}
