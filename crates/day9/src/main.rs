use crate::instructions::Program;
use std::collections::HashSet;

mod instructions;

fn main() {
    let input = include_str!("input.txt");

    let mut visited_locations = HashSet::new();
    let (mut tx, mut ty) = (0, 0);
    let (mut hx, mut hy) = (0, 0);

    for instr in Program::from(input.to_string()).into_iter() {
        visited_locations.insert((tx, ty));
        visited_locations.insert((hx, hy));
    }

    println!("{}", visited_locations.len());
}
