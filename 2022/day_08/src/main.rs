use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

static DATA:&str = "data.txt";

type ViewScape = Vec<Option<u32>>;

struct Forest {
    rows: Vec<Vec<u32>>,
    cols: Vec<Vec<u32>>,
    width: usize,
    height: usize
}

impl Forest {
    fn new(rows: Vec<Vec<u32>>) -> Forest {
        Forest {
            cols: transpose(&rows),
            width: rows[0].len() ,
            height: rows.len(),
            rows: rows,
        }
    }
    fn view_distances(&self, row: usize, column: usize,) -> ViewScape {
        /*
        Returns a vector of Options containing the distance until the first blocking
        tree looking left, right, up, and down.
        A particualr Option will be None if you can see all the way to the
        edge in that direction.
        */
        let n = self.rows[row][column];
        vec![
            // left
            index_greater(&n, &mut self.rows[row][..column].iter().rev()),
            // right
            index_greater(&n, &mut self.rows[row][column+1..].iter()),
            // up
            index_greater(&n, &mut self.cols[column][..row].iter().rev()),
            // down
            index_greater(&n, &mut self.cols[column][row+1..].iter())
        ]
    }
}


fn transpose(v: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.iter().map(|n| n.iter()).collect();

    (0..len)
    .map(|_| {iters.iter_mut().map(|n| *n.next().unwrap()).collect() })
    .collect()
}

fn index_greater<'a, I>(tree_height: &u32, view:I) -> Option<u32>
    where
    I: Iterator<Item = &'a u32>
    /*
    Returns the index of the first item in the iterator greater or
    equal to tree_height. Returns None if there are no items greater or equal
    */
{
    view
    .enumerate()
    .filter(|(_, n)| *n >= tree_height)
    .map(|(i, _)| (i + 1) as u32)
    .next()
}


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

fn part_one(forest: &Forest) -> u32 {
    let mut total = 0;
    for row in 0..forest.height {
        for column in 0..forest.width {
            let viewscape = forest.view_distances(row, column);
            if viewscape.iter().any(|o| o.is_none()) {
                total+=1
            }
        }
    }
    total
}


fn part_two(forest: &Forest) -> u32 {
    let mut max = 0;
    for row in 0..forest.height {
        for column in 0..forest.width {
            let viewscape = forest.view_distances(row, column);
            let total = viewscape[0].unwrap_or(column as u32)
                * viewscape[1].unwrap_or((forest.width - column - 1) as u32)
                * viewscape[2].unwrap_or(row as u32)
                * viewscape[3].unwrap_or((forest.height - row - 1) as u32);

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
    let forest = Forest::new(lines);
    println!("Part One: {}", part_one(&forest));
    println!("Part Two: {}", part_two(&forest));
}

