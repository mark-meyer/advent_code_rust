use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

static DATA: &str  = "data.txt";


fn main() {
    let p = Path::new(DATA);
    let f = File::open(p).expect("could not open the file!");
    let it = BufReader::new(f).lines().map(|line| line.unwrap());

    let mut ticks = vec![1];
    let mut x:i32 = 1;

    for line in it {
        match line.split_once(" ") {
            Some((_, n)) => {
                ticks.push(x);
                x += n.parse::<i32>().expect("Somthing went terribly wrong.");
                ticks.push(x);
            },
            None => {
                ticks.push(x)
            }
        }
    }

    let total:i32 = (20..=220)
        .step_by(40)
        .fold(0, |acc, i| acc + i as i32 * ticks[i-1]); 
      
    println!("Part one: {}", total);
    
    println!("Part two: ");
    for row_start in (0..=200).step_by(40) {
        let v:String = (0..40)
            .map(|i| {
                match (i as i32 - ticks[i + row_start]).abs() <= 1 {
                    true => '#',
                    false => '.'
                }
            })
            .collect();
        println!("{}", v);
    }

}

