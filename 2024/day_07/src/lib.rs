#[derive(Debug)]
pub struct TestValue {
    pub target: u64,
    pub values: Vec<u64>,
}

impl TestValue {
    pub fn new(target: u64, values: Vec<u64>) -> Self {
        TestValue {target, values}
    }

    pub fn valid_total<F>(&self, operators: &Vec<F>) -> Option<u64> 
        where F: Fn(u64, u64) -> u64
    {
        if self.values.is_empty(){
            return None;
        }
        let mut stack = vec![(self.values[0], 1)];

        while !stack.is_empty(){
            if let Some((total, index)) = stack.pop() {
                if index == self.values.len() {
                    if total == self.target {
                        return Some(total)
                    }    
                } else if total <= self.target {
                    stack.extend(operators
                        .iter()
                        .map(|f| (f(total, self.values[index]), index+1))
                    );
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use std::ops::{Add, Mul};
    use super::*;

    #[test]
    fn test_is_invalid_empty() {
        let t = TestValue::new(10, vec![]);
        assert_eq!(t.valid_total(&vec![Add::add, Mul::mul]), None);
    }

    #[test]
    fn test_is_valid_mul() {
        let t = TestValue::new(10, vec![5, 1, 2]);
        assert_eq!(t.valid_total(&vec![Add::add, Mul::mul]), Some(10));
    }

    #[test]
    fn test_is_valid_single() {
        let t = TestValue::new(10, vec![10]);
        assert_eq!(t.valid_total(&vec![Add::add, Mul::mul]), Some(10));
    }

    #[test]
    fn test_is_mul() {
        let t = TestValue::new(10, vec![10, 1]);
        assert_eq!(t.valid_total(&vec![Add::add, Mul::mul]), Some(10));
    }
    #[test]
    fn test_is_add() {
        let t = TestValue::new(13, vec![10, 1, 2]);
        assert_eq!(t.valid_total(&vec![Add::add, Mul::mul]), Some(13));
    }

    #[test]
    fn test_is_combo_valid() {
        let t = TestValue::new(22, vec![10, 1, 2]);
        assert_eq!(t.valid_total(&vec![Add::add, Mul::mul]), Some(22));
    }
    #[test]
    fn test_is_combo_invalid() {
        let t = TestValue::new(23, vec![10, 1, 2]);
        assert_eq!(t.valid_total(&vec![Add::add, Mul::mul]), None);
    }

}
