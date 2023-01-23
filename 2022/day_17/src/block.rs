use crate::jet::Jet;


pub struct Block {
    pub rows: Vec<u8>
}

impl Block {
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn intersects(&self, other: &[u8]) -> bool {
        let h = self.len().min(other.len());
        (0..h).any(|i| self.rows[i] & other[i] > 0)
    }

    pub fn shift(&mut self, jet: &Jet, neighbors: &[u8])  {
        match jet {
            Jet::Left => {
                let m = self.rows.iter().max().unwrap();
                let intersects = neighbors
                    .iter().enumerate()
                    .any(|(i, n)| self.rows[i] << 1 & n > 0);

                if *m << 1 < 127 && !intersects{
                    for n in &mut self.rows {
                        *n = *n << 1
                    }
                }
            },
            Jet::Right => {
                if !self.rows.iter().enumerate()
                    .any(|(i, n)| n % 2 == 1 || n >> 1 & neighbors.get(i).unwrap_or(&0) > 0 )
                {
                    for n in &mut self.rows {
                        *n = *n >> 1
                    }
                }
            }
        }

    }
}
