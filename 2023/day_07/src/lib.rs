use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;

pub mod solutions;
// This is extra complicated becuase I want to manually try out 
// some of these traits.

#[derive(Debug)]
pub struct Hand {
    pub cards: [char; 5],
    pub bid: u32,
    counts: Vec<u8>
}

impl Hand {
    fn get_card_sort(c: char) -> u8 {
        match c {
            'j' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'J' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => 0
        }
    }
}
impl Ord for Hand{
    fn cmp(&self, other: &Self) -> Ordering {
        let self_card_score = self.cards.map(Hand::get_card_sort);
        let other_card_score = other.cards.map(Hand::get_card_sort);
        (&self.counts, self_card_score).cmp(&(&other.counts, other_card_score))
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_card_score = self.cards.map(Hand::get_card_sort);
        let other_card_score = other.cards.map(Hand::get_card_sort);
        Some((&self.counts, self_card_score).cmp(&(&other.counts, other_card_score)))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl From<String> for Hand {
    fn from(line: String) -> Self {
        let (hand, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse().expect("Bad bid input"); 
        
        let mut counts:HashMap<char, u8> = HashMap::new();
        let mut cards: [char; 5] = ['x'; 5];

        for (idx, card) in hand.chars().enumerate() {
            cards[idx] = card;
            counts.entry(card).and_modify(|counter| *counter += 1).or_insert(1);
        }
        let mut counts:Vec<u8> = counts.values().cloned().collect();
        
        counts.sort();
        counts.reverse();
        
        Hand {cards, bid, counts}
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string(){
        let hand = Hand::from("KTJJT 220".to_string());
        assert_eq!(hand.bid, 220);
        assert_eq!(hand.cards, ['K', 'T', 'J', 'J', 'T']);
        assert_eq!(hand.counts, vec![2, 2, 1]);
    }
    #[test]
    fn test_eq(){
        let hand = Hand::from("KTJJT 220".to_string());
        let hand2 = Hand::from("KTJJT 820".to_string());
        assert_eq!(hand, hand2);
    }
    #[test]
    fn test_neq(){
        let hand = Hand::from("KTJJT 220".to_string());
        let hand2 = Hand::from("KTJTJ 220".to_string());
        assert_ne!(hand, hand2);
    }
    #[test]
    fn test_equal_counts_order_by_cards(){
        let hand = Hand::from("KTJJT 220".to_string());
        let hand2 = Hand::from("KTJJJ 220".to_string());
        assert_eq!(hand.cmp(&hand2), Ordering::Less);
        assert_eq!(hand2.cmp(&hand), Ordering::Greater);
    }
    #[test]
    fn test_order_by_type(){
        let hand = Hand::from("T55J5 220".to_string());
        let hand2 = Hand::from("KK677 220".to_string());
        assert_eq!(hand.cmp(&hand2), Ordering::Greater);
    }
}