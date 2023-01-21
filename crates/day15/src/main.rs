use crate::grid::{Grid, Int};

mod grid;

fn main() {
    let input = include_str!("input.txt");

    let grid = Grid::parse(input).unwrap().1;
    p1(grid);
    // p2(grid);
}

fn p1(grid: Grid) {
    println!("{}", grid.to_no_in_row(2000000));
}
fn p2(grid: Grid) {
    let pos = grid.find_empty(4000000);
}
