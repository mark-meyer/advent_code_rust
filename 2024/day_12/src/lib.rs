use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    NorthWest,
    NorthEast(usize),
    South(usize),
    SouthWest(usize),
    SouthEast(usize, usize),
    East(usize),
    West
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize
}

impl Point {
    pub fn step(self, direction: &Direction) -> Option<Point> {
        let Point{row, col} = self; 
        match direction {
            Direction::North if row > 0  => 
                Some(Point{row: row - 1, col}),
            Direction::NorthWest if row > 0 && col > 0 =>
                Some(Point{row: row - 1, col: col - 1}),
            Direction::West if col > 0  => 
                Some(Point{row, col:col - 1}),
            Direction::SouthWest(bound) if row < *bound-1 && col > 0 =>
                Some(Point{row: row + 1, col: col - 1}),           
            Direction::South(bound) if row < *bound-1 => 
                Some(Point{row: row + 1,col}),
            Direction::SouthEast(r_bound, c_bound) if row < *r_bound-1 && col < *c_bound-1 =>
                Some(Point{row: row + 1, col: col + 1}),
            Direction::East(bound) if col < *bound-1  => 
                Some(Point{row, col:col + 1}),
            Direction::NorthEast(bound) if row > 0 && col < *bound-1 =>
                Some(Point{row: row - 1, col: col + 1}),
            _ => None
        }
    }
}

type Matrix = Vec<Vec<char>>;
#[derive(Debug)]
pub struct Field {
    pub matrix: Matrix,
    pub h: usize,
    pub w: usize
}

impl Field {
    pub fn get(&self, p:&Point) -> char{
        self.matrix[p.row][p.col]
    }
}
impl TryFrom<File> for Field {
    type Error = std::io::Error;
    fn try_from(f: File) -> Result<Self, Self::Error> {
        let matrix = BufReader::new(f)
        .lines()
        .map(|line| {
            let line = line?;
            Ok(line.chars().collect())
        })
        .collect::<Result<Matrix, Self::Error>>()?;

        let h = matrix.len();
        let w = matrix[0].len();
        Ok(Field{matrix, h, w})
    }
}
pub struct FieldIter<'a> {
    field: &'a Field,
    row: usize,
    col: usize
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = (Point, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.field.h {
            return None
        }
        let p = Point{row: self.row, col:self.col};
        let c = self.field.matrix[self.row][self.col];

        self.col += 1;
        if self.col >= self.field.w {
            self.col = 0;
            self.row += 1;
        }
        Some((p, c))
    } 
}

impl<'a> IntoIterator for &'a Field {
    type Item = (Point, char);
    type IntoIter = FieldIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FieldIter {
            field: &self,
            row: 0,
            col: 0
        }
    }
}