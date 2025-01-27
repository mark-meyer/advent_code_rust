use std::collections::HashMap;
use itertools::Itertools;


fn next(orig:i64) -> i64 {
    let mut orig = orig ^ orig * 64  % 16777216;
    orig = orig ^ orig / 32 % 16777216;
    
    orig ^  orig * 2048 % 16777216
}

pub fn nth_next(number:i64, iterations: usize) -> i64 {
    let mut current = number;
    for _ in 0..iterations {
        current = next(current);
    }
    current
}

pub fn add_prices(n: i64, iterations:usize) ->  HashMap<(i8, i8, i8, i8), i64> {
    
    let mut local_prices = HashMap::new();
    let mut current = n;
    
    let mut diffs:(i8, i8, i8, i8) = (0..4)
    .map(|_| {
        let next_secret = current;
        current = next(current);
        (current % 10  - next_secret % 10) as i8
    })
    .collect_tuple().unwrap();

    local_prices.insert(diffs, current % 10);

    for _ in 0..iterations - 4 {
        let next_secret = current;
        current = next(current);
        diffs = (
            diffs.1,
            diffs.2,
            diffs.3,
            (current % 10  - next_secret % 10) as i8
        );
        if !local_prices.contains_key(&diffs) {
            local_prices.insert(diffs, current % 10);
        } 
    }

    local_prices
}




// #[cfg(test)]
// mod test {
//     use super::*;

    // #[test]
    // fn test_next() {
    //     let n = SecretNumber(123);
    //     assert_eq!(n.skip(2).next(), Some(16495136));

    //     let n = SecretNumber(10);
    //     assert_eq!(n.skip(2000).next(), Some(4700978));
    // }

    // #[test]
    // fn test_from_string() {
    //     let s = "1234".to_string();
    //     let n = SecretNumber::try_from(s).unwrap();
    //     assert_eq!(1234, n.0)
    // }
// }
