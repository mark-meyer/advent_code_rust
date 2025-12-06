use std::fs::read_to_string;

#[derive(Debug)]
enum Operator {
    Sum,
    Product
}

impl From<&str> for Operator {
   fn from(op: &str) -> Operator {
        match op {
            "+" => Operator::Sum,
            "*" => Operator::Product,
            _ => panic!("Non-cephalopod operator found!")
        }
   } 
}

fn parse_file_two(raw_data: &str) -> Vec<(Operator, Vec<u64>)>{
    let lines: Vec<Vec<char>> = raw_data.split("\n").map(|l| l.chars().collect()).collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let ops = lines[rows-1].iter().filter(|c| !c.is_ascii_whitespace());

    let mut n_gen = (0..cols).map(|col| {
        (0..rows-1)
        .map(|row| lines[row][col])
        .filter(|n| n.is_ascii_digit())
        .collect::<String>()
    });

    ops.map(|op| {
        let operator = Operator::from(op.to_string().as_str());

        let nums:Vec<u64> = n_gen.by_ref()
            .take_while(|s| s != "")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        
        (operator, nums)
    })
    .collect()
}

fn parse_file_one(raw_data: &str) -> Vec<(Operator, Vec<u64>)> {
    let lines: Vec<Vec<&str>> = raw_data.split("\n")
    .map(|line| line.split_ascii_whitespace().collect())
    .collect();

    let rows = lines.len();
    let cols = lines[0].len();
    
    let mut n_gen = (0..cols).map(|col| {
        (0..rows-1)
        .map(|row| lines[row][col].parse().unwrap()).collect()
    });
    let ops = &lines[rows-1];

    ops.into_iter().map(|&op| {
        let operator = Operator::from(op);
        (operator, n_gen.next().unwrap())
    })
    .collect()
}

fn cephalopod_math(input: Vec<(Operator,  Vec<u64>)>) -> u64 {
    input.iter()
    .map(|(operator, nums)| match operator {
        Operator::Sum => nums.iter().sum::<u64>(),
        Operator::Product => nums.iter().product(),
    })
    .sum()
}

fn main() {
    let s = read_to_string("data.txt").expect("could not open file");
    
    let answer_one = cephalopod_math(parse_file_one(&s));
    println!("Part One: {}", answer_one);

    let answer_two = cephalopod_math(parse_file_two(&s));
    println!("Part One: {}", answer_two);
}