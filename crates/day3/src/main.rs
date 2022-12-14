#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").lines().map(str::trim);
    let collected: Vec<_> = input.collect::<Vec<_>>();
    let list = collected
        .chunks(3) //Change the no of chunks for p1 vs p2
        .map(line_to_similar_value)
        .map(char_to_priority);

    println!(
        "Sum of similar values = {}",
        list.map(u32::from).sum::<u32>()
    );
}

fn line_to_similar_value(lines: &[&str]) -> char {
    let mut hs: HashSet<char> = ('a'..='z').chain('A'..='Z').collect();

    for l in lines {
        let hs_local: HashSet<_> = l.chars().collect();
        hs = hs_local.intersection(&hs).copied().collect();
    }

    *hs.iter().next().expect("needs intersection")
}

#[must_use]
pub const fn char_to_priority(b: char) -> u8 {
    let b = b as u8;
    match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 27,
        _ => 0,
    }
}
