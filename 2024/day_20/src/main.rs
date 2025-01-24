use rayon::prelude::*;
use std::cmp::max;
use std::fs;

use day_20::*;

/// points are (u,v) points to skew them to manhattan distance is axis aligned.
fn find_cheats(
    uv_points: &[((isize, isize), usize)],
    cheat_length: usize,
    min_savings: usize,
) -> usize {
    let len = uv_points.len();

    uv_points
        .par_iter()
        .enumerate()
        .map(|(i, &((u, v), start_dist))| {
            let mut left = i;
            let mut right = i;

            while right < len - 1 && uv_points[right + 1].0 .0 < u + 1 + cheat_length as isize {
                right += 1;
            }

            while left > 0 && uv_points[left - 1].0 .0 >= u - cheat_length as isize {
                left -= 1;
            }

            uv_points[left..=right]
                .iter()
                .filter(|&&((u_next, v_next), end_dist)| {
                    let cheat = max(v.abs_diff(v_next), u.abs_diff(u_next)); // Manhattan distance
                    cheat <= cheat_length
                        && start_dist + cheat < end_dist
                        && end_dist.abs_diff(start_dist) - cheat >= min_savings
                })
                .count()
        })
        .sum()
}

fn main() {
    let s = fs::read_to_string("data.txt").expect("no map found?");
    let mut map = Map::from(&s);
    let mut uv_path = map.bfs().unwrap();

    uv_path.sort_by_key(|((u, _v), _dist)| *u);

    let part_one = find_cheats(&uv_path, 2, 100);
    println!("Part one: {}", part_one);

    let part_two = find_cheats(&uv_path, 20, 100);
    println!("Part two: {}", part_two);
}
