#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::map::Grid;

mod map;

fn main() {
    let input = include_str!("input.txt");
    let grid = Grid::from(input.to_string());

    println!("{:#?}", grid.can_see_edge());

    let mut scores = grid.scenic_scores();
    scores.sort_unstable();
    println!("Top Score: {:?}", scores.pop().unwrap());
}
