use std::collections::HashMap;
use std::fs;
use std::error::Error; 


fn parse(s: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    s
    .split_whitespace()
    .map(|n| n.parse().map_err(|_| "Stones with non numbers".into()))
    .collect()
} 

fn blink(counts: &HashMap<u64, u64>, blinks: u32) -> HashMap<u64, u64> {
    let mut current = counts.clone();
    let mut update_counts = HashMap::new();

    for _ in 0..blinks {
        update_counts.clear();
        for (stone, count) in &current {
            match stone {
                n if *n == 0 => {
                    *update_counts.entry(1).or_default() += count;
                }
                n if (n.ilog10() + 1) % 2 == 0 => {
                    let (l, r) = split_n(*n);
                    *update_counts.entry(l).or_default() += count;
                    *update_counts.entry(r).or_default() += count;
                }
                n => {
                    *update_counts.entry(n * 2024).or_default() += count;
                }
            }
        }
        std::mem::swap(&mut current, &mut update_counts); 
    }
    current
}


fn split_n(n: u64) -> (u64, u64) {
    let d = (n.ilog10() + 1) / 2;
    let p = 10_u64.pow(d);
    let right = n % p;
    let left = n / p;
    (left,right)
}

fn main() {
    let s = fs::read_to_string("data.txt").expect("the stones blinked out of existence");
    let stones = parse(&s).unwrap();

    let mut counts:HashMap<u64, u64> = HashMap::new();
    for n in stones {
        *counts.entry(n).or_default() += 1
    }

    let h = blink(&counts, 25);
    println!("Part One: {:?}", h.values().sum::<u64>());

    let h = blink(&counts, 75);
    println!("Part Two: {:?}", h.values().sum::<u64>());
}
