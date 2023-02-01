use std::fs;

fn shift(l: &mut Vec<(usize, isize, usize)>, from: usize) {
    
    let len = l.len();
    let mut insert_index = from;
    let num =  l[from].1;
    
    let shift = num.rem_euclid(len as isize - 1);
    if shift == 0 {
        return
    }
    for _ in 0..shift{
        let j = l[insert_index].2;
        insert_index = j;
    }

    // take the next index and prev index
    // effectively remove the moving node out of the list
    let prev = l[from].0;
    let next = l[from].2;
    
    l[prev].2 = next;
    l[next].0 = prev;

    let  after_insert_index = l[insert_index].2;

    l[from].0 = insert_index;
    l[from].2 = l[insert_index].2;

    l[insert_index].2 = from;
    l[after_insert_index].0 = from;
}

fn reorder(l: &Vec<(usize, isize, usize)>) -> (Vec<isize>, Option<usize>) {
    let mut v = Vec::with_capacity(l.len());
    let mut j = 0;
    let mut zero_index = None;
    for i in 0..l.len(){
        if l[j].1 == 0 {
            zero_index = Some(i);
        }
        v.push(l[j].1);
        j = l[j].2;
    }
    (v, zero_index)

}

fn build_input(data: &Vec<isize>, key:isize) -> Vec<(usize, isize, usize)> {
    let mut l:Vec<_> = data.iter().enumerate().map(|(i, n)| (i.saturating_sub(1), *n * key, i + 1)).collect();
    let last_index = l.len() - 1;
    l[0].0 = last_index;
    l[last_index].2 = 0;
    l
}

fn main() {
    let data = fs::read_to_string("data.txt").expect("no signal found!");
    let orig: Vec<isize> = data.lines().flat_map(|n| n.parse().ok()).collect();

    let mut l = build_input(&orig, 1);

    (0..l.len()).for_each(|i| shift(&mut l, i));
    let (v, z) = reorder(&l);
    let z = z.unwrap();
    let part1:isize = [1000, 2000, 3000].iter().map(|i| v[(i + z) % l.len()]).sum();

    println!("Part one: {}", part1);

    let key = 811589153;
    let mut l = build_input(&orig, key);

    for _ in 0..10 {
        (0..l.len()).for_each(|i| shift(&mut l, i));
    }
    let (v, z) = reorder(&l);
    let z = z.unwrap();
    let part2:isize = [1000, 2000, 3000].iter().map(|i| v[(i + z) % l.len()]).sum();

    println!("Part two: {}", part2);


}

