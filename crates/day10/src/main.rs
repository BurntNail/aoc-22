#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use program::Program;

mod program;

fn main() {
    let input = include_str!("input.txt");
    let program = Program::from(input.to_string());
    let signals = program.run();

    let important_ones = Program::to_signal_strengths(signals.clone(), 20, 40);
    println!("Sum of important strengths: {}", important_ones.into_iter().sum::<i32>());

    Program::part_2(signals, 40);
}
