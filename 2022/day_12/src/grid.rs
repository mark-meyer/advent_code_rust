pub struct Grid {
    height: usize,
    width: usize,
}

impl Grid {
    // p = (row, col)
    pub fn new(h:usize, w: usize) -> Grid {
        Grid {height: h, width: w }
    }
    pub fn neighbors(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        //println!("Getting neighbors of {:?}", p);
        let mut v = Vec::with_capacity(4);
        if p.0 > 0 {
            v.push((p.0 - 1, p.1));
        }
        if p.1 > 0 {
            v.push((p.0, p.1 -1));
        }
        if p.0 + 1 < self.height {
            v.push((p.0 + 1, p.1));
        }
        if p.1 + 1 < self.width {
            v.push((p.0, p.1 + 1));
        }

        v
    }
}