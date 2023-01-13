#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::self_named_constructors)]

use crate::monke::{parse_multiple_monkeys, Monkey};

mod monke;

fn p1(mut monkeys: Vec<Monkey>, no_rounds: usize) -> Vec<usize> {
    let mut transactions = vec![0; monkeys.len()];
    for round_no in 0..no_rounds {
        println!("\n\n Starting round {}", round_no + 1);
        for i in 0..monkeys.len() {
            println!("Monkey {i} starting with {:?}", monkeys[i].clone_items());

            for (value, next_id) in monkeys[i].run_round() {
                transactions[i] += 1;
                monkeys[next_id].add_item(value);
            }
        }
    }
    transactions
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");
    let (_, monkeys) = parse_multiple_monkeys(input)?;

    let mut transactions = p1(monkeys, 20);
    transactions.sort_unstable();

    println!("{:?}", transactions.clone());

    println!(
        "MB: {}",
        transactions.pop().unwrap() * transactions.pop().unwrap()
    );

    Ok(())
}
