use crate::grid::Grid;

mod grid;

fn main() {
    let input = include_str!("input.txt");

    let grid = Grid::parse(input).unwrap().1;
    p1(grid);
    // p2(grid);
}

fn p1(grid: Grid) {
    println!("{}", grid.to_num_in_row(2000000));
}
// fn p2(grid: Grid) {
//     let pos = grid.find_empty(4000000);
// }
