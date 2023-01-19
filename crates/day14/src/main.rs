#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use array2d::Array2D;
use itertools::Itertools;
use rock::State;

use crate::rock::{Coords, Line};

mod rock;

fn main() {
    let rock_lines = Line::parse_all(include_str!("input.txt")).unwrap().1;
    println!("No of sands that work: {}", p1(rock_lines));
}

fn get_rocks_array(rock_lines: Vec<Line>) -> (Array2D<State>, Coords) {
    assert!(!rock_lines.is_empty());

    let rs_and_cs = rock_lines
        .into_iter()
        .flat_map(Line::into_interior)
        .collect_vec();

    let (mut min_row, mut min_col, mut max_row, mut max_col) =
        (usize::MAX, usize::MAX, usize::MIN, usize::MIN);
    for (row, col) in rs_and_cs.iter().chain(vec![(0, 500)].iter()) {
        let (row, col) = (*row, *col);

        min_row = min_row.min(row);
        max_row = max_row.max(row);
        min_col = min_col.min(col);
        max_col = max_col.max(col);
    }

    let mut array =
        Array2D::filled_with(State::Nothing, max_row - min_row + 1, max_col - min_col + 1);

    for (row, col) in rs_and_cs {
        array
            .set(row - min_row, col - min_col, State::Rock)
            .expect("failed to set variable");
    }

    (array, (0 - min_row, 500 - min_col))
}

fn p1(rock_lines: Vec<Line>) -> usize {
    let (mut array, sand_start) = get_rocks_array(rock_lines);
    println!("{sand_start:?}");

    let mut sands = 0;
    'outer: loop {
        let mut sand_pos = sand_start;

        let check = |delta_row, delta_col, pos: Coords| -> Option<Coords> {
            let (check_row, check_col) = (pos.0 as isize + delta_row, pos.1 as isize + delta_col);
            let (check_row, check_col) = (check_row as usize, check_col as usize);
            if array
                .get(check_row, check_col)
                .map_or(true, |x| x.is_solid())
            {
                None
            } else {
                Some((check_row, check_col))
            }
        };

        'inner: loop {
            if sand_pos.0 == array.num_rows() - 1 {
                println!(
                    "Breaking outer at {:?}",
                    (sand_pos.0 + sand_start.0, sand_pos.1 + sand_start.1)
                );
                break 'outer;
            }

            if let Some(new) = check(1, 0, sand_pos) {
                sand_pos = new;
            } else if let Some(new) = check(1, -1, sand_pos) {
                sand_pos = new;
            } else if let Some(new) = check(1, 1, sand_pos) {
                sand_pos = new;
            } else {
                break 'inner;
            }
        }

        array
            .set(sand_pos.0, sand_pos.1, State::FallenSand)
            .unwrap();

        sands += 1;
    }

    for row in array.as_rows() {
        for item in row {
            print!("{item}");
        }
        println!();
    }

    sands
}
