use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

type RuleSet = HashMap<usize,HashSet<usize>>;

fn parse(f:File) -> Result<(RuleSet, Vec<Vec<usize>>), std::io::Error> {  
    let mut lines = BufReader::new(f).lines();
    let mut rules:RuleSet = HashMap::new();
    
    for line_result in lines.by_ref().take_while(|res| {
        if let Ok(line) = res {
            !line.is_empty()
        } else {
            false
        }

    }) {
        let line = line_result?;
        let (k, v) = line.split_once('|').unwrap();
        let k:usize = k.parse().unwrap();
        let v:usize = v.parse().unwrap();
        rules.entry(k).or_default().insert(v);
    };

    let mut updates:Vec<Vec<usize>> = Vec::new();
    
    for line_result in lines {
        let line = line_result?;
        let element:Vec<usize> = line.split(',').map(|n| n.parse::<usize>().unwrap()).collect();
        updates.push(element);
    }
    Ok((rules, updates))
}

fn is_valid(rules:&RuleSet, update:&[usize]) -> bool {
    for (i, n) in update.iter().enumerate() {
        for m in update.iter().skip(i) {
            if let Some(must_follow) = rules.get(&m) {
                if must_follow.contains(&n) {
                    return false
                }
            }
        }
    }
    true
}

fn get_middle(update:&[usize]) -> usize {
    update[update.len() / 2]
}

fn part_one(rules:&RuleSet, updates:&Vec<Vec<usize>>) -> usize {
    updates
    .iter()
    .filter(|update| is_valid(rules, update))
    .map(|update| get_middle(update))
    .sum()
}

fn part_two(rules:&RuleSet, updates:&Vec<Vec<usize>>) -> usize {
    updates
    .iter()
    .filter(|update|!is_valid(rules, update))
    .map(|update| {
        let mid = update.len() / 2;
        let mut sorted = update.clone();
        sorted.select_nth_unstable_by(mid, |a, b| match rules.get(a) {
            Some(must_follow) if must_follow.contains(b) => Ordering::Less,
            _ => Ordering::Equal
        });
        sorted[mid]
    }).sum()
}

fn main() {
    let p = Path::new("data.txt");
    let f = File::open(p).expect("Couln't open file!");

    let (rules, updates) = parse(f).unwrap();
    
    println!("Part One: {:?}", part_one(&rules, &updates));
    println!("Part One: {:?}", part_two(&rules, &updates));


}