#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use crate::rock::{Coords, Line};
use std::{collections::HashSet, time::Instant};

mod rock;

fn main() {
    let rock_lines = Line::parse_all(include_str!("input.txt")).unwrap().1;
    // println!("No of sands that work: {}", part(rock_lines.clone(), false));
    let tim = Instant::now();
    println!(
        "No of sands that work with a floor: {}",
        part(rock_lines, true)
    );
    println!("Took {:?}.", tim.elapsed());
}

fn get_rocks_array(rock_lines: Vec<Line>, make_floor: bool) -> (HashSet<Coords>, usize) {
    assert!(!rock_lines.is_empty());

    let mut max_row = usize::MIN;
    let mut max_col = usize::MIN;

    let mut rs_and_cs: HashSet<Coords> = rock_lines
        .into_iter()
        .flat_map(Line::into_interior)
        .map(|(row, col)| {
            max_row = max_row.max(row);
            max_col = max_col.max(col);
            (row, col)
        })
        .collect();

    if make_floor {
        max_row += 2;
        for col in 0..max_col + 100 {
            rs_and_cs.insert((max_row, col));
        }
    }

    (rs_and_cs, max_row)
}

fn part(rock_lines: Vec<Line>, is_p2: bool) -> usize {
    const SAND_START: Coords = (0, 500);
    let (mut map, max_row) = get_rocks_array(rock_lines, is_p2);

    let mut sands = usize::from(is_p2);
    'outer: loop {
        let mut sand_pos = SAND_START;

        let check = |delta_row, delta_col, pos: Coords| -> Option<Coords> {
            if pos.1 == 0 && delta_col == -1 {
                //all the way on the left, going further left, overflow protection
                None
            } else {
                let (check_row, check_col) =
                    (pos.0 + delta_row, (pos.1 as i64 + delta_col) as usize);
                if check_row >= max_row || map.contains(&(check_row, check_col)) {
                    None
                } else {
                    Some((check_row, check_col))
                }
            }
        };

        'inner: loop {
            if sand_pos.0 == max_row {
                break 'outer;
            }

            if let Some(new) = check(1, 0, sand_pos) {
                sand_pos = new;
            } else if let Some(new) = check(1, -1, sand_pos) {
                sand_pos = new;
            } else if let Some(new) = check(1, 1, sand_pos) {
                sand_pos = new;
            } else if sand_pos == SAND_START {
                break 'outer;
            } else {
                break 'inner;
            }
        }

        map.insert(sand_pos);

        sands += 1;
    }

    sands
}
