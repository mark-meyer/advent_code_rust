use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
pub struct SegmentData {
    pub value: u64,
    pub data: Vec<u32>,
}
pub struct SegmentTree<F>
where F: Fn(&SegmentData, &SegmentData) -> Ordering {
    /// A bindar tree that keeps track of its largest
    /// child. Nodes are arranged in a flat list
    /// to be cache friendly. 
    /// For a vec of size s. Leaf nodes start at t[s]
    /// The children of a parent at t[s] are found at t[s*2] and t[s*2+1]
    /// Likewise a nodes parent is a t[s/2] (integer division)
    size: usize,
    n: usize,
    tree: Vec<SegmentData>,
    comparator: F
}

impl<F> SegmentTree<F>
where F: Fn(&SegmentData, &SegmentData) -> Ordering {
    pub fn new(data: &[SegmentData], comparator:F) -> Self {
        let n = data.len();
        let mut size = 1;

        while size < n {
            size <<= 1;
        }

        let default = SegmentData {
            value: 0,
            data: Vec::new(),
        };
        
        let mut tree = vec![default.clone(); 2 * size];
        
        // leaves go in the right half of the vec
        for (i, item) in data.iter().enumerate() {
            tree[i + size] = item.clone()
        }

        // Capture the largest size available in sub-tree
        for i in (0..size).rev() {
            let left = &tree[2 * i];
            let right = &tree[2 * i + 1];

            tree[i] = match comparator(left, right) {
                Ordering::Greater => left.clone(),
                _ => right.clone(),
            };
        }

        SegmentTree {
            size,
            n,
            tree,
            comparator
        }
    }

    pub fn get(&self, idx: usize) -> Option<&SegmentData> {
        if idx >= self.size {
            None
        } else {
            self.tree.get(self.size + idx)
        }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &SegmentData> {
        self.tree[self.size..self.size + self.n].iter()
    }

    pub fn update(&mut self, idx: usize, value: u64, data:Vec<u32>) {
        if idx >= self.size {
            return
        }

        let mut pos = self.size + idx;
        let seg_data = self.tree.get_mut(pos).unwrap();
        seg_data.value = value;
        seg_data.data = data;

        while pos >= 1 {
            pos /= 2;
            let left = &self.tree[2 * pos];
            let right = &self.tree[2 * pos + 1];

            let new_val = match (self.comparator)(left, right) {
                Ordering::Greater => left.clone(),
                _ => right.clone(),
            };
            if self.tree[pos] == new_val {
                // No need to propagated upwards
                break; 
            }
            self.tree[pos] = new_val;
        }
    }

    pub fn leftmost_fee_space(&self, x: u64) -> Option<usize> {
        // Find the left most node that can accomodate x space
        if (self.comparator)(&self.tree[1], &SegmentData { value: x, data: Vec::new() }) == Ordering::Less {
            return None;
        }
        let mut idx = 1;
        while idx < self.size {
            let left = 2 * idx;
            if (self.comparator)(&self.tree[left], &SegmentData { value: x, data: Vec::new() }) != Ordering::Less {
                idx = left;
            } else {
                idx = left + 1;
            }
        }
        let result = idx - self.size;
        if result < self.size {
            Some(result)
        } else {
            None
        }
    }
}
