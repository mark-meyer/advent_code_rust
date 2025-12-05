use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

#[derive(Debug, Clone)]
pub struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize
}

impl Grid {
    fn index(&self, row: usize, col: usize) -> usize {
    self.width * row + col
}
}

pub fn parse_input(f:File) -> Grid{
    let mut data: Vec<u8> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;

    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.expect("read line");
        let bytes = line.as_bytes();

        if width == 0 {
            width = bytes.len()
        }
        data.extend(bytes.iter().map(|&b| (b == b'@') as u8));
        height += 1;
    }
    Grid {data, width, height}
}



fn sum_neighbors(grid: &Grid, row:usize, col:usize) -> u8 {
    let row_min = row.saturating_sub(1);
    let col_min = col.saturating_sub(1); 
    let row_mox = (row + 1).min(grid.height - 1);
    let col_mox = (col + 1).min(grid.width - 1);

    let mut val = 0;
    for r in row_min..=row_mox{
        for c in col_min..=col_mox {
            val += grid.data[grid.index(r, c)]
        }
    }
    val - grid.data[grid.index(row, col)]
}

pub fn part_one(grid: &Grid) -> u16 {
    let mut removed = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            let v = grid.data[grid.index(row, col)];
            if v == 1 {
                let n = sum_neighbors(grid, row, col);
                if n < 4 { 
                    removed += 1;
                }
            }
        }
    }
    return removed;
} 

pub fn part_two_old(grid: &mut Grid) -> u16 {
    let mut removed = 0;
    let mut changed = true;
    let mut next = Grid { data: vec![0; grid.data.len()], width: grid.width, height: grid.height };

    while changed {
        changed = false;
        for row in 0..grid.height {
            for col in 0..grid.width {
                let idx = grid.index(row, col);
                let v = grid.data[idx];
                if v == 1 {
                    let n = sum_neighbors(grid, row, col);
                    if n < 4 { 
                        removed += 1;
                        next.data[idx] = 0;
                        changed = true;
                    } else {
                        next.data[idx] = 1
                    }
                } else {
                    next.data[idx] = 0
                }
            }
        }
        swap(grid,  &mut next);
    }
    removed
}

/*
    sum_neighbors was fine for part one. But let's try to avoid
    the saturating_sub bounds checks and branching in the hot loop.

    Using a padded buffer to store calculations wil avoid this at
    the expense of some bookkeeping. 

    Benchmarks show this ~ 1.7 ms vs 3.3 ms for the original
 */ 


pub fn part_two(grid: &mut Grid) -> usize {
    let w = grid.width;
    let h = grid.height;

    let mut removed_total = 0;
    let mut next = vec![0u8; grid.data.len()];

    // padded buffer for convolve
    let pw = w + 2;

    let mut pad = vec![0u8; (h + 2) * (w + 2)];

    loop {
        for row in 0..h {
            let src = row * h;
            let dst = (row + 1) * pw + 1;
            pad[dst..dst + w].copy_from_slice(&grid.data[src..src + w]);
        }

        let mut removed_this_round: usize = 0;

        for row in 0..h{
            let base = row * w;
            let pbase = (row + 1) * pw;
            for col in 0..w {
                let i = base + col;
                let v = grid.data[i];
                if v == 0 {
                    next[i] = 0;
                    continue;
                }
                let p = (pbase + (col + 1)) as usize;
                // convolve. Since we have a padded buffer, we don't have bounds checks
                let n =
                    pad[p - pw - 1] + pad[p - pw] + pad[p - pw + 1] +
                    pad[p - 1]                    + pad[p + 1]      +
                    pad[p + pw - 1] + pad[p + pw] + pad[p + pw + 1];

                if n < 4 {
                    next[i] = 0;
                    removed_this_round += 1;
                } else {
                    next[i] = 1;
                }
            }
        }
        if removed_this_round == 0 {
            break;
        }
        removed_total += removed_this_round;
        
        swap(&mut grid.data,  &mut next);
    }
    removed_total
}
