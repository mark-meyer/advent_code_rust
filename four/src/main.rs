extern crate num;

use std::path::Path;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use num::Integer;

static PATH: &'static str = "./data.txt";

#[derive(Debug, PartialEq, Clone)]
enum Square<T:Integer + Copy> {
    Open(T),
    Marked
}

#[derive(Debug, Clone)]
struct Board<T:Integer + Copy>  {
    squares:Vec<Square<T>>
}

impl<T:Integer + Copy> Board<T> {
    fn new(data: Vec<T>) -> Self {
        Board {
            squares: data.into_iter().map(|n| Square::Open(n)).collect()
        }
    }
    fn play(&mut self, n:T) {
        for square in self.squares.iter_mut() {
            if let Square::Open(m) = square {
                if *m == n {
                    *square = Square::Marked;
                }
            }          
        }
    }
    fn winning(&self) -> bool {
        // where is my zip(*)??
        let col = (0..5)
            .any(|col| self.squares.iter().skip(col).step_by(5).all(|square| matches!(square, Square::Marked)));
        let rows = self.squares.chunks(5)
            .any(|row| row.iter().all(|square| matches!(square, Square::Marked)));
        
        rows || col
    }
    fn sum_unmarked(&self) -> T {
        let mut sum = T::zero();
        for square in &self.squares {
            if let Square::Open(n) = square {
                sum = sum + *n;
            }
        }
        sum
    }
} 

fn get_data<R: Read>(input: R) -> (Vec<i32>, Vec<Board<i32>>) {
    let mut reader = BufReader::new(input);
    let mut numbers = String::new();    
    reader.read_line(&mut numbers).expect("Number hopper is broken");

    let numbers:Vec<i32> = numbers
        .split(",")
        .map(|n| n.trim().parse().expect("can't parse integers?!"))
        .collect();

    let mut boards:Vec<Board<i32>> = Vec::new();
    let mut raw_numbers: Vec<i32> = Vec::new();

    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        if line.is_empty() {
            boards.push(Board::new(raw_numbers));
            raw_numbers = Vec::new(); 
            continue;
        }
        for num in line.trim().split_whitespace() {
            raw_numbers.push(num.parse().unwrap());
        }
    }

    (numbers, boards)
}

fn part_one(numbers:&Vec<i32>, boards:&mut Vec<Board<i32>>) -> i32{
    for n in numbers {
        for board in boards.iter_mut() {
            board.play(*n);
            if board.winning() {
                return board.sum_unmarked() * n
            }
        }
    }
    panic!("No winning boards");
}

fn part_two(numbers:&Vec<i32>, boards:&mut Vec<Board<i32>>) -> i32 {
    for n in numbers {
        if boards.len() == 1 {
            boards[0].play(*n);
            if boards[0].winning() {
                return boards[0].sum_unmarked() * n
            }
        } else {
            for board in boards.iter_mut() {
                board.play(*n);            
            }
        }
        
        boards.retain(|board| {
            !board.winning()
        });
        
    }
    panic!("No winning boards");
}


fn main() {
    let path = Path::new(PATH);
    let file = File::open(path).expect("Error opening file, no bingo for you!");
    let (numbers, boards) = get_data(file);
    let solution_one = part_one(&numbers, &mut boards.clone());
    println!("Solution 1: {}", solution_one);
    let solution_two = part_two(&numbers, &mut boards.clone());
    println!("Solution 2: {}", solution_two);

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let b:Board<i32> = Board::new(vec![22,13,17,11,0,8,2,23,4,24,21,9,14,16,7,6,10,3,18,5,1,12,20,15,19]);
        assert_eq!(b.squares[0], Square::Open(22));
    }
    #[test]
    fn play() {
        let mut b:Board<i32> = Board::new(vec![22,13,17,11,0,8,2,23,4,24,21,9,14,16,7,6,10,3,18,5,1,12,20,15,19]);
        b.play(17);
        b.play(19);
        assert_eq!(b.squares[2], Square::Marked);
        assert_eq!(b.squares[24], Square::Marked);
    }
    #[test]
    fn win_row() {
        let mut b:Board<i32> = Board::new(vec![22,13,17,11,0,8,2,23,4,24,21,9,14,16,7,6,10,3,18,5,1,12,20,15,19]);
        assert_eq!(b.winning(), false);

        b.play(21);
        b.play(9);
        b.play(14);
        assert_eq!(b.winning(), false);

        b.play(16);
        b.play(7);
        assert_eq!(b.winning(), true);
    }
    #[test]
    fn win_col() {
        let mut b:Board<i32> = Board::new(vec![22,13,17,11,0,8,2,23,4,24,21,9,14,16,7,6,10,3,18,5,1,12,20,15,19]);
        assert_eq!(b.winning(), false);

        b.play(13);
        b.play(2);
        b.play(9);
        assert_eq!(b.winning(), false);

        b.play(10);
        b.play(12);
        assert_eq!(b.winning(), true);
    }
    #[test]
    fn sum_unmarked() {
        let mut b:Board<i32> = Board::new(vec![22,13,17,11,0,8,2,23,4,24,21,9,14,16,7,6,10,3,18,5,1,12,20,15,19]);
        assert_eq!(b.sum_unmarked(), 300);
        b.play(10);
        assert_eq!(b.sum_unmarked(), 290);
    }
}