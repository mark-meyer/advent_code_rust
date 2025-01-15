use std::fs;
use day_15::*;

fn parse(data:&str) -> (Warehouse, Vec<Direction>) {
    let mut bot_pos = None;
    let (map_data, path_data) = data.split_once("\n\n").unwrap();
    
    let map = map_data.split("\n")
    .enumerate()
    .map(|(row, line)| line
        .chars().enumerate()
        .map(|(col, c)| {
            match c {
                '#' => Object::Wall,
                'O' => Object::Box,
                '@' => {
                    bot_pos = Some(Point{row, col});
                    Object::Bot
                }
                _ => Object::Space
            }
        }).collect()
    )
    .collect();

    let path  = path_data.trim().replace('\n', "")
    .chars()
    .map(|c| match c {
        '<' => Direction::West,
        '^' => Direction::North,
        '>' => Direction::East,
        'v' => Direction::South,
        bad => panic!("Bad direction {}", bad)
     })
     .collect();

    (
        Warehouse {map, bot:bot_pos.unwrap()},
        path
    )
 
}

fn main() {
    let data = fs::read_to_string("data.txt").expect("What? No map!");
    let (mut warehouse, path) = parse(&data);
    let big_warehouse = &warehouse.expand();

    println!("{:?}", &warehouse.bot);

    for dir in &path {
        warehouse.push(dir);
    }
    // println!("{}", warehouse);
    println!("score: {}", &warehouse.score());

    println!("{}", &big_warehouse);
    println!("bot: {:?}", &big_warehouse.bot);

}
