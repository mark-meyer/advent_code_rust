use std::num::ParseIntError;
use std::collections::HashMap;
use itertools::Itertools;


#[derive(Debug, Clone, Copy)]
pub struct SecretNumber(pub i64);

impl SecretNumber {

    pub fn add_prices(&self, iterations:usize) ->  HashMap<(i8, i8, i8, i8), i64> {
        
        let mut local_prices = HashMap::new();

        let tups = self
            .tuple_windows()
            .map(|(a, b)| (b % 10 - a % 10) as i8)
            .tuple_windows::<(_, _, _, _)>();
        
        self
            .skip(4)
            .map(|n| (n % 10))
            .zip(tups)
            .take(iterations)
            .for_each(|(price, diffs)| {
                if !local_prices.contains_key(&diffs) {
                    local_prices.insert(diffs, price);
                } 
            } );
            
        local_prices
    }
}

impl TryFrom::<String> for SecretNumber {
    type Error = ParseIntError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<i64>().map(SecretNumber)
    }
}

impl Iterator for SecretNumber {
    type Item=i64;
    fn next(&mut self) -> Option<i64> {
        let orig = self.0;
        let m = self.0 * 64;
        // mix
        self.0 = self.0 ^ m;
        // prune
        self.0 %= 16777216;
    
        let m = self.0 / 32;
        self.0 = self.0 ^ m;
        self.0 %= 16777216;
    
        let m = self.0 * 2048;
        self.0 = self.0 ^ m;
        self.0 %= 16777216;
        Some(orig)
    }
}

impl From<SecretNumber> for i64 {
    fn from(num: SecretNumber) -> Self {
        num.0
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next() {
        let n = SecretNumber(123);
        assert_eq!(n.skip(2).next(), Some(16495136));

        let n = SecretNumber(10);
        assert_eq!(n.skip(2000).next(), Some(4700978));
    }

    #[test]
    fn test_from_string() {
        let s = "1234".to_string();
        let n = SecretNumber::try_from(s).unwrap();
        assert_eq!(1234, n.0)
    }
}
