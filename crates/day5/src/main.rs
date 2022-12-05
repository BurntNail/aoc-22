#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

peg::parser!(grammar crates_list() for str {
    rule crate_contents() -> Option<char> = ['['|' '] c:['A'..='Z']? " "?  [']'|' '] { c }
    pub rule crates () -> Vec<Option<char>> = l:(crate_contents() ** " ") {l}
});
peg::parser!(grammar movement() for str {
    rule number() -> u32 = n:$(['0'..='9']+) {n.parse().expect("invalid number")}
    pub rule movement () -> (usize, usize, usize) = "move " n:number() " from " a:number() " to " b:number() { (n as usize, a as usize - 1, b as usize - 1) } //Minus one as input.txt is human indexed
});
use crates_list::crates;
use movement::movement;

use color_eyre::{
    eyre::{bail, eyre},
    Result,
};

fn find_crate_rows<'a>(input: impl Iterator<Item = &'a String>) -> Result<usize> {
    for (i, l) in input.enumerate() {
        if l.contains(" 1") {
            return Ok(i);
        } else if l.contains("move") {
            break; //uh oh
        }
    }

    bail!("unable to find end of buckets")
}

fn find_vertical_buckets(input: &[String], no_crate_rows: usize) -> Result<Vec<Vec<char>>> {
    let mut buckets: Vec<Vec<char>> = vec![Vec::with_capacity(no_crate_rows); 9];
    for crate_instr in &input[0..no_crate_rows] {
        let hb: Vec<Option<char>> = crates(crate_instr)?;
        for (i, el) in hb.into_iter().enumerate() {
            if let Some(el) = el {
                buckets[i].push(el);
            }
        }
    }
    buckets.iter_mut().for_each(|b| b.reverse());
    Ok(buckets)
}

fn execute_instructions(
    buckets: &mut Vec<Vec<char>>,
    input: &[String],
    no_crate_rows: usize,
    is_p1: bool,
) -> Result<()> {
    for move_instr in &input[(no_crate_rows + 2)..] {
        let (no, from, to): (usize, usize, usize) = movement(move_instr)?;

        let mut taken = (0..no)
            .into_iter()
            .map(|_| buckets[from].pop().ok_or_else(|| eyre!("Empty Stack")))
            .collect::<Result<Vec<_>, _>>()?;

        if is_p1 {
            taken.reverse();
        }

        buckets[to].append(&mut taken);
    }

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.trim_end().to_string())
        .collect::<Vec<_>>();

    let no_crate_rows = find_crate_rows(input.iter())?;
    let mut buckets = find_vertical_buckets(&input, no_crate_rows)?;
    execute_instructions(&mut buckets, &input, no_crate_rows, true)?;

    for mut b in buckets.into_iter() {
        print!("{}", b.pop().expect("element to pop"));
    }

    Ok(())
}
