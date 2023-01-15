use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::ops::Sub;
use std::cmp::max;

static DATA: &str = "data.txt";

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn new(start:i64, end:i64) -> Interval { Interval { start, end }}
    fn len(&self) -> u64 {(self.end - self.start) as u64}
    fn merge(self, other:Self) -> (Option<Self>, Option<Self>) {
        // assumes self.start < other.start
        if other.start <= self.end {
            if other.end < self.end {
                (Some(self), None)
            } else {
                (Some(Self::new(self.start, other.end)), None)
            }
        } else {
            (Some(self), Some(other))
        }
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    reach: u64,
}

impl Sensor {
    fn new(pos: Point, beacon: Point) -> Sensor {
        let reach = pos.dist(&beacon);
        Sensor { pos, reach }
    }
    fn coverage_at_row(&self, row: i64) -> Option<Interval> {
        // The interval of columns covered at row
        let y_dist = (self.pos.y - row).abs() as u64;
        if y_dist > self.reach {
            None
        } else {
            let h_dist = (self.reach - y_dist) as i64;
            Some(Interval::new(self.pos.x - h_dist, self.pos.x + h_dist))
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new(x: i64, y:i64) -> Point {
        Point {x ,y}
    }

    fn partition_points(tl: Point, br: Point) -> [(Point, Point); 4] {
        // Give Top Left (tl) and Bottom Right (br) of square,
        // return the four quadrants.
        let mid = tl.midpoint(&br);
        [
            (Point::new(tl.x, mid.y+1), Point::new(mid.x, br.y)),
            (Point::new(mid.x+1, tl.y), Point::new(br.x, mid.y)),
            (Point::new(mid.x+1, mid.y+1), br),
            (tl, mid)
        ]
    }

    fn dist(&self, other: &Self) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u64
    }

    fn midpoint(&self, other:&Self) -> Self {
        // Centerpoint of square defined by corners
        Point::new((self.x + other.x ) / 2, (self.y + other.y) / 2)
    }

    fn clamp_dist(&self, other:&Self) -> u64 {
        (max(self.x - other.x, 0) + max(self.y - other.y, 0)) as u64
    }
}

impl Sub for &Point {
    type Output = Point;
    fn sub(self, other: Self) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

fn parse_file(p: &Path) -> Vec<Sensor> {
    let f = File::open(p).expect("no sensor data!");
    let buffer = BufReader::new(f);

    let re = Regex::new(r"(-?\d+)").unwrap();

    buffer.lines()
        .map(|line| {
            let l = line.unwrap();

            let coords = re.find_iter(&l)
            .flat_map(|digits| digits.as_str().parse().ok())
            .collect::<Vec<i64>>();

            let sensor = Point::new(coords[0], coords[1]);
            let beacon = Point::new(coords[2], coords[3]);

            Sensor::new(sensor, beacon)
            })
        .collect()
}

fn main() {
    let p = Path::new(DATA);
    let sensors = parse_file(p);

    /*
        Part One
        A lot of this is wasted work since the coverage has no holes
        We could just take the minimum start and maximum end...
    */

    let y = 2000000;

    let mut ranges:Vec<Interval> = sensors
        .iter()
        .flat_map(|p| p.coverage_at_row(y))
        .collect();

    ranges.sort_by_key(|r| r.start);


    let mut ranges = ranges.into_iter();
    let mut current = ranges.next().unwrap();

    let mut results:Vec<Interval> = vec![];

    for r in ranges {
        match current.merge(r) {
            (Some(c), None) => current = c,
            (Some(prev), Some(c)) => {
                current = c;
                results.push(prev)
            },
            _ => ()
        }
    }
    results.push(current);

    let part_one:u64 = results.iter().map(|i| i.len()).sum();
    println!("Part One: {:?}", part_one);


    /*
        Part Two
        Partitions into smaller quadrants and eliminate based on distances.
    */
    let side = 4000000;

    let mut stack = vec![(
        Point::new(0, 0),
        Point::new(side, side)
    )];

    while let Some((top_left, bottom_right)) = stack.pop() {
        if top_left == bottom_right {
            println!("Part Two: {}", top_left.x * side + top_left.y);
            break
        }

        stack.extend(Point::partition_points(top_left, bottom_right)
            .into_iter()
            .filter(|(tl, br)| {
                sensors
                .iter()
                .all(|Sensor {pos, reach}| {
                    let tl_dist = pos.clamp_dist(tl);
                    let br_dist = br.clamp_dist(pos);
                    *reach < tl_dist + br_dist
                })
            })
        )
    }

}
