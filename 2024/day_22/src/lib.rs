use ahash::AHashMap;


const KEY_SIZE:u32 = 4;
const BITS_PER_SLOT: u32 = 5;
const MASK: u32 = (1 << (KEY_SIZE * BITS_PER_SLOT)) - 1; 


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

fn shift_in(n:i8, key:u32) -> u32 {
    // the idea is the a key needs to accomodate 
    // 4 numbers from -9 to 9. Adding 9 gets us 0 - 18
    let n =  (n + 9) as u32;

    ((key << BITS_PER_SLOT) | n) & MASK
}

pub fn add_prices(n: i64, iterations:usize) ->  AHashMap<u32, i64> {
    
    let mut local_prices = AHashMap::new();
    let mut current = n;
    let mut key = 0;

    for _ in 0..4 {
        let next_secret = current;
        current = next(current);
        key = shift_in((current % 10  - next_secret % 10) as i8, key);
    }
    

    local_prices.insert(key, current % 10);

    for _ in 0..iterations - 4 {
        let next_secret = current;
        current = next(current);
        key = shift_in((current % 10  - next_secret % 10) as i8, key);

        if !local_prices.contains_key(&key) {
            local_prices.insert(key, current % 10);
        } 
    }

    local_prices
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next() {
        let n = 123;
        assert_eq!(next(next(n)),16495136);
    }

    #[test]
    fn test_get_nth() {
        assert_eq!(nth_next(10, 2000), 4700978);
    }

}
