use std::collections::{VecDeque, HashSet};

pub mod kdtree;

pub enum Direction {
    North,
    East,
    South,
    West
}

const DIRECTIONS:[Direction; 4] = [
    Direction::North, 
    Direction::East,
    Direction::South,
    Direction::West
];

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point{
    row: usize,
    col: usize
}
impl Point {
    pub fn new(row: usize, col:usize) -> Self {
        Point {row, col}
    }
    pub fn to_uv(&self) -> (isize, isize) {
        let (row, col) = (self.row as isize, self.col as isize);
        (row + col, row - col)
    }
}

pub struct Map{
    width: usize,
    height: usize,
    start: Point,
    end: Point,
    pub matrix: Vec<Vec<Option<usize>>>
}

impl Map {
    fn get(&self, p:&Point) -> Option<usize>{
        self.matrix[p.row][p.col]
    }

    pub fn neighbors(&self, p: &Point) -> impl Iterator<Item = Point> + '_ {
        let &Point { row, col } = p;
        DIRECTIONS.iter().filter_map(move |d| {
            
            match d {
                Direction::North if row > 0 => Some(Point::new(row - 1, col)),
                Direction::West  if col > 0 => Some(Point::new(row,col - 1)),
                Direction::South if row < self.height - 1 =>  Some(Point::new(row + 1, col)),
                Direction::East  if col < self.width - 1 => Some(Point::new(row,col + 1)),
                _ => None
            }
        })
    }

    /// Perform BFS but skew the resulting path so 
    /// equal Manhattan distances will be in an 
    /// axis-aligned box.
    pub fn bfs(&mut self) -> Option<Vec<((isize, isize), usize)>> {
        let mut seen = HashSet::new();
        seen.insert(self.start);
        let mut path = vec![];
        let mut queue = VecDeque::from([(self.start, 0)]);

        while let Some((p, dist)) = queue.pop_front() {
            self.matrix[p.row][p.col] = Some(dist);

            path.push((p.to_uv(), dist));
            if p == self.end { 
               return Some(path);
            }
            for neighbor in self.neighbors(&p) {
                if !seen.contains(&(neighbor)) && self.get(&neighbor).is_some(){
                    seen.insert(neighbor);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
        None
    }
}

impl From<&String> for Map {
    fn from(s:&String) -> Self {
        let mut start = None;
        let mut end = None;
        let m:Vec<Vec<Option<usize>>> = s.split("\n")
        .enumerate()
        .map(|(row, line)| {
            line.chars()
            .enumerate()
            .map(|(col, c)| {
                match c {
                    '#' => None,
                    '.' => Some(usize::MAX),
                    'S' => {
                        start = Some(Point{row, col});
                        Some(0)
                    },
                    'E' => {
                        end = Some(Point{row, col});
                        Some(usize::MAX)
                    }
                    _ => panic!("Debis on the field!!")
                }
            }).collect()
        }).collect();
        Map {
            start: start.unwrap(),
            end: end.unwrap(),
            height: m.len(),
            width: m[0].len(),
            matrix: m,
        }
    }
}

#[derive(Debug)]
pub struct KdTree<T, const D: usize> {
    pub root:Option<Box<KdTreeNode<T, D>>>
}

#[derive(Debug)]
pub struct KdTreeNode<T, const D: usize> {
    value: [T; D],
    left: Option<Box<KdTreeNode<T,D>>>,
    right: Option<Box<KdTreeNode<T,D>>>
}

impl<T, const D: usize> KdTree<T, D> 
    where  T: PartialOrd + Copy + std::fmt::Debug
{
    pub fn insert(&mut self, value: [T; D]) {
        let mut d = 0;
        let mut current = &mut self.root;
        while let Some(next) = current {
            if value[d] < next.value[d] {
                current = &mut next.left;
            } else {
                current = &mut next.right;
            }
            d = (d + 1) % D;
        } 
        *current = Some(
            Box::new(KdTreeNode{
                value,
                left:None,
                right:None
            }))
    }

    pub fn values(&self) -> Vec<[T;D]> {
        let mut stack = vec![&self.root];
        let mut res = vec![];
        while !stack.is_empty() {
            match stack.pop().unwrap() {
                Some(next) => {
                    res.push(next.value);
                    stack.push(&next.left);
                    stack.push(&next.right);
                },
                None => {}
            }
        }
        res
    }

    pub fn find(&self, value: [T; D]) -> Option<&KdTreeNode<T, D>> {
        let mut current = &self.root;
        let mut d = 0;
        while let Some(next) = current {
            if next.value == value {
                return Some(&next)
            }
            if value[d] < next.value[d] {
                current = &next.left;
            } else {
                current = &next.right;
            }
            d = (d + 1) % D;
        }
        None
    }
    pub fn range_query(&self, min: [T; D], max: [T; D]) -> Vec<[T; D]> {
        let mut result = Vec::new();
        let mut stack = vec![(&self.root, 0)]; 
    
        while !stack.is_empty() {
            if let (Some(node), d) = stack.pop().unwrap() {
                let mut in_range = true;
                for i in 0..D {
                    if node.value[i] < min[i] || node.value[i] > (max[i]) {
                        in_range = false;
                        break;
                    }
                }
                if in_range {                    
                    result.push(node.value);
                }
    
                if min[d] <= node.value[d] {
                    stack.push((&node.left, (d + 1) % D));
                }
                if max[d] >= node.value[d] {
                    stack.push((&node.right, (d + 1) % D));
                }
            }
        }
        result
    }
} 

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kd_new() {
        let mut kd = KdTree{root:None};
        kd.insert([2, 3]);
        kd.insert([4, 4]);
        kd.insert([3, 2]);
        assert_eq!(&kd.root.as_ref().unwrap().value, &[2, 3] );
        assert_eq!(&kd.root.unwrap().right.unwrap().left.unwrap().value, &[3, 2])
    }
    #[test]
    fn test_kd_range() {
        let mut kd = KdTree{root:None};
        kd.insert([6, -4, 2]);

        //let res = kd.range_query([2, 2], [4,4]);
        //assert_eq!(res, vec![[2,4], [4,3], [4,3]]);

        let res = kd.range_query([2, -4, -10], [6,0, 20]);
        assert_eq!(res, vec![[6,-4, 2]])
    }
    #[test]
    fn test_values() {
        let mut kd = KdTree{root:None};
        kd.insert([2, 3]);
        kd.insert([4, 4]);
        kd.insert([3, 2]);
        let res = kd.values();
        assert_eq!(res, [[2,3], [4,4], [3,2]])
    
    }
}

/*
  0 1 2 3 4 5 
0 . . . . . .
1 . . . . . .
2 . . * . . .
3 . . . . . .
4 . . . . * . 
5 . . . . . .
*/