use crate::instructions::{Direction, Program};
use std::collections::HashSet;

mod instructions;

fn p1(singles: Vec<Direction>) -> usize {
    let mut visited_locations = HashSet::new();
    let (mut tx, mut ty) = (0, 0);
    let (mut hx, mut hy) = (0, 0);

    for instr in singles {
        visited_locations.insert((tx, ty));

        match instr {
            //Top left is (0,0)
            Direction::Up => {
                hy += 1;
            }
            Direction::Down => {
                hy -= 1;
            }
            Direction::Left => {
                hx -= 1;
            }
            Direction::Right => {
                hx += 1;
            }
        }

        let (dx, dy) = (hx - tx, hy - ty);

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
    visited_locations.insert((tx, ty));
    visited_locations.len()
}

fn main() {
    let input = include_str!("input.txt");
    let singles = Program::from(input.to_string()).to_singles();

    println!("P1: {}", p1(singles));
    // println!("{:?}", part_one(input));
}
