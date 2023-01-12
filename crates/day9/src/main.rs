#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::instructions::{Direction, Program};
use std::collections::HashSet;

mod instructions;

fn p1(singles: Vec<Direction>) -> usize {
    let mut visited_locations = HashSet::new();
    let (mut tx, mut ty) = (0, 0);
    let (mut hx, mut hy) = (0, 0);

    for instr in singles {
        visited_locations.insert((tx, ty));

        direction(instr, (&mut hx, &mut hy));
        follow((&mut tx, &mut ty), (hx, hy));
    }
    visited_locations.insert((tx, ty));
    visited_locations.len()
}

fn follow((tx, ty): (&mut i32, &mut i32), (hx, hy): (i32, i32)) {
    let (dx, dy) = (hx - *tx, hy - *ty);

    let (addx, addy) = match (dx, dy) {
        (1, 2) | (2, 1) => (1, 1),
        (-1, 2) | (-2, 1) => (-1, 1),
        (1, -2) | (2, -1) => (1, -1),
        (-1, -2) | (-2, -1) => (-1, -1),
        (0, 2) => (0, 1),
        (0, -2) => (0, -1),
        (2, 0) => (1, 0),
        (-2, 0) => (-1, 0),
        (0 | 1 | -1, 0 | 1 | -1) => (0, 0),
        _ => panic!("Unseen dx dy: {dx},{dy}"),
    };

    *tx += addx;
    *ty += addy;
}

fn direction(instr: Direction, (hx, hy): (&mut i32, &mut i32)) {
    match instr {
        //Top left is (0,0)
        Direction::Up => {
            *hy += 1;
        }
        Direction::Down => {
            *hy -= 1;
        }
        Direction::Left => {
            *hx -= 1;
        }
        Direction::Right => {
            *hx += 1;
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let singles = Program::from(input.to_string()).into_singles();

    println!("P1: {}", p1(singles));
    // println!("{:?}", part_one(input));
}
