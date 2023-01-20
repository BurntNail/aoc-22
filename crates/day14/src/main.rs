#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use crate::rock::{Coords, Line};
use rock::State;
use std::{collections::HashMap, time::Instant};

mod rock;

fn main() {
    let rock_lines = Line::parse_all(include_str!("input.txt")).unwrap().1;
    // println!("No of sands that work: {}", part(rock_lines.clone(), false));
    let timer = Instant::now();
    println!(
        "No of sands that work with a floor: {}",
        part(rock_lines, true)
    );
    println!("Took {:?}.", timer.elapsed()); //running repeatedly, fastest is around 17ms
}

fn get_rocks_array(rock_lines: Vec<Line>, make_floor: bool) -> (HashMap<Coords, State>, usize) {
    assert!(!rock_lines.is_empty());

    let mut max_row = usize::MIN;
    let mut max_col = usize::MIN;

    let mut rs_and_cs: HashMap<Coords, State> = rock_lines
        .into_iter()
        .flat_map(Line::into_interior)
        .map(|(row, col)| {
            max_row = max_row.max(row);
            max_col = max_col.max(col);
            ((row, col), State::Rock)
        })
        .collect();

    if make_floor {
        max_row += 2;
        for col in 0..max_col + 100 {
            rs_and_cs.insert((max_row, col), State::Rock);
        }
    }

    (rs_and_cs, max_row)
}

fn part(rock_lines: Vec<Line>, is_p2: bool) -> usize {
    const SAND_START: Coords = (0, 500);
    let (mut map, max_row) = get_rocks_array(rock_lines, is_p2);

    let mut sands = 0;
    'outer: loop {
        let mut sand_pos = SAND_START;

        let check = |delta_row, delta_col, pos: Coords| -> Option<Coords> {
            if pos.1 == 0 && delta_col == -1 {
                //all the way on the left, going further left, overflow protection
                None
            } else {
                let (check_row, check_col) = (pos.0 as i64 + delta_row, pos.1 as i64 + delta_col);
                let (check_row, check_col) = (check_row as usize, check_col as usize);
                if check_row == SAND_START.0
                    || check_col == SAND_START.1
                    || map.contains_key(&(check_row, check_col))
                {
                    None
                } else {
                    Some((check_row, check_col))
                }
            }
        };

        'inner: loop {
            if !is_p2 && sand_pos.0 == max_row {
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

        map.insert(sand_pos, State::FallenSand);

        sands += 1;
    }

    if is_p2 {
        sands += 1; //for the start point
    }

    // for row in array.as_rows() {
    //     for item in row {
    //         print!("{item}");
    //     }
    //     println!();
    // }

    sands
}
