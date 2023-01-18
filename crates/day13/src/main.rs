use crate::lists::Pair;
use lists::Item;
use nom::{character::complete::i32, IResult};
use std::cmp::Ordering;

mod lists;

pub type IntItem = i32;
pub fn int_item(input: &str) -> IResult<&str, IntItem> {
    i32(input)
}

fn main() {
    let input = include_str!("input.txt");
    let pairs = Pair::get_all(input).unwrap().1;
    println!("Working sum is {:?}.", p1(pairs.clone()));
    println!("Divisors product is {:?}", p2(pairs.clone()));
}

fn p1 (v: Vec<Item>) -> usize {
    let pairs = Pair::get_pairs(v);

    println!("Checking {} pairs", pairs.len());

    pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            let i = i + 1;
            if pair.compare() == Ordering::Less {
                Some(i)
            } else {
                None
            }
        })
        .sum()
}

fn p2 (mut v: Vec<Item>) -> usize {
    let divisors = Pair::get_all("[[2]]\n[[6]]").unwrap().1;

    v.sort();

    println!("{:?}", divisors.iter().map(|x| format!("{x:?}")).collect::<Vec<String>>().join("\n"));

    v.into_iter().enumerate().filter_map(|(index, item)| if divisors.contains(&item) { Some(index + 1) } else {None}).product()
}