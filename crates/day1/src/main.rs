fn main() {
    let inp = include_str!("input.txt");
    let mut list = vec![];
    let mut current = 0;

    for line in inp.lines().map(|l| l.trim()) {
        if line == "" {
            list.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    list.sort();
    let len = list.len() - 1;
    println!("Largest: {}", list[len]);
    println!("Top 3 Largest: {}", list[len] + list[len - 1] + list[len - 2]);
}
