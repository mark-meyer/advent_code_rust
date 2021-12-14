
/// Parse input data into Points and tuples of char, int like: ('x', 7)
pub fn parse_data(data: &str) -> (Vec<Point>, Vec<(char, usize)>) {
    let (points, flips) = data.split_once("\n\n").unwrap();

    // Make list of (x, y) Points
    let points = points.lines()
        .map(|line| line.split(','))
        .map(|s| s.into()).collect();

    // Make list of ('x|y', n) flips
    let flips = flips.lines()
    .map(|line| line[11..].split("="))
    .map(|mut split| (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap()
    )).collect();

    (points, flips)
}

/// Convert a list of points into a list of strings
pub fn make_strings(points:&Vec<Point>) -> Vec<String>{
    let x_max = points.iter().map(|p| p.x).max().unwrap() + 1;
    let y_max = points.iter().map(|p| p.y).max().unwrap() + 1;

    let mut s = vec![' '; x_max * y_max];
    for p in points {
        s[(x_max * p.y) + p.x] = '⧯'
    }
    s.chunks(x_max)
    .take(y_max)
    .map(|c| c.into_iter().collect())
    .collect()
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    // To move a point we don't need to consider where it sits on paper
    // or where the fold is. If the coordinate x, y is greater than n
    // just alter the point to make it less than n by the same distance.

    pub fn transform_on_axis(self, fold: &(char,usize)) -> Self {
        let (axis, n) = fold;
        match axis {
            'x' if self.x > *n => Point{x: n - (self.x - n), y: self.y},
            'y' if self.y > *n => Point{x: self.x, y: n - (self.y - n)},
            _ => self
        }
    }
}

impl<'a, I:Iterator<Item=&'a str>> From<I> for Point {
    fn from(mut pair: I) -> Self {
        Self {
            x: pair.next().unwrap().parse().unwrap(),
            y: pair.next().unwrap().parse().unwrap()
        }
    }
}

