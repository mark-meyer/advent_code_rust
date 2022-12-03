use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};


static PATH: &'static str = "./data.txt";

fn read_integers<R: Read>(input: R) -> Vec<usize> {
    let in_data = BufReader::new(input);
    in_data.lines()
        .map(|line| line.unwrap().parse().expect("The elves could not parse the integers!"))
        .collect()
}

fn problem_one(nums:&Vec<usize>) -> usize {
    nums.windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count()
}

fn problem_two(nums:&Vec<usize>) -> usize {
    let totals:Vec<usize> = nums.windows(3).map(|s| s.iter().sum()).collect();
    totals.windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count()}


fn main(){
    let path = Path::new(PATH);
    let file = File::open(&path).expect("Error opening file, Merry X-Mas!");

    let data = read_integers(file);
    
    let solution1 = problem_one(&data);
    println!("solution one: {}", solution1); 

    let solution2 = problem_two(&data);
    println!("solution two: {}", solution2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn p_one(){
        let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(problem_one(&v), 7);
    }
#[test]
    fn p_two(){
        let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(problem_two(&v), 5);
    }
}
