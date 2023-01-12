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

        let (nhx, nhy) = direction(instr, (hx, hy));
        hx = nhx;
        hy = nhy;

        let (ntx, nty) = follow((tx, ty), (hx, hy));
        tx = ntx;
        ty = nty;
    }
    visited_locations.insert((tx, ty));
    visited_locations.len()
}

fn p2(singles: Vec<Direction>, no: usize) -> usize {
    assert!(no > 1);
    let mut visited_locations = HashSet::new();
    let mut snake = vec![(0, 0); no];

    for instr in singles {
        visited_locations.insert(snake[no - 1]);

        let (nhx, nhy) = direction(instr, snake[0]);
        snake[0].0 = nhx;
        snake[0].1 = nhy;

        for i in 1..no {
            let (ntx, nty) = follow(snake[i], snake[i - 1]);
            snake[i].0 = ntx;
            snake[i].1 = nty;
        }
    }

    visited_locations.insert(snake[no - 1]);
    visited_locations.len()
}

///Returns the new tail
fn follow((mut tx, mut ty): (i32, i32), (hx, hy): (i32, i32)) -> (i32, i32) {
    let (dx, dy) = (hx - tx, hy - ty);

    let (addx, addy) = match (dx, dy) {
        (1 | 2, 2) | (2, 1) => (1, 1),
        (-1 | -2, 2) | (-2, 1) => (-1, 1),
        (1 | 2, -2) | (2, -1) => (1, -1),
        (-1 | -2, -2) | (-2, -1) => (-1, -1),
        (0, 2) => (0, 1),
        (0, -2) => (0, -1),
        (2, 0) => (1, 0),
        (-2, 0) => (-1, 0),
        (0 | 1 | -1, 0 | 1 | -1) => (0, 0),
        _ => panic!("Unseen dx dy: {dx},{dy}"),
    };

    tx += addx;
    ty += addy;

    (tx, ty)
}

const fn direction(instr: Direction, (mut hx, mut hy): (i32, i32)) -> (i32, i32) {
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
    (hx, hy)
}

fn main() {
    let input = include_str!("input.txt");
    let singles = Program::from(input.to_string()).into_singles();

    println!("P1: {}", p1(singles.clone()));
    println!("P2: {}", p2(singles, 10));
}
