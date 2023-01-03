use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

static DATA:&str = "data.txt";

fn parse_input(p: &Path) -> Vec<Vec<u32>>{
    let f = File::open(p).expect("can't read the file");
    BufReader::new(f)
    .lines()
    .map(|line| parse_line(line.unwrap()))
    .collect()
}
fn parse_line(s: String) -> Vec<u32>{
    s.chars().flat_map(|c| c.to_digit(10)).collect()
}

fn transpose(v: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.iter().map(|n| n.iter()).collect();

    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect()
        })
        .collect()
}

fn index_taller<'a, I>(tree_height: &u32, view:I) -> Option<usize>
    where
    I: Iterator<Item = &'a u32>
{
    view
    .enumerate()
    .filter(|(_, n)| *n >= tree_height)
    .map(|(i, _)| i + 1)
    .next()
}

fn part_one(forest: &Vec<Vec<u32>>) -> usize {
    let mut visible:HashSet<(usize, usize)> = HashSet::new();

    let height = forest.len();
    let width = forest[0].len();

    // Forward - find cummulative max moving left-to-right in rows
    // and top-to-bottom in columns
    let mut column_max = forest[0].clone();
    for (row, line) in forest[1..height-1].iter().enumerate() {
        let mut row_max = line[0];
        for (column, n) in line[1..width-1].iter().enumerate() {
            if *n as u32 > row_max {
                row_max = *n;
                visible.insert((row+1, column+1));
            }
            if *n as u32 > column_max[column+1] {
                column_max[column+1] = *n;
                visible.insert((row+1, column+1));
            }
        }
    }

    // Backward the oppoite
    let mut column_max = forest[height-1].clone();
    for (row, line) in forest[1..height-1].iter().enumerate().rev() {
        let mut row_max = line[width-1];

        for (column, n) in line[1..width-1].iter().enumerate().rev() {
            if *n as u32 > row_max {
                row_max = *n;
                visible.insert((row+1, column+1));
            }
            if *n as u32 > column_max[column+1] {
                column_max[column+1] = *n;
                visible.insert((row+1, column+1));
            }
        }
    }
    let outside = 2 * height + 2 * width - 4; // don't count corners twice
    visible.len()  + outside

}


fn part_two(forest: &Vec<Vec<u32>>) -> u32 {
    let width = forest[0].len();
    let height = forest.len();
    let cols = transpose(&forest);
    let mut max = 0;
    for (row, line) in forest.iter().enumerate() {
        for (column, n) in line.iter().enumerate() {
            let col = &cols[column];

            let taller_right = index_taller(n, &mut line[..column]
                .iter().rev()).unwrap_or(column);
            let taller_left = index_taller(n, &mut line[column+1..]
                .iter()).unwrap_or(width - column - 1);
            let taller_up = index_taller(n, &mut col[..row]
                .iter().rev()).unwrap_or(row);
            let taller_down = index_taller(n, &mut col[row+1..]
                .iter()).unwrap_or(height - row - 1);

            let total = taller_right * taller_left * taller_up * taller_down;
            if total > max {
                max = total
            }
        }
    }

    max as u32
}
fn main() {
    let path = Path::new(DATA);
    let lines:Vec<Vec<u32>>  = parse_input(path);

    println!("Part One: {}", part_one(&lines));
    println!("Part Two: {}", part_two(&lines));
}

