#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::grid::Grid;

mod graph;
mod grid;

fn main() {
    let input = include_str!("input.txt");
    let grid = Grid::try_from(input.to_string())
        .expect("unable to get input TS")
        .to_places_i_can_get_to();
    println!("{grid:#?}");
}
