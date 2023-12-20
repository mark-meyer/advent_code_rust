use std::path::Path;
use std::fs;
use day_10::*;


fn find_polygon<'a>(map: &'a PipeMap, start: &'a Pipe) -> Vec<&'a Pipe> {
    let mut current = start;
    let mut previous = start;
    let mut poly = Vec::new();

    loop {
        if let Some((p1, p2)) = current.neighbor_points() {
            let p1 = &map.data[p1.row as usize][p1.col as usize];
            let p2 = &map.data[p2.row as usize][p2.col as usize];

            if p1 != previous {
                previous = current;
                current = p1;
            } else {
                previous = current;
                current = p2;
            }
        } else {
            panic!("no poly")
        }
        poly.push(current);
        if current == start {
            break
        }
    }
    poly
}

fn shoelace(poly:&Vec<&Pipe>) -> isize {
    let mut pipes = poly.iter();
    let start = pipes.next().unwrap();
    let mut current = start;
    let mut total = 0;
    for next in pipes {
        let p1  = current.point().unwrap();
        let p2  = next.point().unwrap();
        let d =  (p2.row * p1.col) - (p1.row * p2.col);
        current = next;
        total += d
    } 
    // Don't forget the last link
    let p2 = start.point().unwrap();
    let p1 = current.point().unwrap();

    total  +=  (p2.row * p1.col) - (p1.row * p2.col);

    1+ (total - poly.len() as isize) / 2
}

fn main() {
    let path = Path::new("data.txt");
    let f = fs::read_to_string(path).expect("could not open file");
    let data = PipeMap::from(f.as_str());
    let start_pipe = data.start_pipe;
    let poly = find_polygon(&data, &start_pipe);
  
    println!("Part One: {}", (poly.len()) /  2);

    let area = shoelace(&poly);
    println!("Part two: {}", area);

}
