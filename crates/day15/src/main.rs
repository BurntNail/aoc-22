use crate::grid::Grid;

mod grid;

fn main() {
    let input = include_str!("input.txt");

    let grid = Grid::parse(input).unwrap().1;
    p1(grid.clone());
    p2(grid);
}

fn p1(grid: Grid) {
    println!("Number in 2_000_000: {}", grid.into_num_in_row(2000000));
}
fn p2(grid: Grid) {
    let pos = grid.beacon_position(0..=4_000_000, 0..=4_000_000).unwrap();
    println!("TF of {pos:?} == {}", pos.0 * 4_000_000 + pos.1);
}
