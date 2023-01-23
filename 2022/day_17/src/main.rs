#![allow(unused)]

use std::fs;
use std::boxed::Box;
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


struct Room {
    blocks: Vec<u8>,
}

impl Room {
    fn new() -> Room {
        Room {
            blocks: vec![],
        }
    }


    fn drop_block(&mut self,  mut block: Block, mut jets:  impl Iterator<Item=Jet>) {
        let top = self.blocks.len();
        let mut pos = top + 3;
        let block_height = block.len();
        let mut overlap = 0;

        loop {
            let jet = jets.next().unwrap();
            // the number of rows overlapping between block and room
            overlap = top.saturating_sub(pos).min(block_height);

            let room_slice = if overlap > 0 {
                &self.blocks[pos..pos + overlap]
            } else {
                &[]
            };

            block.shift(&jet, room_slice);

            if pos == 0 { break }

            if  pos <= top  {
                // slice of the room one step below
                let slice_below = if top > 0 {
                    &self.blocks[pos-1..pos+overlap]
                } else {
                    &[]
                };
                if block.intersects(slice_below) {
                    break
                }
            }
            pos -= 1;
        }
        // Merge overlsp
        if overlap == 0 {
            self.blocks.extend(block.rows.iter());
        } else {
            for i in 0..overlap {
                self.blocks[(pos + i) ] |=  block.rows[i]
            }
            self.blocks.extend(block.rows[overlap ..].iter());
        }
    }
}



fn main() {
    let input = fs::read_to_string("data.txt").expect("the jets are not blowing");
    let mut jets = input.chars().map(Jet::from).cycle();

    let blocks = BLOCKS.iter().map(|b| Block {rows: b.to_vec()}).cycle();
    let mut room = Room::new();

    for block in blocks.take(2022) {
        room.drop_block(block, &mut jets);
    }

    println!("Part one: {}", room.blocks.len());
}
