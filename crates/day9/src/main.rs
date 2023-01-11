use crate::instructions::{Direction, Program};
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

        let mut close_enough = || ((tx - hx) as f32).hypot((ty - hy) as f32) < 1.5;

        match instr.0 {
            Direction::Up => {
                hy -= 1;
                if tx == hx {
                    ty -= 1;
                }
                if close_enough() {
                    continue;
                }
            }
            Direction::Down => {
                hy += 1;
                if tx == hx {
                    ty += 1;
                }
                if close_enough() {
                    continue;
                }
            }
            Direction::Left => {
                hx -= 1;
                if ty == hy {
                    tx -= 1;
                }
                if close_enough() {
                    continue;
                }
            }
            Direction::Right => {
                hx += 1;
                if ty == hy {
                    tx += 1;
                }
                if close_enough() {
                    continue;
                }
            }
        };
    }

    println!("{}", visited_locations.len());
}
