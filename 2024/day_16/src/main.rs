use std::fs;
use std::collections::HashSet;
use day_16::*;


fn count_paths(end: (usize, usize), costs:&Vec<Vec<[u64;4]>>) -> usize  {
    // work backward to find valid paths based on 
    // the costs saved from part one.

    let mut points:HashSet<(usize, usize)> = HashSet::new();
    let final_cost = costs[end.0][end.1].iter().min().unwrap();
    
    let mut stack:Vec<(u64, Point)> = DIRECTIONS.iter()
        .map(|&d| (*final_cost, Point{row:end.0, col:end.1, dir:d}))
        .collect();

    while !stack.is_empty() {
        let (target_cost, point) = stack.pop().unwrap();
        points.insert((point.row, point.col));

        if target_cost == 0 { // back at start
            continue
        }

        for direction in DIRECTIONS {
            let next = point.step(direction);
    
            let travel_cost = costs[next.row][next.col][direction.opposite() as usize];

            if direction == point.dir {   
                // move to the next spot             
                if travel_cost < u64::MAX && travel_cost + 1 == target_cost {
                    stack.push((target_cost - 1, point.step(direction)));
                } 
            } else {
                // turn in place, but don't move
                let travel_cost = costs[point.row][point.col][direction.opposite() as usize];             
                if travel_cost < u64::MAX && travel_cost + 1000 == target_cost  {
                    stack.push((target_cost - 1000, Point{dir: direction, ..point}));
                }
            }
        }
    }
    return points.len()
}
fn main() {
    let raw_map = fs::read_to_string("data.txt").expect("The elves lost the course map!");
    let maze:Maze = raw_map.into();
    if let Some((cost, costs)) = maze.least_cost() {
        println!("Part one: {}", cost);
    
        let paths = count_paths(maze.end, &costs);
        println!{"Part Two: {}", paths};    
    }
}
