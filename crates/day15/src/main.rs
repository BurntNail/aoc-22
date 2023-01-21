use crate::grid::{Grid, Int};

mod grid;

pub const ROW_I_CARE_ABOUT: Int = 2000000;

fn main() {
    let input = include_str!("input.txt");

    let grid = Grid::parse(input).unwrap().1;

    let count = grid.to_no_in_row(ROW_I_CARE_ABOUT);

    println!("{count}");
}
