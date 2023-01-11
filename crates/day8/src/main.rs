use crate::map::Grid;

mod map;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");
    let grid = Grid::from(input.to_string());
    // println!("{:#?}", grid.can_see_edge());

    let mut scores = grid.scenic_scores();
    scores.sort();
    println!("Top Score: {:?}", scores.pop().unwrap());

    Ok(())
}
