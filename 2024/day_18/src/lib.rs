use std::collections::VecDeque;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}
impl Point {
    pub fn new(row: usize, col: usize) -> Point {
        Point { row, col }
    }
}

pub struct Map {
    height: usize,
    width: usize,
    blocks: Vec<bool>,
}

impl Map {
    pub fn new(height: usize, width: usize, blocks: &[Point]) -> Self {
        let mut h = vec![true; height * width];
        for point in blocks {
            h[point.row * width + point.col] = false;
        }
        Map {
            height,
            width,
            blocks: h,
        }
    }
    fn neighbors(&self, p: Point) -> impl Iterator<Item = Point> + '_ {
        let Point { row, col } = p;
        DIRECTIONS
            .iter()
            .flat_map(move |d| match d {
                Direction::North if p.row > 0 => Some(Point { row: row - 1, col }),
                Direction::West if p.col > 0 => Some(Point { row, col: col - 1 }),
                Direction::South if p.row < self.height - 1 => Some(Point { row: row + 1, col }),
                Direction::East if p.col < self.width - 1 => Some(Point { row, col: col + 1 }),
                _ => None,
            })
            .filter(|p| self.blocks[p.row * self.width + p.col])
    }

    pub fn bfs(&self, start: &Point, end: &Point) -> Option<u32> {
        // see if this is a bit faster than a hashset:
        let mut seen = vec![false; self.height * self.width];
        seen[start.row * self.width + start.col] = true;

        let mut queue = VecDeque::from([(*start, 0)]);

        while let Some((current, dist)) = queue.pop_front() {
            if &current == end {
                return Some(dist);
            }
            for neighbor in self.neighbors(current) {
                let idx = neighbor.row * self.width + neighbor.col;
                if !seen[idx] {
                    seen[idx] = true;
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
        None
    }
}
