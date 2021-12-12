use std::path::Path;
use std::fs::File;
use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, BufReader};
use ndarray::prelude::*;
use std::cmp::{max, min};

const PATH: &str = "./data.txt";

fn parse_string_data(f: File, w:usize, h:usize) -> Array2<u32> {
    let f = BufReader::new(f);

    let arr: Vec<u32> = f.lines()
    .flat_map(|l| l.unwrap().trim().chars()
         .map(|n| n.to_digit(10).unwrap())
         .collect::<Vec<u32>>())
    .collect();    
    Array2::from_shape_vec((h, w), arr).unwrap()
}

fn step(grid: &mut Array2<u32>) {
    *grid += 1;
    *grid %= 10;
}

fn get_neighbors(p: (usize, usize), grid: &Array2<u32>) -> Vec<(usize, usize)>{
    let mut points = Vec::new();
    let row_min = max(p.0 as isize - 1, 0);
    let row_max = min(p.0 as isize + 1 , grid.shape()[0] as isize -1);
    let col_min = max(p.1 as isize - 1, 0);
    let col_max = min(p.1 as isize + 1, grid.shape()[0] as isize -1 );
    for row  in row_min..row_max+1 {
        for col  in col_min..col_max+1 {
            let n = (row as usize, col as usize);
            if n != p {
                points.push(n)
            }
        }
    }
    points
}

fn flash(grid: &mut Array2<u32>) -> usize {
    let mut init_flashes = VecDeque::from(get_flashing_index(grid));
    let mut flash_count: usize = init_flashes.len();
    let mut seen:HashSet<(usize, usize)> = HashSet::new();
    for coord in &init_flashes {
        seen.insert(coord.clone());
    }
    while !init_flashes.is_empty() {
        let next = init_flashes.pop_front().unwrap();
        seen.insert(next.clone());
        for coord in get_neighbors(next, &grid) {
            if seen.contains(&coord) {continue}
            
            grid[[coord.0, coord.1]] += 1;
            grid[[coord.0, coord.1]] %= 10;
            if grid[[coord.0, coord.1]] == 0 {
                // just flashed a neighbor
                flash_count += 1;
                seen.insert(coord.clone());
                init_flashes.push_back(coord)
            }  
        }
    }
    flash_count
}

fn get_flashing_index(grid: &Array2<u32>) -> Vec<(usize, usize)>{
    grid.indexed_iter()
    .filter(|(_pair, &value)| value == 0)
    .map(|(pair, _values)| pair)
    .collect()
}

fn main() {
    let w = 10;
    let h = 10;
    let path = Path::new(PATH);
    let file = File::open(path).expect("Cephalopod Danger! Can't find the file.");
    let mut grid = parse_string_data(file, w, h);
    let mut flashes = 0;
    let mut step_count = 0;
    let mut solution_one = 0;

    loop {
        step_count += 1;
        step(&mut grid);
        flashes += flash(&mut grid);
        if step_count == 100 {
            solution_one = flashes;
        }
        let is_zero = grid.fold(true, |a, n| a & (*n == 0));
        if is_zero {
            break
        }
        if step_count > 1000 {
            println!("took too long");
            break
        }
    }
    println!("Solution One: {:?}", solution_one);
    println!("Solution Two: {:?}", step_count);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_neighbors() {
        let grid = Array2::<u32>::zeros((5,5));
        let ns = get_neighbors((0, 0), &grid);
        assert_eq!(ns, vec![(0,1),(1,0),(1,1)]);
        
        let ns = get_neighbors((4, 4), &grid);
        assert_eq!(ns, vec![(3,3),(3,4),(4,3)]);

        let ns = get_neighbors((1, 1), &grid);
        assert_eq!(ns, vec![(0,0), (0,1), (0,2), (1,0), (1,2), (2,0), (2,1), (2,2)]);
    }
}