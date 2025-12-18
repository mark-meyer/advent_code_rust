use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;


#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

struct Poly {
    poly: Vec<Point>,
    xs: Vec<u32>,
    ys: Vec<u32>,
    len: usize,
}

impl Poly {
    fn new(poly: Vec<Point>) -> Self {
        let mut xs: Vec<u32> = poly.iter().map(|p| p.x).collect();
        let mut ys: Vec<u32> = poly.iter().map(|p| p.y).collect();
        // sorted unique x and y values
        xs.sort(); xs.dedup();
        ys.sort(); ys.dedup();

        let len = poly.len();
        
        Poly {
            poly,
            xs,
            ys,
            len 
        }

    }
    fn get_x_idx(&self, x_val:u32) -> usize {
        self.xs.binary_search(&x_val).unwrap()
    }
    fn get_y_idx(&self, y_val:u32) -> usize {
        self.ys.binary_search(&y_val).unwrap()
    }

        
}


pub fn part_one(poly: &[Point]) -> u64 {
    // we could probably find the dominant points on the outside and import O(n^2)...maybe later
    let mut max = 0;
    let n = poly.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let area = (1 + poly[i].x.abs_diff(poly[j].x)) as u64 * (1 + poly[i].y.abs_diff(poly[j].y)) as u64;
            max = max.max(area);
        }
    }
    max
}

pub fn solve_compression(poly: &[Point]) -> u32 {
    // make a compression matrix to make constant time lookups
    // of outside vs inside. 

    let poly = Poly::new(poly.into());
    // compression matrix size
    let w = poly.xs.len() - 1;
    let h = poly.ys.len() - 1;
    let n = poly.len;

    // proporocess vertical edges
    let mut vert_edges = vec![Vec::new(); poly.xs.len()];
    
    // help lookup index
    let p_indices: Vec<(usize, usize)> = poly.poly.iter().map(|p| {
        (poly.get_x_idx(p.x), poly.get_y_idx(p.y))
    }).collect();

    for i in 0..n {
        let p1 = poly.poly[i];
        let p2 = poly.poly[(i + 1) % n];

        // If vertical edge
        if p1.x == p2.x {
            let (x_idx, y1_idx) = p_indices[i];
            let (_, y2_idx) =  p_indices[(i + 1) % n];
            
            let (y_min, y_max) = if y1_idx < y2_idx { (y1_idx, y2_idx) } else { (y2_idx, y1_idx) };
            vert_edges[x_idx].push((y_min, y_max));
        }
    }
     
    let stride = h + 1;
    let mut pref = vec![0u32; (w + 1) * stride];
     
    for j in 0..h {
        let mut inside = false;
        
        // Accumulate the row prefix sum as we go
        let mut row_sum = 0; 
        
        for i in 0..w {
            // Does this cross any vertical edges at xs[i] that span this y-row?
            for &(ymin, ymax) in &vert_edges[i] {
                if j >= ymin && j < ymax {
                    inside = !inside;
                }
            }

            let val = if inside { 1 } else { 0 };
            row_sum += val;

            let curr_idx = (i + 1) * stride + (j + 1);
            let above_idx = curr_idx - 1; // since j moves by 1
            
            pref[curr_idx] = row_sum + pref[above_idx];
        }
    }
     
     
    let mut max_area = 0;

    for i in 0..n {
        let (ix1, iy1) = p_indices[i];
        let p1 = poly.poly[i];

        //  Order shouldn't matter for rectangle corners.
        for j in (i + 1)..n {
            let (ix2, iy2) = p_indices[j];
            let p2 = poly.poly[j];

            // Normalize ranges
            let (x_start, x_end) = if ix1 < ix2 { (ix1, ix2) } else { (ix2, ix1) };
            let (y_start, y_end) = if iy1 < iy2 { (iy1, iy2) } else { (iy2, iy1) };

            // if x_start == x_end || y_start == y_end { continue; }

            // Query 1D Prefix Sum
            // Sum = P[x2][y2] - P[x1][y2] - P[x2][y1] + P[x1][y1]
            let idx_br = x_end * stride + y_end;
            let idx_bl = x_start * stride + y_end;
            let idx_tr = x_end * stride + y_start;
            let idx_tl = x_start * stride + y_start;

            // watch out for subtraction overflow
            let sum_filled = (pref[idx_br] + pref[idx_tl]) - pref[idx_bl] - pref[idx_tr];
            let total_cells = (x_end - x_start) as u16 * (y_end - y_start) as u16;

            if sum_filled as u16 == total_cells {
                let area = (1+ p1.x.abs_diff(p2.x))  * (1 + p1.y.abs_diff(p2.y));
                if area > max_area { max_area = area; }
            }
        }
    }
    max_area as u32
}


pub fn parse_input(f: File) -> Result<Vec<Point>, Box<dyn Error>> {
    BufReader::new(f)
    .lines()
    .map(|line| {
        let line = line?;
        let (x, y) = line.split_once(",").ok_or(format!("couldn't split: {:?}", line))?;
        Ok(Point{x: x.parse()?, y: y.parse()?})
    })
    .collect()
}
