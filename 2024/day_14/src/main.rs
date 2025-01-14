use std::fs;
use regex::Regex;
use day_14::*;

#[derive(Debug, Clone, Copy)]
pub struct Bot {
    pub pos: (isize, isize),
    pub vel: (isize, isize)
}

impl Bot {
    pub fn calc_pos(&self, time:isize, max_row:isize, max_col:isize) -> Bot{
        Bot {
            pos:  (
                (self.pos.0 + self.vel.0 * time).rem_euclid(max_row),
                (self.pos.1 + self.vel.1 * time).rem_euclid(max_col)
            ), ..*self
        }
     } 
     pub fn quadrant(&self, w:isize, h:isize) -> usize {
        match ((self.pos.0 < w /2), (self.pos.1 < h/2)) {
            (true, true) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (false, false) => 3
        }
     }
}

fn find_image_index(bots: &Vec<Bot>, w:isize, h:isize) -> Option<(f32, isize)> {
    // Find the standard deviation of the bot's coordinates.
    // The assumption is that the bots will be clustered in 
    // any sort of image to a greater degree that random placement.
    (1..10000).map(|time| {
        let positions:Vec<isize> = bots.iter()
            .map(|bot| bot.calc_pos(time, w, h))
            .flat_map(|Bot{pos:(x,y),vel: _}| [x , y])
            .collect();

        (std_deviation(&positions).unwrap(), time)
    }).min_by(|(a, _), (b, _)| a.total_cmp(b))
    
}

fn parse(s:String) -> Vec<Bot> {
    let rx = Regex::new(r".+?(\d+),(\d+).+?([-]?\d+),([-]?\d+)").unwrap();
    rx.captures_iter(&s)
    .map(|m| m.extract())
    .map(|(_, points)| points.map(|p| p.parse().unwrap()))
    .map(|[px,py,vx,vy]:[isize;4]| {
        Bot {
            pos: (px, py),
            vel: (vx, vy)
        }
    }).collect()
}

fn ascii_art_bots(bots:&Vec<Bot>) {
    let mut matrix =[[" "; 101]; 103];
    for Bot{pos:(x, y), vel:_} in bots {
        matrix[*y as usize][*x as usize] = "#"
    }
    for row in matrix{
        println!("{:?}", row.join(""));
    }
}

fn main() {
    let w = 101;
    let h = 103;
    let s = fs::read_to_string("data.txt").expect("couldn't open file");

    let found = parse(s);

    let mut quad_counts = [0; 4];

    found.iter()
    .map(|bot| bot.calc_pos(100, w, h))
    .filter(|bot| bot.pos.0 != w/2 && bot.pos.1 != h/2)
    .for_each(|b| quad_counts[b.quadrant(w, h)] += 1);
    
    println!("Part One: {:?} ",quad_counts.iter().product::<i32>());

    let (_, solution_time) = find_image_index(&found, w, h).unwrap();
    println!("Part Two: {}", solution_time);
    
    /* Print to the terminal! */
    let solution_bots:Vec<Bot> = found.iter()
    .map(|bot| bot.calc_pos(solution_time, w, h))
    .collect();

    ascii_art_bots(&solution_bots);
}
