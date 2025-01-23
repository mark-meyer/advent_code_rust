use std::fs;
use std::cmp::max;
use day_20::*;

/// points are (u,v) points to skew them to manhattan distance is axis aligned.
fn find_cheats(tree:&KdTree<isize, 3>, uv_points:&[((isize, isize), usize)], cheat_length: isize, min_savings:usize) -> usize{
    let mut total = 0;

    for &((u, v), start_dist) in uv_points {

        for  &[u_next, v_next, end_dist] in &tree.range_query(
                [u - cheat_length , v - cheat_length , 0 as isize ],
                [u + cheat_length + 1, v + cheat_length + 1, 1000000]) {
            let cheat = max(v.abs_diff(v_next), u.abs_diff(u_next)) as isize;   // manhattan distance between the two points
            //println!("{}", end_dist.abs_diff(start_dist as isize) - (cheat as usize) );
            //if cheat <= cheat_length                                   // don't go through too many walls
            if (start_dist as isize) + cheat < end_dist                           // the cheat is actually an improvement
            && end_dist.abs_diff(start_dist as isize) - (cheat as usize) >= min_savings {  // avoid counting tiny savings
                println!("cheat: {}", end_dist.abs_diff(start_dist as isize) - (cheat as usize));

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
    
    let mut kdtree = KdTree{root:None};
    for &((u, v), d) in &uv_path {
        kdtree.insert([u, v, d as isize]);
    }

    
    uv_path.sort_by_key(|((u, _v), _dist)| *u);
    
    let part_one = find_cheats(&kdtree, &uv_path, 2, 0);
    println!("Part one: {}", part_one);

    // let part_two = find_cheats(&uv_path, 20, 100);
    // println!("Part two: {}", part_two);
}
