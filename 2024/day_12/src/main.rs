use std::fs::File;
use std::collections::HashSet;
use day_12::*;

fn flood_fill(p:&Point, crop:&char, field:&Field) -> (HashSet<Point>,usize) {
    /* Get contigous element…get the perimeter while we're here. */
    let directions = [
        Direction::South(field.h),
        Direction::East(field.w),
        Direction::West,
        Direction::North,
    ];

    let mut group = HashSet::new();
    group.insert(*p);
    let mut stack = vec![*p];

    let mut total_perimeter = 0;

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        let mut local_perimeter = 4;

        for d in &directions {
            if let Some(neighbor) = current.step(&d) {
                if group.contains(&neighbor) {
                    local_perimeter -= 1;
                    continue
                }
                if field.get(&neighbor) == *crop {
                    stack.push(neighbor);
                    group.insert(neighbor);
                    local_perimeter -= 1;
                } 
            }
        }
        
        total_perimeter += local_perimeter;
    }
    (group, total_perimeter)
}

fn count_corners(point:&Point, field:&Field, group:&HashSet<Point>) -> usize {
    let corners = [
        [Direction::North, Direction::NorthEast(field.w), Direction::East(field.w)],
        [Direction::East(field.w), Direction::SouthEast(field.h, field.w), Direction::South(field.h)],
        [Direction::South(field.h), Direction::SouthWest(field.h), Direction::West],
        [Direction::West, Direction::NorthWest, Direction::North],
    ]
    .map(|ds| ds.map(|d| point.step(&d).and_then(|p| group.get(&p))));
    
    
    corners.iter().map(|corner| {
        match corner {
            [None, None, None] =>  1,
            [None, Some(_), None] =>  1,
            [Some(_),None,Some(_)] =>  1,
            _ => 0
        }
    }).sum()
}

fn part_two(groups:&Vec<HashSet<Point>>, field:&Field) -> usize{
    groups.iter().map(|group| {
        group.iter()
        .map(|p| count_corners(p, field, &group))
        .sum::<usize>() * group.len()
    }).sum()
}

fn get_crop_groups(field:&Field) -> (Vec<HashSet<Point>>, usize) {
    let mut groups = Vec::new();
    let mut seen:HashSet<Point> = HashSet::new();
    let mut total = 0;
    for (p, crop) in field {
        if seen.contains(&p) { continue }
        
        let (group, perimeter_len) = flood_fill(&p, &crop, &field);
        
        total += group.len() * perimeter_len;
        seen.extend(&group);
        groups.push(group);
    }
    (groups, total)
}

fn main() {
    let f = File::open("data.txt").expect("No garden!");
    let field:Field = Field::try_from(f).expect("Could not parse field data");

    let (groups, total) = get_crop_groups(&field);
    println!("Part One: {}", total);
    println!("Part two: {}", part_two(&groups, &field));

}
