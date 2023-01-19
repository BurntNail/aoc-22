#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use array2d::Array2D;
use itertools::Itertools;
use rock::RockState;

use crate::rock::RockLine;

mod rock;

fn main() {
    let rock_lines = RockLine::parse_all(include_str!("input.txt")).unwrap().1;
    println!("No of sands that work: {}", p1(rock_lines));
}

fn get_rocks_array(rock_lines: Vec<RockLine>) -> Array2D<RockState> {
    assert!(!rock_lines.is_empty());

    let rs_and_cs = rock_lines
        .into_iter()
        .map(RockLine::to_interior)
        .flatten()
        .collect_vec();

    let (mut min_row, mut min_col, mut max_row, mut max_col) = {
        let (r, c) = rs_and_cs[0];
        (r, c, r, c)
    };

    for (row, col) in &rs_and_cs {
        let (row, col) = (*row, *col);
        if row < min_row {
            min_row = row;
        }
        if row > max_row {
            max_row = row;
        }
        if col < min_col {
            min_col = col;
        }
        if col > max_col {
            max_col = col;
        }
    }

    let mut array = Array2D::filled_with(RockState::Nothing, max_row - min_row + 1, max_col - min_col + 1);

    for (row, col) in rs_and_cs {
        array
            .set(row - min_row, col - min_col, RockState::Rock)
            .expect("failed to set variable");
    }

    array
}

fn p1(rock_lines: Vec<RockLine>) -> usize {
    let array = get_rocks_array(rock_lines);

    for row in array.as_rows() {
        for item in row {
            print!("{item}");
        }
        println!();
    }

    0
}
