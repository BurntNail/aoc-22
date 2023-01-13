#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::self_named_constructors)]

use crate::monke::parse_multiple_monkeys;

mod monke;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");
    let (_, monkeys) = parse_multiple_monkeys(input)?;
    println!("{monkeys:#?}");

    Ok(())
}
