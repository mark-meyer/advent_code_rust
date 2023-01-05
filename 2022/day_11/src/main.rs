use std::fs;
use std::path::Path;
use std::cell::RefCell;

static DATA:&str = "data.txt";

struct Monkey {
    items: Vec<u64>,
    inspect_count: u64,
    modulus: u64,
    div: u64, // this is just to allow us to turn it off for part two.
    monkey_true_i: usize,
    monkey_false_i: usize,
    op: Box<dyn Fn(u64) -> u64>
}

impl Iterator for Monkey {
    type Item =  (usize, u64);
    fn next(&mut self) ->  Option<Self::Item> {
        match self.items.pop() {
            Some(n) => {
                self.inspect_count += 1;
                let val = (self.op)(n) / self.div;
                if val % self.modulus ==  0 {
                    Some((self.monkey_true_i, val))
                } else {
                    Some((self.monkey_false_i, val))
                }
            }
            None => None
        }
    }
}

fn parse_operation(s: &str) -> Box<dyn Fn(u64) -> u64> {
    let (_, op) = s.split_once("= ").expect("Bad Op");
    let components:Vec<_> = op.split_whitespace().collect();

    if let Ok(n) = components[2].parse::<u64>() {
        match components[1] {
            "*" =>  Box::new(move |old| old * n),
            "+" =>  Box::new(move |old| old + n),
            _ => panic!("bad op")
        }
    } else {
        match components[1] {
            "*" =>  Box::new(|old| old * old),
            "+" =>  Box::new(|old| old + old),
            _ => panic!("bad op")
        }
    }
}

fn parse_monkey(s: &str) -> Monkey {
    let mut lines = s.split('\n');
    lines.next();
    let (_, nums) = lines.next().unwrap().split_once(": ").expect("Bad Monkey");
    let items:Vec<u64> = nums.split(", ").flat_map(|n| n.parse()).collect();
    
    let op = parse_operation(lines.next().unwrap());
    let (_, div) = lines.next().unwrap().rsplit_once(" ").expect("Bad Monkey");
    let div = div.parse::<u64>().expect("Bad modulus");

    let (_, i) = lines.next().unwrap().rsplit_once(" ").expect("Bad Monkey");
    let true_i = i.parse::<usize>().expect("Bad number");

    let (_, i) = lines.next().unwrap().rsplit_once(" ").expect("Bad Monkey");
    let false_i = i.parse::<usize>().expect("Bad number");

    Monkey{
        items: items,
        inspect_count: 0,
        modulus: div,
        div: 3,
        monkey_true_i: true_i,
        monkey_false_i: false_i,
        op: op
    }
}

fn top_2(v: &Vec<RefCell<Monkey>>) ->(u64, u64) {
    v.iter().map(|m| m.borrow().inspect_count).fold((0, 0), |t, c| {
        match c {
            c if c >= t.0 => (c, t.0),
            c if c > t.1 => (t.0, c),
            _ => t
        }
    })
}
fn main() {
    let path = Path::new(DATA);
    let text = fs::read_to_string(path).expect("can't read the file!");
    
    /*
         Part One
    */
    let monkeys:Vec<RefCell<Monkey>> = text.split("\n\n").map(|l| RefCell::new(parse_monkey(l))).collect();
 
    for _ in 0..20 {
        for monkey in &monkeys {
            let mut m = monkey.borrow_mut();
            while let Some((to, val)) = m.next() {
                let mut m2 = monkeys[to].borrow_mut();
                m2.items.push(val)
            }        
        }
    }

    let top:(u64, u64) = top_2(&monkeys);

    println!("Part One: {}", top.0 * top.1 );

    /* 
        Part Two
    */
    let monkeys:Vec<RefCell<Monkey>> = text.split("\n\n").map(|l| RefCell::new(parse_monkey(l))).collect();
    let modulus:u64 = monkeys.iter().map(|m| m.borrow().modulus).product();
    
    for _ in 0..10000 {
        for monkey in &monkeys {
            let mut m = monkey.borrow_mut();
            m.div = 1;
            while let Some((to, val)) = m.next() {
                let mut m2 = monkeys[to].borrow_mut();
                m2.items.push(val % modulus)
            }        
        }
    }
    
    let top:(u64, u64) = top_2(&monkeys);
    println!("Part Two: {}", top.0 * top.1 );
    
}
