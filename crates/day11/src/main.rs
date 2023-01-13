#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::self_named_constructors)]

use crate::monke::{parse_multiple_monkeys, IntItem, Monkey};
use num_traits::One;

mod monke;

fn part(mut monkeys: Vec<Monkey>, no_rounds: usize, div_factor: IntItem) -> Vec<usize> {
    let mut transactions = vec![0; monkeys.len()];
    let divisor_product = monkeys.iter().map(|m| m.test.clone()).product::<IntItem>(); //not funny, aoc: https://fasterthanli.me/series/advent-of-code-2022/part-11#math-check

    for _ in 0..no_rounds {
        for i in 0..monkeys.len() {
            for (value, next_id) in monkeys[i].run_round(div_factor.clone(), &divisor_product) {
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

    {
        let mut transactions = part(monkeys.clone(), 20, IntItem::from(3_u8));
        transactions.sort_unstable();

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
