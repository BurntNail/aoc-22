#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::self_named_constructors)]

use crate::monke::{parse_multiple_monkeys, IntItem, Monkey};
use num_traits::One;
use std::{iter::Inspect, time::Instant};

mod monke;

fn part(mut monkeys: Vec<Monkey>, no_rounds: usize, div_factor: IntItem) -> Vec<usize> {
    let mut transactions = vec![0; monkeys.len()];
    let mut start = Instant::now();

    let round_mark = no_rounds / 500.min(no_rounds);
    for round_no in 0..no_rounds {
        if round_no % round_mark == 0 {
            println!("Starting round {round_no} at {:?}", start.elapsed());
        }

        for i in 0..monkeys.len() {
            println!("Monkey {i} starting with {:?}", monkeys[i].clone_items());

            for (value, next_id) in monkeys[i].run_round(div_factor.clone()) {
                transactions[i] += 1;
                monkeys[next_id].add_item(value);
            }
        }
        if round_no % round_mark == 0 {
            println!("Ending {round_no}  at {:?} \n\n", start.elapsed());
        }
    }
    transactions
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");
    let (_, monkeys) = parse_multiple_monkeys(input)?;

    {
        let mut transactions = part(monkeys.clone(), 20, IntItem::from(3_u8));
        transactions.sort_unstable();

        println!("{:?}", transactions.clone());

        println!(
            "MB: {}",
            transactions.pop().unwrap() * transactions.pop().unwrap()
        );
    }

    {
        let mut transactions = part(monkeys, 10000, IntItem::one());
        transactions.sort_unstable();

        println!("{:?}", transactions.clone());

        println!(
            "MB: {}",
            transactions.pop().unwrap() * transactions.pop().unwrap()
        );
    }

    Ok(())
}
