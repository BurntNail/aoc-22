use crate::lists::Pair;
use nom::{character::complete::i32, IResult};
use std::cmp::Ordering;

mod lists;

pub type IntItem = i32;
pub fn int_item(input: &str) -> IResult<&str, IntItem> {
    i32(input)
}

fn main() {
    //working for sample given, but not for actual solution - too low
    let input = include_str!("input.txt");
    let pairs = Pair::get_pairs(input).expect("getting pairs").1;

    println!("Checking {} pairs", pairs.len());

    let product_working = pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            let i = i + 1;
            println!("Checking {i}");
            if pair.compare() == Ordering::Less {
                Some(i)
            } else {
                None
            }
        })
        .sum::<usize>();
    // .collect::<Vec<_>>();
    println!("Working sum is {product_working:?}.");
}
