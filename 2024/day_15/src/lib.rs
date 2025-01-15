use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize
}

impl Point {
    pub fn step(self, direction: &Direction) -> Point {
        let Point{row, col} = self; 
        match direction {
            Direction::North => Point{row: row - 1, col},
            Direction::West =>  Point{row, col:col - 1},
            Direction::South => Point{row: row + 1, col},
            Direction::East => Point{row, col:col + 1},
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Object {
    Box,
    Wall,
    Space,
    CrateLeft,
    CrateRight,
    Bot
}

type Matrix = Vec<Vec<Object>>;
#[derive(Debug)]
pub struct Warehouse {
    pub map: Matrix,
    pub bot: Point,
}
impl Warehouse {
    pub fn width(&self) -> usize {self.map[0].len()}
    pub fn height(&self) -> usize {self.map.len()}

    fn get(&self, point:&Point) -> &Object {
        &self.map[point.row][point.col]
    }

    pub fn get_moves(&self, point:Point, dir:&Direction) -> Option<Vec<(Point, Point)>> {
        let mut current = point;
        let mut next = point.step(dir);
        let mut moves = vec![];
        
        loop {
            match self.get(&next) {
                Object::Wall => 
                    return None,
                Object::Space => {
                    moves.push((next, current));
                    return Some(moves)
                },
                _ => {
                    moves.push((next, current));
                    (current, next) = (next,  current.step(dir));
                }
            }
        }
    }

    pub fn push(&mut self, dir:&Direction){
        let point = self.bot;
        if let Some(moves) = self.get_moves(point, dir) {
            for (dest, source) in moves.iter().rev() {
                let src_obj = self.get(source).clone();
                let dst_obj = self.get(dest).clone();

                self.map[source.row][source.col] = dst_obj;
                self.map[dest.row][dest.col] = src_obj;
            }
        self.bot = self.bot.step(&dir);
        }
    }

    pub fn score(&self) -> usize{
        self.map
        .iter()
        .enumerate()
        .flat_map(|(row, line)| line
            .iter()
            .enumerate()
            .map(move |(col, obj)| match obj {
                Object::Box => row  * 100 + col,
                _ => 0 
            })
        ).sum()
    }

    pub fn expand(&self) -> Warehouse {
        let mut bot_pos = None;
        let matrix = self.map.iter()
        .enumerate()
        .map(|(row, line)| line.iter().enumerate()
            .flat_map(|(col, c)| {
                match c {
                    Object::Wall => [Object::Wall, Object::Wall],
                    Object::Box => [Object::CrateLeft, Object::CrateRight],
                    Object::Bot => {
                        bot_pos = Some(Point{row:row, col:col*2});
                        [Object::Bot, Object::Space]
                    }
                    _ => [Object::Space, Object::Space]
                }
            }).collect()
        ).collect();

        Warehouse {map:matrix, bot:bot_pos.unwrap()}
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v:String= self.map
        .iter()
        .map(|line| line
            .iter()
            .map(|obj| match obj {
                Object::Wall => "#",
                Object::Box => "O",
                Object::Bot => "@",
                Object::CrateLeft => "[",
                Object::CrateRight => "]",
                Object::Space => "." 
            }).collect::<Vec<&str>>().join("")
        ).collect::<Vec<String>>().join("\n");
        write!(f, "{}", v)
        
    }
}