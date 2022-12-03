use lazy_static::lazy_static;
use std::collections::LinkedList;
use std::collections::HashMap;
use rayon::prelude::*;

lazy_static! {
    static ref DELIM_MATCH: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        (')', '('),
        ('[', ']'),
        (']', '['),
        ('{', '}'),
        ('}', '{'),
        ('<', '>'),
        ('>', '<'),
    ]);

    static ref SCORES_1: HashMap<char, usize> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    static ref SCORES_2: HashMap<char, usize> = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4)
    ]);
}

fn remaining_open(line: &str) -> Option<usize> {
    let mut openings = LinkedList::<char>::new(); 
    for c in line.chars() {
        let found = match c {
            '(' | '[' | '{' | '<' 
                => { openings.push_back(c); None },
            _   => openings.pop_back().map_or(None, |open| {
                    if open != *DELIM_MATCH.get(&c).unwrap() { 
                        Some(c) 
                    } else { 
                        return None 
                    }
                })
        };
        if found.is_some() {
            return None
        }
    }
    let mut total = 0;
    while let Some(c) = openings.pop_back() {
        total = total * 5 + SCORES_2.get(&c).unwrap();
    }
    Some(total)
}

fn first_illegal_score(line: &str) -> Option<char> {
    let mut openings = LinkedList::<char>::new();
    for c in line.chars() {
        let found = match c {
            '(' | '[' | '{' | '<' 
                => { openings.push_back(c); None },
            _   => openings.pop_back().map_or(None, |open| {
                    if open != *DELIM_MATCH.get(&c).unwrap() { 
                        Some(c) 
                    } else { 
                        None 
                    }
                })
        };
        if found.is_some() {
            return found
        }
    }
    None
}

pub fn solution_two(s:&str) -> usize {
    let mut top:Vec<usize> = s.par_lines()
    .flat_map(|line| remaining_open(line))
    .collect();
    top.sort();

    top[top.len() / 2]
}

pub fn solution_one(s:&str) -> usize {
    s.par_lines()
    .flat_map(|s| first_illegal_score(s))
    .flat_map(|c| SCORES_1.get(&c))
    .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_first_illegal_score() {
        let illegal = first_illegal_score("{([(<{}[<>[]}>{[]{[(<()>");
        assert_eq!(illegal, Some('}'));
    }
    #[test]
    fn test_solution_one() {
        let s = "[({(<(())[]>[[{[]{<()<>>\n\
            [(()[<>])]({[<{<<[]>>(\n\
            {([(<{}[<>[]}>{[]{[(<()>\n\
            (((({<>}<{<{<>}{[]{[]{}\n\
            [[<[([]))<([[{}[[()]]]\n\
            [{[{({}]{}}([{[{{{}}([]\n\
            {<[[]]>}<{[{[{[]{()[[[]\n\
            [<(<(<(<{}))><([]([]()\n\
            <{([([[(<>()){}]>(<<{{\n\
            <{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(solution_one(s), 26397)
    }

    #[test]
    fn test_remaining_open() {
        let s = "[({(<(())[]>[[{[]{<()<>>";
        assert_eq!(remaining_open(&s), Some(288957));
    }
    #[test]
    fn test_remaining_open_valid() {
        let s = "[()()<{}>]";
        assert_eq!(remaining_open(&s), Some(0));
    }
    #[test]
    fn test_remaining_open_invalid() {
        let s = "[()()<{>]";
        assert_eq!(remaining_open(&s), None);
    }
}