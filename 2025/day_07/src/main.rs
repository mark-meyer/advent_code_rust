use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Beam(u64),
    Splitter(bool),
    Space
}

type Manifold = Vec<Vec<Cell>>;

fn parse_file(s:String) -> Manifold {
    s.lines().map(|line| line.chars().map(|c| match c {
        '^' => Cell::Splitter(false),
        'S' => Cell::Beam(1),
        '.' => Cell::Space,
        _ => panic!("There's a bug in the tachyon manifold. Shut it down!")
        }).collect()
    )
    .collect()
}


fn run_transporter(m: &Manifold) -> Manifold {
    let mut record = m.clone();
    let h = record.len();
    let w = record[0].len();

    for row in 0..h-1 {
        for col in 0..w {
            if let Cell::Beam(count) = record[row][col] {
                match record[row+1][col] {
                    Cell::Splitter(_) => {
                        match record[row+1][col-1] {
                            Cell::Beam(prev_count) => record[row+1][col-1] = Cell::Beam(count + prev_count),
                            Cell::Space => record[row+1][col-1] = Cell::Beam(count),
                            Cell::Splitter(_) => ()
                        }
                        match record[row+1][col+1] {
                            Cell::Beam(prev_count) => record[row+1][col+1] = Cell::Beam(count + prev_count),
                            Cell::Space => record[row+1][col+1] = Cell::Beam(count),
                            Cell::Splitter(_) => ()
                        }
                        record[row+1][col] = Cell::Splitter(true);
                    }
                    Cell::Beam(prev_count) => {
                        record[row+1][col] = Cell::Beam(count+prev_count);
                    }
                    Cell::Space => { 
                        record[row+1][col] = Cell::Beam(count) 
                    }
                }
            }
        }
    }
    record
}

fn count_activated_splitters(m: &Manifold) -> u64 {
    let mut count = 0;
    for row in m.iter(){
        for c in row.iter() {
            if let Cell::Splitter(true) = c {
                count += 1;
            }
        }
    }
    count
}

fn count_paths(m: &Manifold) -> u64 {
    let last_row:&Vec<Cell> = m.iter().last().unwrap();

    last_row.iter()
    .fold(0, |acc, cell| {
        if let Cell::Beam(count) = cell {
            count + acc
        } else {
            acc
        }
    })
  
}

fn main() {
    let raw_input = read_to_string("data.txt").expect("Couldn't start the tachyon emiiter");
    let manifold = parse_file(raw_input);

    let record = run_transporter(&manifold);
    let splits = count_activated_splitters(&record);
    let paths = count_paths(&record);
    println!("Part one: {}", splits);
    println!("Part two {}", paths);

}
