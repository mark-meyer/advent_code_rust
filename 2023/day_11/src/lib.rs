use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::fs::File;

#[derive(Debug)]
pub struct Universe {
    pub galaxies: Vec<Galaxy>,
    pub occupied_rows: HashSet<usize>,
    pub occupied_cols: HashSet<usize>  
}

impl Universe {
    pub fn distance(&self, g1: &Galaxy, g2: &Galaxy, scale_factor:usize) -> usize {
        let row_dist = g1.row.abs_diff(g2.row);
        let min_row = g1.row.min(g2.row);
        let expand_row_space = (min_row..(min_row + row_dist))
            .collect::<HashSet<usize>>()
            .difference(&self.occupied_rows).count();

        let col_dist = g1.col.abs_diff(g2.col);
        let min_col= g1.col.min(g2.col);
        let expand_col_space = (min_col..(min_col + col_dist))
            .collect::<HashSet<usize>>()
            .difference(&self.occupied_cols).count();
        
        row_dist + col_dist + (expand_row_space + expand_col_space) * (scale_factor -1)
    }

    pub fn pair_distances(&self, scale_factor:usize) -> usize {
        let mut total = 0;
        
        for i in 0..self.galaxies.len() {
            for j in i+1..self.galaxies.len() {
                total += self.distance(&self.galaxies[i] , &self.galaxies[j], scale_factor);
            }
        }
        total
    }
}

impl From<File> for Universe {
    fn from(file: File) -> Self {
        let reader = BufReader::new(file);
        
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut occupied_rows = HashSet::new();
        let mut occupied_cols = HashSet::new();
    
        for (row, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Galaxy{row, col});
                    occupied_rows.insert(row);
                    occupied_cols.insert(col);
                }
            }
        }
        Universe {galaxies, occupied_rows, occupied_cols}
    }
}

#[derive(Debug)]
pub struct Galaxy {
    pub row: usize,
    pub col: usize
}


