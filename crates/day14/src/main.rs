#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use array2d::Array2D;
use itertools::Itertools;
use rock::State;

use crate::rock::{Coords, Line};

mod rock;

fn main() {
    let rock_lines = Line::parse_all(include_str!("input.txt")).unwrap().1;
    // println!("No of sands that work: {}", p1(rock_lines.clone(), false));
    println!(
        "No of sands that work with a floor: {}",
        p1(rock_lines, true)
    );
}

fn get_rocks_array(rock_lines: Vec<Line>, make_floor: bool) -> (Array2D<State>, Coords) {
    assert!(!rock_lines.is_empty());

    let mut rs_and_cs = rock_lines
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
    max_row += 2;

    if make_floor {
        rs_and_cs.extend((min_col..=max_col).map(|col| (max_row, col)));
    }

    let mut array =
        Array2D::filled_with(State::Nothing, max_row - min_row + 1, max_col - min_col + 1);

    for (row, col) in rs_and_cs {
        array
            .set(row - min_row, col - min_col, State::Rock)
            .expect("failed to set variable");
    }

    array.set(0 - min_row, 500 - min_col, State::Start).unwrap();

    (array, (0 - min_row, 500 - min_col))
}

fn p1(rock_lines: Vec<Line>, is_p2: bool) -> usize {
    let (mut array, sand_start) = get_rocks_array(rock_lines, is_p2);
    let max = (array.num_rows() - 1, array.num_columns() - 1);

    let mut sands = 0;
    'outer: loop {
        let mut sand_pos = sand_start;

        let check = |delta_row, delta_col, pos: Coords| -> Option<Coords> {
            let (check_row, check_col) = (pos.0 as isize + delta_row, pos.1 as isize + delta_col);
            let (check_row, check_col) = (check_row as usize, check_col as usize);
            if check_row > max.0
                || check_col > max.1
                || array
                    .get(check_row, check_col)
                    .map_or(true, |x| x.is_solid())
            {
                None
            } else {
                Some((check_row, check_col))
            }
        };

        'inner: loop {
            if !is_p2 && sand_pos.0 == max.0 {
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

        if array
            .get(sand_pos.0, sand_pos.1)
            .map_or(false, |x| *x == State::Start)
        {
            break 'outer;
        }

        array
            .set(sand_pos.0, sand_pos.1, State::FallenSand)
            .unwrap();

        sands += 1;
        println!("Sands at {sands}");

        // if sands % 5000 == 0 {
        //     for row in array.as_rows() {
        //         for item in row {
        //             print!("{item}");
        //         }
        //         println!();
        //     }
        //     std::thread::sleep(std::time::Duration::from_millis(6000));
        // }
    }

    for row in array.as_rows() {
        for item in row {
            print!("{item}");
        }
        println!();
    }

    sands
}
