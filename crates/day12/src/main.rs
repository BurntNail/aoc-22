#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::{graph::Graph, grid::Grid};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc,
};

mod graph;
mod grid;

pub type Coord = (usize, usize);

fn main() {
    let input = include_str!("input.txt");

    let grid = Grid::try_from(input.to_string()).expect("unable to get input TS");
    let starts = grid.get_starting_locations(false); //true for p1, false for p2
    let (end, adjacencies) = grid.to_places_i_can_get_to();

    let ones_left = Arc::new(AtomicI32::new(starts.len() as i32));

    let num_steps = starts
        .into_par_iter()
        .map_with((adjacencies, ones_left), |(adjacency, ones_left), start| {
            let graph = Graph::new(adjacency.clone(), start, end);
            let e = graph.find_end();

            let ol = ones_left.fetch_sub(1, Ordering::SeqCst) - 1;
            println!("From {start:?} found in {e} steps. {ol} left");

            e
        })
        .min();

    println!("Took {num_steps:?} steps");
}
