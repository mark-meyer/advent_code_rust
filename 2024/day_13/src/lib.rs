use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy)]
pub struct Button {
    pub x: i64,
    pub y: i64
}

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r".+?(\d+).+?(\d+)").unwrap()
});

impl From<&str> for Button {
    fn from(s:&str) -> Self {
        let caps = RE.captures(&s).unwrap();

        Button {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Machine {
    pub button_a: Button,
    pub button_b: Button,
    pub prize: Button
}

impl Machine {
    pub fn solve_linear_congruence(&self) -> Option<(i64, i64, i64)> {
        // Solves the congruence a * A.row ≡ C.row (mod B.row):

        let (g, mut x, _) = coef_gcd(self.button_a.x, self.button_b.x);

        // No solution
        if self.prize.x % g != 0 {
            return None
        }
        
        x *= self.prize.x / g;
        let base = self.button_b.x / g;
        x = x % base;

        Some((g, x, base))
    }

    pub fn min_route(&self) -> Option<i64>{
        let cong_sol = self.solve_linear_congruence()?;
        let (_, x, base) = cong_sol;

        let d = self.button_a.y * self.button_b.x - self.button_a.x * self.button_b.y;
        let e = self.prize.y * self.button_b.x - self.prize.x * self.button_b.y;

        let numerator = e - x * d;
        let denom = base * d;

        if numerator % denom != 0 {
            return None
        }

        let k = numerator / denom;
        
        let a = x + k * base;
        let b = (self.prize.x - a * self.button_a.x) / self.button_b.x;
        Some(3 * a + b)
    }
}

pub fn coef_gcd(a:i64, b:i64) -> (i64, i64, i64) {
    /*
    Extended GCD -- Need a little more than math.gcd gives. To solve equations below we need x and y
    that allows xa + yb = gcd(a,b) where a and b are a column of the button vectors
    */
    let (mut r_1 , mut r) = (a as i64, b as i64); 
    let (mut x_1, mut x) = (1, 0);
    let (mut y_1, mut y) = (0, 1);
   
    while r != 0 {
        let q = r_1 / r;
        (r_1, r) = (r, r_1 - q * r);
        (x_1, x) = (x, x_1 - q * x);
        (y_1, y) = (y, y_1 - q * y);

    }
    (r_1, x_1, y_1)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_base() {
        let (g, x, y) = coef_gcd(10, 0);
        assert_eq!((g, x, y), (10, 1, 0));
    }

    #[test]
    fn test_gcd() {
        // https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Example
        let (g, x, y) = coef_gcd(240, 46);
        assert_eq!((g, x, y), (2, -9, 47));
    }

    #[test]
    fn test_congruence_find() {
        let m = Machine {
            button_a: Button {x:2, y:20},
            button_b: Button{x:10, y:20},
            prize: Button{x:16, y:20}
        };
        // a== 3 : 3 * 2 = 16 mod(10)
        assert_eq!(Some((2, 3, 5)), m.solve_linear_congruence());
    }
    #[test]
    fn test_congruence_no_solution() {
        let m = Machine {
            button_a: Button {x:2, y:20},
            button_b: Button{x:10, y:20},
            prize: Button{x:15, y:20}
        };
        // ? * 2 = 15 mod 10  None
        assert_eq!(None, m.solve_linear_congruence());
    }

    #[test]
    fn test_machine_solution(){
        let m = Machine {
            button_a: Button {x:94, y:34},
            button_b: Button{x:22, y:67},
            prize: Button{x:8400, y:5400}
        };
        assert_eq!(Some(280), m.min_route());
    }
}
