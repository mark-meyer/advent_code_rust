use std::fs;
use std::path::Path;
use std::collections::{VecDeque, HashSet};

use day_09::*;

fn parse(s:&str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let files:Vec<(u64, u64)> = s
    .chars()
    .step_by(2)
    .enumerate()
    .map(|(i, n)| (i as u64, n.to_digit(10).unwrap() as u64))
    .collect();

    let space:Vec<u64> = s
    .chars()
    .skip(1)
    .step_by(2)
    .map(|n| n.to_digit(10).unwrap() as u64)
    .collect();

    (files, space)
}

fn get_checksum(files: &Vec<(u64, u64)>, free: &Vec<u64>) -> u64 {
    let mut files = VecDeque::from(files.clone());
    let mut free_blocks = VecDeque::from(free.clone());
    let mut pos:u64 = 0;
    let mut checksum:u64 = 0;

    let (mut compact_id, mut compact_count) = match files.pop_back() {
        Some((id, count)) => (id, count),
        None => return 0, 
    };

    while let Some((file_id, size)) = files.pop_front() {
        for _ in 0..size {
            checksum += file_id * pos;
            pos += 1;
        }
        
        if let Some(free) = free_blocks.pop_front() {
            for _ in 0..free {
                checksum += compact_id * pos;
                pos += 1;
                compact_count = compact_count.saturating_sub(1);

                if compact_count == 0 {
                    match files.pop_back() {
                        Some((c_id, c_count)) => {
                            compact_count = c_count;
                            compact_id = c_id;
                        },
                        None => break,
                    }
                }
            }
        }
    }

    for _ in 0..compact_count {
        checksum += pos * compact_id;
        pos += 1;
    }
    checksum
}   

fn compact_files(files: &Vec<(u64, u64)>, free_block: &Vec<u64>) -> u64 {
    let segment_data:Vec<SegmentData> = free_block
        .iter()
        .map(|&value| SegmentData{value: value, data:vec![]})
        .collect();

    let mut disk_blocks = SegmentTree::new(
        &segment_data, |a, b| a
        .value
        .partial_cmp(&b.value)
        .unwrap_or(std::cmp::Ordering::Equal)
    );

    let mut move_indices = HashSet::new();

    // start from the back and move file to the first spot on the left that will hold it
    for (move_index, (file_id,  file_size)) in files.iter().enumerate().rev(){

        if let Some(found_idx) = disk_blocks.leftmost_fee_space(*file_size) {
            // don't look beyone the current spot
            if found_idx < move_index {
                let seg_data = disk_blocks.get(found_idx).unwrap();

                let new_data = vec![*file_id as u32; *file_size as usize];

                disk_blocks.update(
                    found_idx, 
                    seg_data.value - *file_size,
                    [seg_data.data.clone(), new_data].concat()
                );
                
                // keep track of moved indices so
                // we don't count them twice.
                move_indices.insert(move_index);
            }
        }
    }
    let mut pos = 0;
    let mut checksum = 0;
    files.iter()
    .zip(disk_blocks.iter())
    .for_each(|(original, SegmentData{value:remaining, data:moved})| {
        let (id, size) = original;
        for _ in 0..*size {
            if !move_indices.contains(&(*id as usize)) {
                checksum += pos * id;
            }
            pos += 1;
        }
        
        for id in moved {
            checksum += pos * *id as u64;
            pos += 1;
    
        }
        pos += *remaining as u64
    });
    checksum
}

fn main() {
    let p = Path::new("data.txt");
    let s = fs::read_to_string(p).expect("Could not read from file");
    let (mut files, free) = parse(&s);
    println!("{:?}", get_checksum(&files, &free));

    println!("{:?}", compact_files(&mut files, &free));
}
