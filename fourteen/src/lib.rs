use std::str;
use std::collections::HashMap;

type Substitution = HashMap<String, (String, String)>;
type Counter = HashMap<String, u64>;

#[derive(Debug)]
pub struct PolymerCounter {
    pub current_counts: Counter,
    pub substitutions: Substitution,
    last_letter: u8
}

impl PolymerCounter {
    pub fn new(template: &str, lookup: &HashMap<&str, &str>) -> Self {
        let mut current_counts:HashMap<String, u64> = HashMap::new();

        for pair in template.as_bytes().windows(2){
            let key = str::from_utf8(pair).unwrap().to_string();
            *current_counts.entry(key).or_insert(0) += 1;
        } 

        let mut substitutions = HashMap::new();
        for (key, value) in lookup {
            let b = key.as_bytes();
            let v = value.as_bytes();
            let pair = (
                str::from_utf8(&[b[0], v[0]]).unwrap().to_owned(),
                str::from_utf8(&[v[0], b[1]]).unwrap().to_owned()
            );

            substitutions.insert(
                key.to_string(),
                pair
            );
        }
        let last_letter = template.as_bytes().last().unwrap().clone();
        Self {current_counts, substitutions, last_letter}
    }

    fn increase_counts(&mut self) {
        let mut counts:Counter = HashMap::new();
        for (pair, count) in &self.current_counts {
            let (p1, p2) = self.substitutions.get(pair).unwrap();
            *counts.entry(p1.to_string()).or_default() += count;
            *counts.entry(p2.to_string()).or_default() += count;
        }
        self.current_counts = counts;
    }

    pub fn run_substitutions(&mut self, n: u8) {
        for _ in 0..n {
            self.increase_counts();
        }
    }
    pub fn count_letters(&self) -> u64 {
        let mut counts: HashMap<u8, u64> = HashMap::new();
        for (key, count) in &self.current_counts {
            let bytes = key.as_bytes();
            *counts.entry(bytes[0]).or_default() += count;
        }
        *counts.entry(self.last_letter).or_default() += 1;
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();
        max - min

    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_new_counter_counts(){
        let template = "abc";
        let lookup = HashMap::from([
            ("ab", "c"),
            ("bc", "d")
        ]);
        let counter = PolymerCounter::new(template, &lookup);
        assert_eq!(
            counter.current_counts,
            HashMap::from([("ab".to_string(), 1), ("bc".to_string(), 1)])
        )
    }
    #[test]
    fn test_new_counter_substitutions(){
        let template = "abc";
        let lookup = HashMap::from([
            ("ab", "c"),
            ("bc", "d")
        ]);
        let counter = PolymerCounter::new(template, &lookup);
        assert_eq!(
            counter.substitutions,
            HashMap::from([
                ("ab".to_string(), ("ac".to_string(), "cb".to_string())), 
                ("bc".to_string(), ("bd".to_string(), "dc".to_string()))
            ])
        )
    }

    #[test]
    fn test_increase_counts(){
        let template = "abc";
        let lookup = HashMap::from([
            ("ab", "c"),
            ("bc", "c")
        ]);
        let mut counter = PolymerCounter::new(template, &lookup);
        counter.increase_counts();
        assert_eq!(
            counter.current_counts,
            HashMap::from([
                ("ac".to_string(), 1), ("cb".to_string(), 1),
                ("bc".to_string(), 1), ("cc".to_string(), 1),
                ])
        )
    }
}