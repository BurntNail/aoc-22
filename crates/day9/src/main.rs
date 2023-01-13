#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::instructions::{Direction, Program};
use std::collections::HashSet;

mod instructions;

fn p1(singles: Vec<Direction>) -> usize {
    let mut visited_locations = HashSet::new();
    let (mut tail_x, mut tail_y) = (0, 0);
    let (mut head_x, mut head_y) = (0, 0);

    for instr in singles {
        visited_locations.insert((tail_x, tail_y));

        let (new_head_x, new_head_y) = direction(instr, (head_x, head_y));
        head_x = new_head_x;
        head_y = new_head_y;

        let (new_tail_x, new_tail_y) = follow((tail_x, tail_y), (head_x, head_y));
        tail_x = new_tail_x;
        tail_y = new_tail_y;
    }
    visited_locations.insert((tail_x, tail_y));
    visited_locations.len()
}

fn p2(singles: Vec<Direction>, no: usize) -> usize {
    assert!(no > 1);
    let mut visited_locations = HashSet::new();
    let mut snake = vec![(0, 0); no];

    for instr in singles {
        visited_locations.insert(snake[no - 1]);

        let (new_head_x, new_head_y) = direction(instr, snake[0]);
        snake[0].0 = new_head_x;
        snake[0].1 = new_head_y;

        for i in 1..no {
            let (new_tail_x, new_tail_y) = follow(snake[i], snake[i - 1]);
            snake[i].0 = new_tail_x;
            snake[i].1 = new_tail_y;
        }
    }

    visited_locations.insert(snake[no - 1]);
    visited_locations.len()
}

///Returns the new tail
fn follow((mut tail_x, mut tail_y): (i32, i32), (head_x, head_y): (i32, i32)) -> (i32, i32) {
    let (delta_x, delta_y) = (head_x - tail_x, head_y - tail_y);

    let (add_x, add_y) = match (delta_x, delta_y) {
        (1 | 2, 2) | (2, 1) => (1, 1),
        (-1 | -2, 2) | (-2, 1) => (-1, 1),
        (1 | 2, -2) | (2, -1) => (1, -1),
        (-1 | -2, -2) | (-2, -1) => (-1, -1),
        (0, 2) => (0, 1),
        (0, -2) => (0, -1),
        (2, 0) => (1, 0),
        (-2, 0) => (-1, 0),
        (0 | 1 | -1, 0 | 1 | -1) => (0, 0),
        _ => panic!("Unseen dx dy: {delta_x},{delta_y}"),
    };

    tail_x += add_x;
    tail_y += add_y;

    (tail_x, tail_y)
}

const fn direction(instr: Direction, (mut head_x, mut head_y): (i32, i32)) -> (i32, i32) {
    match instr {
        Direction::Up => {
            head_y += 1;
        }
        Direction::Down => {
            head_y -= 1;
        }
        Direction::Left => {
            head_x -= 1;
        }
        Direction::Right => {
            head_x += 1;
        }
    }
    (head_x, head_y)
}

fn main() {
    let input = include_str!("input.txt");
    let singles = Program::from(input.to_string()).into_singles();

    println!("P1: {}", p1(singles.clone()));
    println!("P2: {}", p2(singles, 10));
}
