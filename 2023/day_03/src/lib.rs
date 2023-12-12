use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Symbol {
    value: char,
    row: usize,
    col: usize
}
pub struct Bounds {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize
}

pub fn parse_lines<I, S>(lines: I) -> Vec<String> 
    where 
    I: Iterator<Item=S>,
    S: AsRef<str>
{
    lines
    .map(|line| line.as_ref().to_owned())
    .collect()
}
fn find_symbols_in_bounds(matrix: &Vec<&str>, b: &Bounds) -> Vec<Symbol>{
    let mut symbols:Vec<Symbol> = Vec::new();

    for (row, line) in matrix[b.y0..b.y1].iter().enumerate(){
        for (col, c) in line[b.x0..b.x1].char_indices(){
            if !c.is_ascii_digit() && c!='.' {
                symbols.push(Symbol { value: c, row: row+b.y0, col: col + b.x0 })  
            }
        }
    }
    symbols
}

pub fn find_symbols(matrix: &Vec<&str>) -> HashMap<Symbol, Vec<u32>>{
    let mut symbol_lookup = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();

    for (row, line) in matrix.iter().enumerate() {
        for matches in re.find_iter(line) {
            let bounds = Bounds {
                y0: row.saturating_sub(1),
                x0: matches.start().saturating_sub(1),
                y1: (row + 2).min(matrix.len()),
                x1: (matches.end() + 1).min(line.len())
            };
            let symbols = find_symbols_in_bounds(matrix, &bounds);
            for symbol in symbols {
                symbol_lookup.entry(symbol)
                    .or_insert_with(Vec::new)
                    .push(matches.as_str().parse::<u32>().unwrap())
            }
        }
        
    }
    return symbol_lookup
}

pub fn part_one(data: &Vec<&str>) -> u32 {
    let symbols = find_symbols(&data);
    symbols.values().map(|v| v.iter().sum::<u32>()).sum()
}

pub fn part_two(data: &Vec<&str>) -> u32 {
    let symbols = find_symbols(&data);
    symbols.values().filter_map(|v| {
        match v.len() {
            2 => Some(v.iter().product::<u32>()),
            _ => None
        }
    }).sum()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_parse(){
        let lines = vec![
            "467..114..",
            "...*......"
        ];
        let parsed = parse_lines(lines.iter());
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], "467..114..");
        assert_eq!(parsed[1], "...*......");
    }

    #[test]
    fn test_find_symbols_in_bounds() {
        let lines = vec![
            "46*..114..",
            "...*......",
            "........*."
        ];
        let bounds = Bounds { x0:1, y0:0, x1:4,y1:3};

        let s = find_symbols_in_bounds(&lines, &bounds);
        assert_eq!(s.len(), 2);
        assert_eq!(s[0].row, 0);
        assert_eq!(s[0].col, 2);
        assert_eq!(s[1].row, 1);
        assert_eq!(s[1].col, 3);
    }
}