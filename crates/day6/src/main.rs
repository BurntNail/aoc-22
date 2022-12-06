#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::{time::Instant};

const MSG_LEN: usize = 14; //4 for p1, 14 for p2


fn main() {
    let timer = Instant::now();

    let input: Vec<_> = include_str!("input.txt").chars().enumerate().collect();

    let mut end_index = 0;
    'outer: for window in input.windows(MSG_LEN) {

        let mut chars = 0_u32;
        for (wi, (i, c)) in window.iter().enumerate() {
            let new = chars | (1 << (*c as u8 - b'a'));
            if new == chars {
                break; 
            }

            if wi == MSG_LEN - 1 {
                end_index = i + 1; 
                break 'outer;
            }
            chars = new;
        }
    }

    println!("EI: {end_index:?} TT: {:?}", timer.elapsed()); //2 or 3 ms for pre-optimisation
}