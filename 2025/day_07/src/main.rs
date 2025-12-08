use std::fs::read_to_string;
use std::mem::swap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Beam,
    Splitter,
    Space
}

type Manifold = Vec<Vec<Cell>>;

fn parse_file(s:&str) -> Manifold {
    s.lines().map(|line| line.chars().map(|c| match c {
        '^' => Cell::Splitter,
        'S' => Cell::Beam,
        '.' => Cell::Space,
        _ => panic!("There's a bug in the tachyon manifold. Shut it down!")
        }).collect()
    )
    .collect()
}

fn run_transporter_two(m: &Manifold) -> (u64, u64) {
    let w = m[0].len();
    // Keep track of used spliters
    let mut used_splitters = 0;

    // DP record
    let mut prev_counts = vec![0u64; w];
    let mut curr_counts = vec![0u64; w];

    
    let start = m[0].iter().position(|&c| c ==  Cell::Beam).unwrap();
    prev_counts[start] = 1;

    for line in m.iter().skip(1) {
        curr_counts.fill(0);
        for (col, cell) in line.iter().enumerate() {   
            let incoming_beam = prev_counts[col];
            if incoming_beam == 0 {
                continue
            }
            match cell {
                Cell::Space => curr_counts[col] += incoming_beam,
                Cell::Beam => (),
                Cell::Splitter=> {
                    // splitters never happen on edges ftw!
                    curr_counts[col - 1] += incoming_beam;
                    curr_counts[col + 1] += incoming_beam;
                    if incoming_beam > 0 {
                        used_splitters += 1
                    }
                }
            }
        }
        swap(&mut prev_counts, &mut curr_counts);
    }

    (curr_counts.iter().sum(), used_splitters)
}


fn main() {
    let raw_input = read_to_string("data.txt").expect("Couldn't start the tachyon emiiter");
    let manifold = parse_file(&raw_input);

    let (paths, splits) = run_transporter_two(&manifold);
    println!("Part one: {}", splits);
    println!("Part two: {}", paths);
}
