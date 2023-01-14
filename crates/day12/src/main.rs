#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::{graph::Graph, grid::Grid};

mod graph;
mod grid;

pub type Coord = (usize, usize);

fn main() {
    let input = include_str!("input.txt");

    let grid = Grid::try_from(input.to_string()).expect("unable to get input TS");
    let ends = grid.get_starting_locations(false); //true for p1, false for p2
    let (start, adjacencies) = grid.to_places_i_can_get_to();

    let num_steps = Graph::new(adjacencies, start).find_end(&ends);
    println!("Took {num_steps:?} steps");
}
