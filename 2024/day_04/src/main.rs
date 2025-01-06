use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

type WordMatrix = Vec<Vec<char>>;

pub struct PuzzleIterator<'a> {
    puzzle: &'a WordMatrix,
    row: i32,
    col: i32,
    d_row: i32,
    d_col: i32
}

impl<'a> PuzzleIterator<'a> {
    pub fn new(puzzle: &'a WordMatrix, row: i32, col: i32, d_row:i32, d_col:i32) -> Self {
        PuzzleIterator {
            puzzle,
            row, 
            col,
            d_row,
            d_col
        }
    }
}

impl<'a> Iterator for PuzzleIterator<'a> {
    type Item =&'a char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < 0 || self.row >= self.puzzle.len() as i32 {
            return None;
        }
        if self.col < 0 || self.col >= self.puzzle[0].len() as i32 {
            return None;
        }

        let ch = &self.puzzle[self.row as usize][self.col as usize];

        self.row += self.d_row;
        self.col += self.d_col;

        Some(ch)
    }
}

fn parse_input(f: File) -> Result<WordMatrix, std::io::Error> {
    let reader = BufReader::new(f);
    reader
    .lines()
    .map(|line_result| {
        let line = line_result?;        
        Ok(line.chars().collect())      
    })
    .collect()
}


fn findword_at(puzzle: &WordMatrix, row: i32, col:i32, d_col:i32, d_row:i32, word:&str) -> bool {
    let mut puzzle_iter = PuzzleIterator::new(puzzle, row, col, d_col, d_row);

    for w_chr in word.chars() {
        match puzzle_iter.next() {
            Some(&p_char) if p_char == w_chr => {}
            _ => return false
        }
    }
    true
}

fn count_all_directions(puzzle: &WordMatrix, row:i32, col:i32, word:&str) -> usize {
    let directions = [
        (1, 0), 
        (-1, 0),
        (0, 1),  
        (0, -1), 
        (1, 1),  
        (1, -1), 
        (-1, 1), 
        (-1, -1),
    ];
    directions
    .iter()
    .filter(|&&(d_row, d_col)| findword_at(puzzle, row, col, d_row, d_col, word))
    .count()
}

fn main() {
    let f = File::open(Path::new("data.txt")).expect("Could not open this file");
    let puzzle = parse_input(f).unwrap();

    /* Part One */
    let mut found = 0;
    for (r, row) in puzzle.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            found += count_all_directions(&puzzle, r as i32, c as i32, "XMAS");

        }
    }
    println!("Part one: {}", found);

    /* Part Two */
    let mut found2 = 0;
    for (r, row) in puzzle.iter().enumerate().skip(1).take(puzzle.len() - 2) {
        for (c,chr) in row.iter().enumerate().skip(1).take(row.len() - 2) {
            if *chr != 'A' {
                continue
            }
            let d_r = (puzzle[r-1][c-1], puzzle[r+1][c+1]);
            let u_r = (puzzle[r-1][c+1], puzzle[r+1][c-1]);
            if (d_r == ('M', 'S') || d_r == ('S', 'M')) && (u_r == ('M', 'S') || u_r == ('S', 'M')) {
                found2 += 1
            }
        }
    }
    println!("Part two: {}", found2);

}
