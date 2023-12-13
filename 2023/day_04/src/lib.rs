use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    hand: Vec<u32>
}

#[derive(Debug)]
struct ConversionError;

impl From<&str>for Card {
    fn from(value: &str) -> Self {
        let components:Vec<&str> = value.split_whitespace().collect();
        
        let wins:HashSet<u32> = components.iter().skip(2).take_while(|&&s| s != "|").filter_map(|&n| n.parse().ok()).collect();
        let hand: Vec<u32> = components.iter().skip_while(|&&s| s != "|").filter_map(|&n| n.parse().ok()).collect();

       Card {
            winning_numbers: wins,
            hand: hand
        }
    }
}

impl Card {
    fn win_count(&self) -> u32 {
        self.hand.iter().filter(|n| self.winning_numbers.contains(n)).count() as u32
    }
}

pub fn solve_part_one(input: &[&str]) -> u32 {
    input.iter().map(|&line| u32::pow(2, Card::from(line).win_count()) / 2).sum()
}

pub fn solve_part_two(input: &[&str]) -> usize {
    let mut scores: Vec<usize> = vec![1; input.len()];
    let wins: Vec<usize> = input.iter().map(|&line| Card::from(line).win_count() as usize).collect();

    for (idx, w) in wins.iter().enumerate() {
        let multiplier =  scores[idx];
        for j in idx + 1 .. 1 + idx +w {
            scores[j] += multiplier;
        }
    }
    scores.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let h = Card::from(s);
        assert_eq!(h.winning_numbers, HashSet::from([41, 48, 83, 86, 17]));
        assert_eq!(h.hand, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_win_count() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let h = Card::from(s);
        assert_eq!(h.win_count(), 4);

        let s = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let h = Card::from(s);
        assert_eq!(h.win_count(), 2);

        let s = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let h = Card::from(s);
        assert_eq!(h.win_count(), 0);
    }
}