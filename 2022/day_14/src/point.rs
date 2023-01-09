use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {x: x, y: y}
    }

    fn line(&self, other: &Point) -> Vec<Point> {
        let (to, from) = if self < other {
            (self, other)
        } else {
            (other, self)
        };
        if to.x == from.x {
            (to.y..=from.y).map(| y | Point::new(to.x, y)).collect()
        } else {
            (to.x..=from.x).map(| x | Point::new(x, to.y)).collect()
        }
    }

    pub fn parse_line(s: &str) -> HashSet<Point> {
        let end_points = s.split(" -> ")
        .map(|s| s.into())
        .collect::<Vec<Point>>();
    
        end_points.windows(2)
        .flat_map(|p| p[0].line(&p[1]))
        .collect()
    }

    pub fn neighbors(&self) -> impl Iterator<Item=Point> {
        [ 
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x + 1, self.y + 1)
        ].into_iter()
    }

}

impl From<&str> for Point {
    fn from(s: &str) -> Point {
        let (x, y) = s.split_once(',').unwrap();
        Point {x: x.parse().unwrap(), y: y.parse().unwrap()}
    }
}

