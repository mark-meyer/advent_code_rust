#![allow(unused)]

use std::fs;
use std::boxed::Box;
use std::collections::{HashMap, HashSet};
use block::Block;
use jet::Jet;


mod  block;
mod jet;

static BLOCKS: [&[u8]; 5] = [
    &[30],
    &[8, 28, 8],
    &[28, 4, 4],
    &[16, 16, 16, 16],
    &[24, 24]
];


struct Room { blocks: Vec<u8> }

impl Room {
    fn drop_block(&mut self,  mut block: Block, mut jets:  impl Iterator<Item=(usize, Jet)>) -> usize {
        let top = self.blocks.len();
        let mut pos = top + 3;
        let block_height = block.len();
        let mut overlap = 0;

        let jet_num = loop {
            let (jet_num, jet) = jets.next().unwrap();
            // the number of rows overlapping between block and room
            overlap = top.saturating_sub(pos).min(block_height);

            let room_slice = if overlap > 0 {
                &self.blocks[pos..pos + overlap]
            } else {
                &[]
            };

            block.shift(&jet, room_slice);

            if pos == 0 { break  jet_num}

            if  pos <= top  {
                // slice of the room one step below
                let slice_below = if top > 0 {
                    &self.blocks[pos-1..pos+overlap]
                } else {
                    &[]
                };
                if block.intersects(slice_below) {
                    break jet_num
                }
            }
            pos -= 1;
        };
        // Merge overlsp
        if overlap == 0 {
            self.blocks.extend(block.rows.iter());
        } else {
            for i in 0..overlap {
                self.blocks[(pos + i) ] |=  block.rows[i];
            }
            self.blocks.extend(block.rows[overlap ..].iter());
        }
        jet_num
    }

}


fn get_rows(n: usize, prefix_len: usize, cycle_len:usize, rows_per_cycle:usize, block_indices: &Vec<usize>) -> usize {
    let blocks_after_prefix = (n - prefix_len);
    let remaining_block = blocks_after_prefix % cycle_len;
    let even_cycles = blocks_after_prefix / cycle_len;
    let prefix_rows = block_indices[prefix_len];
    let remaining_rows = block_indices[prefix_len + remaining_block] - block_indices[prefix_len];

    prefix_rows + even_cycles * rows_per_cycle + remaining_rows
}

fn main() {
    let input = fs::read_to_string("data.txt").expect("the jets are not blowing");
    let jet_len = input.len();

    let mut jets = input.chars().map(Jet::from).cycle().enumerate();
    // keep track of states to look for repetition
    // block -> row mapping to help calculations
    let mut states = HashMap::new();
    let mut block_indices:Vec<usize> = vec![];

    let mut blocks = BLOCKS.iter().map(|b| Block {rows: b.to_vec()}).cycle();
    let mut room = Room {blocks: vec![]};
    let mut i = 0;

    let k = loop {
        block_indices.push(room.blocks.len());

        let block = blocks.next().unwrap();
        let b_state = block.sum();
        let jet_num = room.drop_block(block, &mut jets);
        if i > 5 {
            // look back 5 block drops (not 5 room rows)
            // if we find a state we've seen before stop
            let bi = block_indices[block_indices.len() - 5];
            let room_state:u32 = room.blocks[bi..].iter().map(|u| *u as u32).sum();
            let k = (b_state, room_state, jet_num % jet_len);
            if states.contains_key(&k) {
                break k
            } else {
                states.insert(k, i);
            }
        }
        i += 1;
    };
    let prefix_len = states[&k] - 4;
    let cycle_len = i - states[&k];
    let rows_per_cycle = block_indices[i] - block_indices[states[&k]];
    let prefix_rows = block_indices[prefix_len];

    let part1 = get_rows(2022, prefix_len, cycle_len, rows_per_cycle, &block_indices);
    println!("Part one: {}", part1);

    let part2 = get_rows(1_000_000_000_000, prefix_len, cycle_len, rows_per_cycle, &block_indices);
    println!("Part two: {}", part2);

}
