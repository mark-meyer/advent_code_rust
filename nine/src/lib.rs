use std::collections::HashSet;
use rayon::prelude::*;

pub type Matrix = Vec<Vec<u8>>;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(x:usize, y:usize) -> Self {
        Point {x, y}
    }
}

fn get_neighbors(p: &Point, height:usize, width:usize) -> Vec<Point> {
    let mut n = Vec::new();
    if p.y > 0 {
        n.push(Point::new(p.x, p.y - 1));
    }
    if p.y < height - 1 {
        n.push(Point::new(p.x, p.y + 1));
    }
    if p.x > 0 {
        n.push(Point::new(p.x - 1, p.y));
    }
    if p.x < width - 1 {
        n.push(Point::new(p.x + 1, p.y));
    } 
    n
}

pub fn make_matrix(data:&str) -> Matrix {
    data.lines()
    .map(|line| line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}


fn dfs(m: &Matrix, start: &Point) -> usize {
    let h = m.len();
    let w = m[0].len();

    let mut count = 0;
    let mut seen:HashSet<Point> = HashSet::new();
    let mut stack:Vec<Point> = vec![start.clone()];


    while !stack.is_empty(){
        let p = stack.pop().unwrap();
        if seen.contains(&p) || m[p.y][p.x] == 9 {
            continue;
        }
        for n in get_neighbors(&p, h, w){
            stack.push(n);
        }
        seen.insert(p);
        count += 1;
    }

    count
}

fn low_points(data: &Matrix) -> (usize, Vec<Point>){
    let h = data.len();
    let w = data[0].len();
    let mut sum: usize = 0;
    let mut points = Vec::new();

    for (y, line) in data.iter().enumerate() {
        for (x, n) in line.iter().enumerate() {
            let p = Point::new(x, y);
            let smallest_neighbor = get_neighbors(&p, h, w)
                .iter()
                .map(|p| data[p.y][p.x])
                .min()
                .unwrap();

            if n < &smallest_neighbor {
                points.push(p);
                sum += 1 + *n as usize
            }
        }
    }

    (sum, points)
}
pub fn solutions(matrix: &Matrix) -> (usize, usize) {
    let (sum, wells) = low_points(&matrix);
    
    //use Rayon to run DFS in threads.
    let mut total:Vec<usize> = wells.par_iter().map(|p| dfs(&matrix, p)).collect();

    // alternatively, don't us rayon's theads:
    //let mut total:Vec<usize> = wells.iter().map(|p| dfs(&matrix, p)).collect();

    total.sort_by(|a, b| b.cmp(a));
    let top = total[..3].iter().fold(1, |a, n| a * n);
    (sum, top)
}


#[cfg(test)]
mod tests{
    use super::*;

    fn get_matrix() -> String{
        String::from("2199943210\n3987894921\n9856789892\n8767896789\n9899965678")
    }
    #[test]
    fn test_make_matrix(){
        let s = get_matrix();
        let m = make_matrix(&s);
        assert_eq!(m.len(), 5);
        assert_eq!(m[0].len(), 10);
        assert_eq!(m[1][1], 9);
        assert_eq!(m[4][9], 8);
    }
    #[test]
    fn test_dfs() {
        let s = get_matrix();
        let m = make_matrix(&s);
        let start = Point {y:3, x:1};
        
        assert_eq!(dfs(&m, &start), 14);
        let start = Point {y: 4, x: 9};
        assert_eq!(dfs(&m, &start), 9);
    }
}