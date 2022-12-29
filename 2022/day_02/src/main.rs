#![allow(unused_variables)]
use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead};
use self::Hand::*; 

const PATH: &str = "data.txt";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Hand{
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

struct Play {
    hand: Hand,
    beats: Hand,
    loses: Hand
}

static ROCK:Play = Play {hand: Rock, beats: Scissors, loses: Paper};
static PAPER:Play = Play {hand: Paper, beats:Rock, loses: Scissors};
static SCISSORS:Play = Play {hand: Scissors, beats: Paper, loses: Rock};


#[derive(Debug)]
struct ParsePlayError;

fn get_play (s: &str) -> Result<&Play, ParsePlayError> {
    match s {
        "A" | "X" => Ok(&ROCK),
        "B" | "Y"=> Ok(&PAPER),
        "C" | "Z" => Ok(&SCISSORS),
        _ => Err(ParsePlayError),
    }
}

fn part_one(file: fs::File) -> u32 {
    let mut total = 0;

    for line in BufReader::new(file).lines(){
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').unwrap();

        let player1 = get_play(a).unwrap();
        let player2 = get_play(b).unwrap();


        total += match &player1.hand {
            h if h == &player2.beats  => 6 ,
            h if h == &player2.loses => 0,
            _ => 3
        } + player2.hand as u32
    }
    return total
}

fn part_two(file: fs::File) -> u32 {
    let mut total = 0;
    for line in BufReader::new(file).lines(){
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').unwrap();
        let player1 = get_play(a).unwrap();

        let score = match b {
            "Z" => 6 + player1.loses as u32,
            "Y" => 3 + player1.hand as u32,
            _ => 0 + player1.beats as u32
        };
        total += score;
    }
    total
}

fn main() {
    let path = Path::new(PATH);
    let file = fs::File::open(path).expect("could not open file");
    println!("Part one: {}", part_one(file));

    let file = fs::File::open(path).expect("could not open file");
    println!("Part two: {}", part_two(file));
}
