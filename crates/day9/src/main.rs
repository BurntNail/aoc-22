use crate::instructions::{Direction, Program};
use std::collections::HashSet;

mod instructions;

fn main() {
    let input = include_str!("input.txt");

    let mut visited_locations = HashSet::new();
    let (mut tx, mut ty) = (0, 0);
    let (mut hx, mut hy) = (0, 0);

    for instr in Program::from(input.to_string()).to_singles() {
        visited_locations.insert((tx, ty));

        match instr {
            Direction::Up => {
                hy -= 1;
            }
            Direction::Down => {
                hy += 1;
            }
            Direction::Left => {
                hx -= 1;
            }
            Direction::Right => {
                hx += 1;
            }
        }

        let dx = hx - tx;
        let dy = hy - ty;

        let (addx, addy) = match (dx, dy) {
            (1, 2) | (2, 1) => (1, 1),
            (-1, 2) | (-2, 1) => (-1, 1),
            (1, -2) | (2, -1) => (1, -1),
            (-1, -2) | (-2, -1) => (-1, -1),
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            (0, 0)
            | (1, 0)
            | (0, 1)
            | (1, 1)
            | (-1, 0)
            | (0, -1)
            | (-1, -1)
            | (1, -1)
            | (-1, 1) => (0, 0),
            _ => panic!("Unseen dx dy  - {},{}", dx, dy),
        };

        tx += addx;
        ty += addy;

        println!("({tx},{ty})");
    }

    println!("{}", visited_locations.len());
}
