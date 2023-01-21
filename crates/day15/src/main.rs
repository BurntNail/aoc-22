use crate::grid::Grid;

mod grid;

fn main() {
    let input = include_str!("input.txt");

    let grid = Grid::parse(input).unwrap().1;

    let slots = grid.to_filled_grid();

    let count = slots.into_iter().filter(|(_, y)| *y == 10).count() - 1;

    println!("{count}");
}
