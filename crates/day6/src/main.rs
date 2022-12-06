#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::collections::HashMap;

const MSG_LEN: usize = 14; //4 for p1, 14 for p2


fn main() {
    let input: Vec<_> = include_str!("input.txt").chars().enumerate().collect();

    let mut end_index = 0;
    for window in input.windows(MSG_LEN) {
        let mut containers: HashMap<char, u32> = HashMap::new();
        for (_, c) in window {
            *containers.entry(*c).or_default() += 1;
        }

        if containers.iter().all(|(_, no)| no == &1) {
            end_index = window[MSG_LEN - 1].0 + 1; 
            break;
        }
    }

    println!("EI: {end_index:?}");
}