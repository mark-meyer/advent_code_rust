use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};
use std::num::ParseIntError;

static FILE: &str = "data.txt";

type Stacks = Vec<Vec<char>>;

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize
}

impl Instruction {
    fn from_line(s: &str) -> Result<Instruction, ParseIntError> {
        let splits:Vec<&str> = s.splitn(6, ' ').collect();
        Ok(Instruction {
            count: splits[1].parse()?,
            from: splits[3].parse()?,
            to: splits[5].parse()?,
        })
    }

    fn move_at_a_time(&self, stacks: &mut Stacks) {
        for _ in 0..self.count {
            if let Some(el) = stacks[self.from - 1].pop() {
                stacks[self.to - 1].push(el);
            }
        }
    }
    fn move_group(&self, stacks: &mut Stacks) {
        let idx = stacks[self.from - 1].len() - self.count;
        let mut moving:Vec<char> = stacks[self.from - 1].drain(idx..).collect();
        stacks[self.to - 1].append(&mut moving);
    }
}

fn get_chars(line: &str) -> Vec<char> {
    let mut it = line.as_bytes().iter();
    it.next();
    it.step_by(4).map(|b| *b as char).collect()
}

fn get_stacks(lines: &mut Lines<BufReader<File>>) -> Stacks{
    let mut stacks:Vec<_> = lines
    .take_while(|line| line.as_ref().unwrap() != "")
    .map(|line| get_chars(&line.unwrap())).collect();
    stacks.pop();
    stacks.reverse();
    
    // Transpose stacks removing empties
    let mut transposed:Stacks= vec![vec![]; stacks[0].len()];
    for row in stacks {
        for (i, c) in row.iter().enumerate(){
            if *c != ' ' {
                transposed[i].push(*c)
            }
        }
    }
    transposed
}

fn main() {
    let path = Path::new(FILE);
    let file = File::open(path).expect("could not open the file");
    let mut lines = BufReader::new(file).lines();
    let mut stacks_1:Vec<_> = get_stacks(&mut lines);
    let mut stacks_2 = stacks_1.clone();

    for line in lines {
        let instruction = Instruction::from_line(&line.unwrap()).expect("Bad instruction");
        instruction.move_at_a_time(&mut stacks_1);
        instruction.move_group(&mut stacks_2);
    }

    let top:String = stacks_1.iter().map(|s| *s.last().unwrap()).collect();
    println!("Part One: {}", top);

    let top:String = stacks_2.iter().map(|s| *s.last().unwrap()).collect();
    println!("Part Two: {}", top);

}
