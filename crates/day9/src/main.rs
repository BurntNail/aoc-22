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
                if tx == hx {
                    ty -= 1;
                }
            }
            Direction::Down => {
                hy += 1;
                if tx == hx {
                    ty += 1;
                }
            }
            Direction::Left => {
                hx -= 1;
                if ty == hy {
                    tx -= 1;
                }
            }
            Direction::Right => {
                hx += 1;
                if ty == hy {
                    tx += 1;
                }

            }
        }
        if ((tx - hx) as f32).hypot((ty - hy) as f32) < 1.5 {
            continue;
        }

        let dx = hx - tx;
        let dy = hy - ty;

        let (addx, addy) = match (dx, dy) {
            (1,2) | (2,1) => (1,1),
            (-1,2) | (-2,1) => (-1,1),
            (1,-2) | (2,-1) => (1,-1),
            (-1,-2) | (-2,-1) => (-1,-1),
            _ => panic!("Unseen dx dy  - {},{}", dx, dy),
        };
        tx += addx;
        ty += addy;
    }

    println!("{}", visited_locations.len());
}
