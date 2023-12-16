use super::*;

pub fn part_one(hands:&mut Vec<Hand>) -> u32 {
    hands.sort();
    hands.iter().enumerate().map(|(i,h)| (1+i as u32) * h.bid).sum()
}

pub fn part_two(hands: &mut Vec<Hand>) -> u32 {
    // adjust each hand to account for Jokers:
    for hand in hands.iter_mut() {
        let mut j_count = 0;
        // replace `J` with `j` which gets a score of 0
        for card in hand.cards.iter_mut() {
            if *card == 'J' {
                j_count += 1;
                *card = 'j'; 
            }
        }
        // remove the joker's count
        if let Some(index) = hand.counts.iter().position(|&count| count == j_count) {
            hand.counts.remove(index);
        }
        // add it to the leading card
        // watch for 5 joker case.
        if hand.counts.is_empty() {
            hand.counts.push(j_count)
        } else {
            hand.counts[0] += j_count;
        }
    }
    hands.sort();
    hands.iter().enumerate().map(|(i,h)| (1+i as u32) * h.bid).sum()
}