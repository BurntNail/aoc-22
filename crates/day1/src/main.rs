#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

fn main() {
    let inp = include_str!("input.txt");
    let mut list = vec![];
    let mut current = 0;

    for line in inp.lines().map(str::trim) {
        if line.is_empty() {
            list.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    list.sort_unstable();
    let len = list.len() - 1;
    println!("Largest: {}", list[len]);
    println!(
        "Top 3 Largest: {}",
        list[len] + list[len - 1] + list[len - 2]
    );
}
