use std::fs::read_to_string;

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

fn run_transporter_two(m: &Manifold) -> (u64, usize) {
    let h = m.len();
    let w = m[0].len();
    // Keep track of used spliters
    let mut used_splitters = 0;

    // DP record
    let mut record = vec![vec![0; w]; h];
    let start = m[0].iter().position(|&c| c ==  Cell::Beam).unwrap();
    record[0][start] = 1;

    for (row, line) in m.iter().enumerate().skip(1) {
        for (col, cell) in line.iter().enumerate() {      
            match cell {
                Cell::Space => record[row][col] = record[row-1][col] + record[row][col],
                Cell::Beam => (),
                Cell::Splitter=> {
                    record[row][col-1] = record[row][col-1] + record[row-1][col];
                    record[row][col+1] = record[row][col+1] + record[row-1][col];
                    if record[row-1][col] > 0 {
                        used_splitters += 1
                    }
                }
            }
        }
    }
    (record[h-1].iter().sum(), used_splitters)

}


fn main() {
    let raw_input = read_to_string("data.txt").expect("Couldn't start the tachyon emiiter");
    let manifold = parse_file(&raw_input);

    let (paths, splits) = run_transporter_two(&manifold);
    println!("Part one: {}", splits);
    println!("Part two: {}", paths);
}
