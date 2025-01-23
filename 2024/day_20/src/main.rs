use std::fs;
use std::cmp::max;
use day_20::*;

/// points are (u,v) points to skew them to manhattan distance is axis aligned.
fn find_cheats(uv_points:&[((isize, isize), usize)], cheat_length: usize, min_savings:usize) -> usize{
    let mut left = 0;
    let mut right = 1;
    let len = uv_points.len();
    let mut total = 0;

    for &((u, v), start_dist) in uv_points {

        while  right < len - 1 && uv_points[right].0.0 < u + 1 + cheat_length as isize {
            right += 1;
        }
        while  uv_points[left].0.0 < u - cheat_length  as isize {
            left += 1;
        }
        for  &((u_next, v_next), end_dist) in &uv_points[left..right+1] {
            let cheat = max(v.abs_diff(v_next), u.abs_diff(u_next));   // manhattan distance between the two points

            if cheat <= cheat_length                                   // don't go through too many walls
            && start_dist + cheat < end_dist                           // the cheat is actually an improvement
            && end_dist.abs_diff(start_dist) - cheat >= min_savings {  // avoid counting tiny savings
                total += 1;
            } 
        }
    }
    total
}
fn main() {
    let s = fs::read_to_string("data.txt").expect("no map found?");
    let mut map = Map::from(&s);
    let mut uv_path =  map.bfs().unwrap();

    uv_path.sort_by_key(|((u, _v), _dist)| *u);
    
    let part_one = find_cheats(&uv_path, 2, 100);
    println!("Part one: {}", part_one);

    let part_two = find_cheats(&uv_path, 20, 100);
    println!("Part two: {}", part_two);
}
