pub mod parse_input;

#[derive(Debug)]
pub struct Conversion {
    pub interval: Interval,
    pub delta: i64
}

impl From<String>for Conversion {
    fn from(input: String) -> Self {
        let ns:Vec<i64> = input.split_whitespace().flat_map(|s| s.parse().ok()).collect();

        let i = Interval {
            start: ns[1],
            stop: ns[1] + ns[2]
        };
        Conversion {
            interval: i,
            delta: ns[0] - ns[1]
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub start: i64,
    pub stop: i64,
}

impl Interval {
    pub fn interval_list_from_seeds(seed_list: &Vec<i64>) -> Vec<Interval> {
        seed_list.iter().step_by(2)
            .zip(seed_list.iter().skip(1).step_by(2))
            .map(|(&start, &length)| Interval {start: start, stop: start + length} )
            .collect()
    }

    pub fn overlaps(&self, other: Interval) -> bool {
        self.start.max(other.start) < self.stop.min(other.stop)
    }

    pub fn contains(&self, n: i64) -> bool {
        n >= self.start && n < self.stop
    }

    pub fn split_conversion(&self, conversion:&Conversion) -> (Option<Interval>, Vec<Interval>) {
        // Converts any part of single interval if it overlaps and returns the converted 
        // interval and any leftover pieces that did not overlap (which don't get converted) 
        if !self.overlaps(conversion.interval) {
            (None, vec![self.clone()])
        } else {
            let mut remainders = Vec::new();
            let start = self.start.max(conversion.interval.start);
            let stop = self.stop.min(conversion.interval.stop);
            
            let converted: Interval = Interval {
                start: start + conversion.delta,
                stop: stop + conversion.delta
            };
            
            if start > self.start {
                remainders.push(Interval{start: self.start, stop:conversion.interval.start})
            } 
            if  stop < self.stop {
                remainders.push(Interval{start: conversion.interval.stop, stop: self.stop})
            }; 
            (Some(converted), remainders)
        }
    }
    pub fn convert_interval_list(&self, conversions: &Vec<Conversion>) -> (Vec<Interval>, Vec<Interval>) {
        let mut total_remaining = vec![self.clone()];
        let mut total_conversions = vec![];

        for conv in conversions {
            let mut leftovers = vec![];

            for interval in total_remaining {
                let (converted, mut remainders) = interval.split_conversion(conv);
                if let Some(converted) = converted{
                    total_conversions.push(converted);
                }
                leftovers.append(&mut remainders);
            }
            total_remaining = leftovers;
        }

        (total_conversions, total_remaining)
    }
}

pub fn convert_seed(seed: i64, conversions: &Vec<Conversion>) -> i64{
    conversions.iter()
        .filter(|conv| conv.interval.contains(seed))
        .fold(seed, |acc, conv| acc + conv.delta )
}

pub fn process_conversions(seed: i64, conversion_collection: &Vec<Vec<Conversion>>) -> i64{
    conversion_collection
        .iter()
        .fold(seed, |acc, conv| convert_seed(acc, conv))
}

pub fn part_one(seeds: &Vec<i64>, conversions: &Vec<Vec<Conversion>>) -> Option<i64>{
    seeds
        .iter()
        .map(|&seed| process_conversions(seed, conversions))
        .min()
}

pub fn part_two(seeds: &Vec<i64>, conversions: &Vec<Vec<Conversion>>) -> Option<i64>{
    let mut seed_ranges = Interval::interval_list_from_seeds(seeds);
    let mut converted: Vec<Interval>;
    let mut leftovers: Vec<Interval>;
    for conversion in conversions {
        converted = vec![];
        leftovers = vec![];
        for seed_interval in seed_ranges{
            let (mut local_converted, mut local_leftovers) = seed_interval.convert_interval_list(conversion);
            converted.append(&mut local_converted);
            leftovers.append(&mut local_leftovers);
        };
        converted.append(&mut leftovers);
        seed_ranges = converted;
    };

    seed_ranges.iter().map(|i| i.start).min()
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap_true(){
        let a = Interval{start: 10, stop: 20};
        let b = Interval{start: 15, stop: 21};
        assert!(a.overlaps(b));

        let a = Interval{start: 10, stop: 20};
        let b = Interval{start: 9, stop: 11};
        assert!(a.overlaps(b));
    }

    #[test]
    fn test_overlap_false() {
        let a = Interval{start: 10, stop: 20};
        let b = Interval{start: 20, stop: 21};
        assert!(!a.overlaps(b));

        let a = Interval{start: 10, stop: 20};
        let b = Interval{start: 30, stop: 41};
        assert!(!a.overlaps(b));
    }

    #[test]
    fn test_conversion_from_string() {
        let input = String::from("52 50 48");
        let conv = Conversion::from(input);

        assert_eq!(conv.delta, 2);
        assert_eq!(conv.interval.start, 50);
        assert_eq!(conv.interval.stop, 98);
    }

    #[test]
    fn test_convert_seed() {
        let conversions = vec! [
            Conversion {interval: Interval {start: 10, stop: 20}, delta: 100},
            Conversion {interval: Interval {start: 18, stop: 200}, delta: 500}
        ];

        assert_eq!( convert_seed(101, &conversions), 601);  // one matching
        assert_eq!( convert_seed(1, &conversions), 1);      // none matching - pass through
        assert_eq!( convert_seed(19, &conversions), 619);   // two matching 
    }

    #[test]
    fn convert_seed_list(){
        let seeds: Vec<i64> = vec![79, 14, 55, 13];
        let intervals = Interval::interval_list_from_seeds(&seeds);
        assert_eq!(intervals[0].start, 79);
        assert_eq!(intervals[0].stop, 93);

        assert_eq!(intervals[1].start, 55);
        assert_eq!(intervals[1].stop, 68);
    }

    #[test]
    fn test_split_conversion() {
        let i1 = Interval {start: 10, stop: 30};
        let c1 = Conversion {
            interval: Interval {start: 11, stop: 25},
            delta: 100
        };
        let (conv, remainder) = i1.split_conversion(&c1);
        assert_eq!(Some(Interval{start: 111, stop: 125}), conv);
        assert_eq!(vec![
            Interval{start: 10, stop: 11},
            Interval{start: 25, stop: 30}
            ], remainder);
    }
    #[test]
    fn test_convert_interval_list() {
        let i1 = Interval {start: 10, stop: 20};
        let c1 = Conversion {
            interval: Interval {start: 11, stop: 15},
            delta: 100
        };
        let c2 = Conversion {
            interval: Interval {start: 15, stop: 17},
            delta: 100
        };

        let (converted, remaining) = i1.convert_interval_list(&vec![c1, c2]);
        assert_eq!(remaining, vec![Interval{start: 10, stop: 11}, Interval{start: 17, stop: 20}]);
        assert_eq!(converted, vec![Interval{start: 111, stop: 115}, Interval{start: 115, stop: 117}])
    }
}
