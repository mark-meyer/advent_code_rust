
use std::io::{BufReader, Read, BufRead};
use std::path::Path;
use std::fs::File;

static PATH: &'static str = "./data.txt";

fn read_integers<R: Read>(input: R) -> Vec<usize> {
    let in_data = BufReader::new(input);
    in_data.lines()
        .map(|line| usize::from_str_radix(&line.unwrap(), 2).expect("The elves are having trouble with binary math!"))
        .collect()
}

fn solution1(data:&Vec<usize>, bits:usize) -> usize {
    let data_size = data.len();
    let most = (0..bits)
        .map(|n| 1 << n)
        .fold(0, |res, p| {
        if data.iter().filter(|n| *n & p == 0 ).count() >= data_size / 2 {res + p} else { res }
    });
    let least = (0..12)
        .map(|n| 1 << n)
        .fold(0, |res, p| {
        if data.iter().filter(|n| *n & p == 0 ).count() < data_size / 2 {res + p} else { res }
    });

    most * least
}

fn solution2(data:&Vec<usize>, bits:usize) -> usize {
    let mut filtered = data.clone();
    let mut p = bits ;
    while filtered.len() > 1 {
        p -= 1;
        let (zeros, ones):(Vec<usize>, Vec<usize>) = filtered.iter().partition(|n| *n & (1 << p) == 0);
        filtered = if zeros.len() > ones.len() {zeros} else {ones};
       
    }
    let oxygen = filtered[0];
    
    let mut filtered = data.clone();
    let mut p =  bits ;
    while filtered.len() > 1 {
        p -= 1;
        let (zeros, ones):(Vec<usize>, Vec<usize>) = filtered.iter().partition(|n| *n & (1 << p) == 0);
        filtered = if zeros.len() > ones.len() {ones} else {zeros};
    }
    let co2 = filtered[0];
    
    co2 * oxygen
}

fn main() {
    let path = Path::new(PATH);
    let file = File::open(&path).expect("Error opening file, submarine problems!");
    let data:Vec<usize> = read_integers(file);

    println!("{:?}", solution1(&data, 12));
    println!("{:?}", solution2(&data, 12));
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn solution_one(){
        let data:Vec<usize> = vec!(
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010
        );
        assert_eq!(solution1(&data, 5), 198);
    }
    #[test]
    fn solution_two(){
        let data:Vec<usize> = vec!(
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010
        );
        assert_eq!(solution2(&data, 5), 230);
    }
}