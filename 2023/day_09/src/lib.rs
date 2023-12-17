pub fn transform_to_zero(history: Vec<i32>)  -> Vec<Vec<i32>> {
    
    let mut transforms = vec![history];
    
    loop {
        if transforms[transforms.len() - 1].iter().all(|&n| n == 0) {
            break
        }
        let last = &transforms[transforms.len() -1];
        let a = last.iter();
        let b = last.iter().skip(1);

        transforms.push(a.zip(b).map(|(a, b)| b - a).collect());
    }
    transforms
}

pub fn transform_from_zeros(histories: &Vec<Vec<i32>>) -> i32{
    histories.iter().map(| v | v[v.len()-1]).sum()
}

pub fn transform_prefix(histories: &Vec<Vec<i32>>) -> i32 {
    histories.iter().rev().fold(0, |a, c| c[0] - a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_zero() {
        let h = vec![0, 3, 6, 9, 12, 15];
        let result = transform_to_zero(h);
        assert_eq!(result, vec![
            vec![0, 3, 6, 9, 12, 15], 
            vec![3, 3, 3, 3, 3], 
            vec![0, 0, 0, 0]
            ]);
    }

    #[test]
    fn test_transform_from_zeros() {
        let z = vec![
            vec![0, 3, 6, 9, 12, 15], 
            vec![3, 3, 3, 3, 3], 
            vec![0, 0, 0, 0]
            ];
        let result = transform_from_zeros(&z);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_transform_prefix() {
        let z = vec![
            vec![10, 13, 16, 21, 30,  45],
            vec![3,  3,  5,  9,  15],
            vec![0,  2,  4,  6],
            vec![2,  2,  2,  2, 2],
            vec![0,  0,  0 ]
        ];
        let result = transform_prefix(&z);
        assert_eq!(result, 5);
    }
}
