use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    pub start: String,
    pub end: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct BoundRange {
    pub start: String,
    pub end: String,
    pub repeated: usize,
}

impl From<(&str, &str)> for Range {
    fn from(pair: (&str, &str)) -> Range {
        let (start, end) = pair;
        Range{start: start.to_string(), end: end.to_string()}
    }
}

impl Range {
    pub fn adjusted_range(&self, repeat: usize) -> Option<BoundRange> {
        let mut left = self.start.clone();
        let mut right = self.end.clone();

        if self.start.len() % repeat as usize != 0 {
            left = String::from("1") + &"0".repeat(left.len());
        }
        if self.end.len() % repeat as usize != 0 {
            right = String::from("9") + &"9".repeat(right.len() -  2);
        }
        if left.len() <= right.len() {
            Some(BoundRange{start: left, end: right, repeated: repeat})
        } else {
            None
        }
    }
}

impl BoundRange{
    pub fn invalid_keys(&self) -> Result<HashSet<usize>, Box<dyn Error>> {
        let mut invalid_ids = HashSet::new();
        let start_n:usize = self.start.parse()?;
        let end_n:usize = self.end.parse()?;

        let start = (self.start[..self.start.len() / self.repeated]).parse::<usize>()?;
        let end = 1 + (self.end[..self.end.len() / self.repeated]).parse::<usize>()?;
        for n in start..end {
            let candidate = n.to_string().repeat(self.repeated).parse()?;
            if start_n <= candidate && end_n >= candidate {
                invalid_ids.insert(candidate);
            }
        }
        Ok(invalid_ids)
    }
}


#[cfg(test)]
mod tests { 
    use super::*;
        
    #[test]
    fn test_adjusted_range(){
        let br = BoundRange{start:"95".to_string(), end: "99".to_string(), repeated: 2};

        assert_eq!(Range::from(("95", "115")).adjusted_range(2), Some(br));
        assert_eq!(Range::from(("1698522", "1698528")).adjusted_range(2), None);
    }
    #[test]
    fn test_invalid_keys() {
        let br = BoundRange{start:"95".to_string(), end: "99".to_string(), repeated: 2};
        assert_eq!(br.invalid_keys().unwrap(), HashSet::from_iter([99]) )
    }
}