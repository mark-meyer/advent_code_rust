use std::collections::HashMap;

type MemoTable = HashMap<((u8, u8), (u8, u8)), (u64, u64)>;
type Player = (u8, u8);

fn update(player: Player, roll:u8) -> (u8, u8) {
    let (pos, score) = player;
    let pos = (pos - 1  + roll) % 10 + 1;
    (pos, score + pos)
}

const THREE_ROLL_PERMUTATIONS:[(u8, u8); 7] = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1)
];

fn game(player1:Player, player2:Player, memo:&mut MemoTable) -> (u64, u64) {
    let mut p1_wins:u64 = 0;
    let mut p2_wins:u64 = 0;

    for (roll, count) in THREE_ROLL_PERMUTATIONS {
        let (pos, score) = update(player1, roll);
        if score >= 21{
            p1_wins += count as u64;
        } else {
            match memo.get(&(player2,(pos, score))) {
                Some((win2, win1)) => {
                    p1_wins += win1 * count as u64;
                    p2_wins += win2 * count as u64;
                }
                None => {
                    let(win2, win1) = game(player2,(pos, score), memo);
                    memo.insert((player2, (pos, score)), (win2, win1));
                    p1_wins += win1 * count as u64;
                    p2_wins += win2 * count as u64;        
                }
            }
        }
    } 
    
    (p1_wins, p2_wins)
}

fn run_game(p1_start: u8, p2_start: u8) -> (u64, u64) {
    let mut memo:MemoTable = HashMap::new();
    let player1: Player = (p1_start, 0);
    let player2: Player = (p2_start, 0);
    
    game(player1, player2, &mut memo)
}

fn main() {
    let result = run_game(10, 7);
    println!("Game Result: {:?}", result);
    println!("Solution 2: {:?}", result.0.max(result.1))
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_update(){
        let res = update((1, 10), 4);
        assert_eq!(res, (5, 15));

        let res = update((9, 2), 4);
        assert_eq!(res, (3, 5));
    }
}