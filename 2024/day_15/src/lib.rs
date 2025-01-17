use std::fmt;
use std::collections::HashSet;

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

    pub fn get_moves(&self, point:Point, dir:&Direction) -> Option<Vec<Point>> {
        let mut next = HashSet::from([point]); 
        let mut moves:Vec<Point> = vec![];
        let mut frontier = HashSet::new();
        while !next.is_empty() {

            for point in &next {
                let next_point = point.step(dir);

                match (self.get(&next_point), dir) {
                    (Object::CrateLeft, Direction::North|Direction::South) => {
                        frontier.extend([next_point, Point{row: next_point.row, col:next_point.col + 1}]);
                    },
                    (Object::CrateRight, Direction::North|Direction::South) => {
                        frontier.extend([next_point, Point{row: next_point.row, col:next_point.col - 1}]);
                    },
                    (Object::Space, _) => {},
                    (Object::Wall, _) => return None,
                    _ => {frontier.insert(next_point);}
                } 
            }

            moves.extend(&next);
            std::mem::swap(&mut next, &mut frontier);
            frontier.clear();
        }
        Some(moves)
    }

    pub fn push(&mut self, dir:&Direction){
        let point = self.bot;
        if let Some(mut moves) = self.get_moves(point, dir) {
            while let Some(point) = moves.pop(){
                let dest = point.step(dir);
                let source_obj = *self.get(&point);
                // Moving objects from leaf to source leave a space 
                // in their wake
                self.map[point.row][point.col] = Object::Space;
                self.map[dest.row][dest.col] = source_obj;
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
                Object::Box | Object::CrateLeft=> row  * 100 + col,
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
                    Object::Wall => [Object::Wall; 2],
                    Object::Box => [Object::CrateLeft, Object::CrateRight],
                    Object::Bot => {
                        bot_pos = Some(Point{row:row, col:col*2});
                        [Object::Bot, Object::Space]
                    }
                    _ => [Object::Space;2]
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