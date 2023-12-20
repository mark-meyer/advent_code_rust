use std::ops::Add;


/*
This is all a little much. Just trying to get used to thinking in enums.
*/
#[derive(Debug)]
pub struct PipeMap {
    pub data: Vec<Vec<Pipe>>,
    pub start_pipe: Pipe
}

impl PipeMap {
    fn find_start(map: &Vec<Vec<Pipe>>) -> Pipe {
        for line in map {
            for found in line {
                if let Pipe::Start(_) = found {
                    return  *found;
                }
            }
        }
        panic!("Can't find starting pipe!")
    }

    pub fn replace_start_pipe(map: &mut Vec<Vec<Pipe>>, start_pipe: Pipe) -> Pipe  {
        // Figuring out what kind of pipe the start position is probably isn't neccesary, but
        // trying it out anywat.

        let vectors:Vec<(isize, isize)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let p:Point = if let Pipe::Start(p) = start_pipe { p.clone() } else {panic!("not a start pipt")};
        
        let mut flag = 0;
        for direction in vectors {
            flag = flag << 1;
            let coord = &p + direction;
            if coord.row >= 0 && coord.col >= 0 {
                let pipe = map[coord.row as usize][coord.col as usize];
                // does the pipe at the location connect to this point?
                if let Some((n1, n2)) = pipe.neighbor_points() {
                    if n1 == p || n2 == p {
                        flag += 1 
                    }
                }
            }
        }
       let new_point =  match flag {
            0b1100 => Pipe::LowerLeft(p), 
            0b1010 => Pipe::Vertical(p),
            0b1001 => Pipe::LowerRight(p),
            0b110 => Pipe::UpperLeft(p),
            0b101 => Pipe::Horizontal(p),
            0b11 => Pipe::UpperRight(p),
            _ => Pipe::Start(p)
        };
        map[p.row as usize][p.col as usize] = new_point;
        new_point
    }
}

impl From<&str> for PipeMap {
    fn from(s: &str) -> Self {
        let mut data:Vec<Vec<Pipe>> = s.lines()
        .enumerate()
        .map(|(row, line)| line
            .chars()
            .enumerate()
            .map(|(col, c)| Pipe::from((&c, Point{row: row as isize, col: col as isize} ) ))
            .collect()
        ).collect();

        let start_pipe: Pipe = PipeMap::find_start(&data);
        let start_pipe = PipeMap::replace_start_pipe(&mut data, start_pipe);

        PipeMap {
            data,
            start_pipe: start_pipe
        }
    }
}


// POINT 
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub row: isize,
    pub col: isize
}

impl Add<(isize, isize)> for &Point {
    type Output = Point;

    fn add(self, other: (isize, isize)) -> Point {
        Point {
            row: self.row + other.0,
            col: self.col + other.1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pipe  {
    Vertical(Point),
    Horizontal(Point),
    LowerLeft(Point),
    LowerRight(Point),
    UpperLeft(Point),
    UpperRight(Point),
    Start(Point),
    Null,
}

impl Pipe {
    pub fn neighbor_points(&self) -> Option<(Point, Point)> {
         match self {
            Pipe::Vertical(p)   => Some(( p + (-1, 0), p + (1, 0) )),
            Pipe::Horizontal(p) => Some(( p + (0, 1),  p + (0, -1) )),
            Pipe::LowerLeft(p)  => Some(( p + (-1, 0), p + (0, 1) )),
            Pipe::LowerRight(p) => Some(( p + (-1, 0), p + (0, -1) )),
            Pipe::UpperLeft(p)  => Some(( p + (1, 0),  p + (0, 1) )),
            Pipe::UpperRight(p) => Some(( p + (1, 0),  p + (0, -1) )),
            Pipe::Null          => None,
            Pipe::Start(_)      => None
        }
    }

    pub fn point(&self) -> Option<&Point> {
        match self {
            Pipe::Vertical(p)   => Some(p),
            Pipe::Horizontal(p) => Some(p),
            Pipe::LowerLeft(p)  => Some(p),
            Pipe::LowerRight(p) => Some(p),
            Pipe::UpperLeft(p)  => Some(p),
            Pipe::UpperRight(p) => Some(p),
            Pipe::Null          => None,
            Pipe::Start(_)      => None
        }
    }
}

impl From<(&char, Point)> for Pipe {
    fn from((symbol, p):(&char, Point)) -> Self {
        match (symbol, p) {
            ('|', p) => Pipe::Vertical(p),
            ('-', p) => Pipe::Horizontal(p),
            ('L', p) => Pipe::LowerLeft(p),
            ('J', p) => Pipe::LowerRight(p),
            ('F', p) => Pipe::UpperLeft(p),
            ('7', p) => Pipe::UpperRight(p),
            ('S', p) => Pipe::Start(p),
            ('.', _p) => Pipe::Null,
            _   => panic!("Unknown Pipe Type")
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neighbor_points() {
        let p = Pipe::Horizontal(Point {row: 10, col: 100});
        let option = p.neighbor_points();
        assert_eq!(option, Some((
            Point{row: 10, col:101},
            Point{row: 10, col:99},
        )));
    }
}