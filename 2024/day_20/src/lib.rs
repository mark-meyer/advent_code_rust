use std::collections::{HashSet, VecDeque};

pub enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    row: usize,
    col: usize,
}
impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }
    pub fn to_uv(&self) -> (isize, isize) {
        let (row, col) = (self.row as isize, self.col as isize);
        (row + col, row - col)
    }
}

pub struct Map {
    width: usize,
    height: usize,
    start: Point,
    end: Point,
    pub matrix: Vec<Vec<Option<usize>>>,
}

impl Map {
    fn get(&self, p: &Point) -> Option<usize> {
        self.matrix[p.row][p.col]
    }

    pub fn neighbors(&self, p: &Point) -> impl Iterator<Item = Point> + '_ {
        let &Point { row, col } = p;
        DIRECTIONS.iter().filter_map(move |d| match d {
            Direction::North if row > 0 => Some(Point::new(row - 1, col)),
            Direction::West if col > 0 => Some(Point::new(row, col - 1)),
            Direction::South if row < self.height - 1 => Some(Point::new(row + 1, col)),
            Direction::East if col < self.width - 1 => Some(Point::new(row, col + 1)),
            _ => None,
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
                if !seen.contains(&(neighbor)) && self.get(&neighbor).is_some() {
                    seen.insert(neighbor);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
        None
    }
}

impl From<&String> for Map {
    fn from(s: &String) -> Self {
        let mut start = None;
        let mut end = None;
        let m: Vec<Vec<Option<usize>>> = s
            .split("\n")
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        '#' => None,
                        '.' => Some(usize::MAX),
                        'S' => {
                            start = Some(Point { row, col });
                            Some(0)
                        }
                        'E' => {
                            end = Some(Point { row, col });
                            Some(usize::MAX)
                        }
                        _ => panic!("Debis on the field!!"),
                    })
                    .collect()
            })
            .collect();
        Map {
            start: start.unwrap(),
            end: end.unwrap(),
            height: m.len(),
            width: m[0].len(),
            matrix: m,
        }
    }
}
