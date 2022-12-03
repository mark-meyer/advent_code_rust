use std::fs;
use std::path::Path;
use std::collections::HashMap;

static PATH: &'static str = "./data.txt";

#[derive(Debug)]
struct Decoder {
    key:String,
    wire_guide: HashMap<char, u16>,
    digits:  HashMap<u16, char>
}

impl Decoder {
    fn new(key:String) -> Self {
        let wire_map = Self::make_wire_map(&key);
        Decoder {
            key: key,
            wire_guide: wire_map,
            digits: HashMap::from([
                (0b1110111, '0'),
                (0b0010010, '1'),
                (0b1011101, '2'),
                (0b1011011, '3'),
                (0b0111010, '4'),
                (0b1101011, '5'),
                (0b1101111, '6'),
                (0b1010010, '7'),
                (0b1111111, '8'),
                (0b1111011, '9')
            ])
        }
    }

    fn word_to_digit(&self, word:&str ) -> char {
        let total = word.chars().fold(0, |a, c| a + self.wire_guide.get(&c).unwrap());
        *self.digits.get(&total).unwrap()
    }

    fn decode_words(&self, words: &str) -> i32 {
        let digits:Vec<char> = words.split_whitespace().map(|w| self.word_to_digit(w)).collect();
        let digits:String = digits.iter().collect();
        digits.parse().unwrap()
    }

    fn reversed_counts(key: &str) -> HashMap<u16, Vec<char>> {
        let mut counts:HashMap<char, u16> = HashMap::new();
        for c in key.chars().filter(|&l| l != ' '){
            let counter = counts.entry(c).or_insert(0);
            *counter += 1;
        }

        let mut rev_counts:HashMap<u16, Vec<char>> = HashMap::new();
        for (k, n) in counts.into_iter() {
            let arr = rev_counts.entry(n).or_insert(Vec::new());
            arr.push(k);
        }
        rev_counts
    }
    
    fn make_wire_map(key: &str) -> HashMap<char, u16> {
        let counts = Self::reversed_counts(key);
        let items:Vec<&str> = key.split_whitespace().collect();
        let counts_no_four:String = items
            .into_iter()
            .filter(|&s| s.len() != 4)
            .collect::<Vec<&str>>()
            .join("");
        
        let counts_no_four = Self::reversed_counts(&counts_no_four);

        let mut wire_map_reversed:HashMap<char, u16> = HashMap::new();

        wire_map_reversed.insert(counts.get(&6).unwrap()[0], 32);
        wire_map_reversed.insert(counts_no_four.get(&6).unwrap()[0], 8);
        wire_map_reversed.insert(counts.get(&4).unwrap()[0], 4);
        let f = counts.get(&9).unwrap()[0];
        wire_map_reversed.insert(f, 2);
        
        let a = counts_no_four.get(&8).unwrap().iter().filter(|&&n| n != f).next().unwrap();
        wire_map_reversed.insert(*a,64);
        let c = counts.get(&8).unwrap().iter().filter(|&n| n != a).next().unwrap();
        wire_map_reversed.insert(*c, 16);
        let g = counts_no_four.get(&7).unwrap().iter().filter(|&n| n != c).next().unwrap();
        wire_map_reversed.insert(*g, 1);

        wire_map_reversed
    }
}

fn parse_line(line: &str) -> (&str, &str) {
    let mut splits = line.trim().splitn(2, " | ");
    let first = splits.next().unwrap();
    let second = splits.next().unwrap();
    (first, second)
}

fn parse_data(data: &str) -> Vec<(&str, &str)> {
    data.lines().map(|line| parse_line(line)).collect()
}

fn solution_two(data: &Vec<(&str, &str)>) -> i32 {
    let mut sum  = 0;
    for (key, cypher) in data {
        let decoder = Decoder::new(key.to_string());
        let num = decoder.decode_words(cypher);
        sum += num;
    }
    sum
}

fn main() {
    let path = Path::new(PATH);
    let data = fs::read_to_string(path).expect("Could not read file, maybe wires are crossed");
    let data = parse_data(&data);

    let solution2 = solution_two(&data);
    println!("solution 2: {}", solution2);
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_line(){
        let line = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let data = parse_line(line);
        assert_eq!(data, ("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab", "cdfeb fcadb cdfeb cdbaf"));
    }
    #[test]
    fn test_wires(){
        let line = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let data = parse_line(line);
        let decoder = Decoder::new(data.0.to_string());
        assert_eq!(decoder.word_to_digit("acedgfb"), '8');
        assert_eq!(decoder.word_to_digit("cdfbe"), '5');
        assert_eq!(decoder.word_to_digit("gcdfa"), '2');
        assert_eq!(decoder.word_to_digit("fbcad"), '3');
        assert_eq!(decoder.word_to_digit("dab"), '7');
        assert_eq!(decoder.word_to_digit("cefabd"), '9');
        assert_eq!(decoder.word_to_digit("cdfgeb"), '6');
        assert_eq!(decoder.word_to_digit("eafb"), '4');
        assert_eq!(decoder.word_to_digit("cagedb"), '0');
        assert_eq!(decoder.word_to_digit("ab"), '1');
    }

    #[test]
    fn test_word(){
        let line = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let data = parse_line(line);
        let decoder = Decoder::new(data.0.to_string());
        let res = decoder.decode_words(data.1);
        assert_eq!(5353, res)
    }
}
