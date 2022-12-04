use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").lines();
    let collected: Vec<_> = input.collect::<Vec<_>>();
    let list = collected
        .chunks(3) //Change the no of chunks for p1 vs p2
        .map(line_to_similar_value_p2)
        .map(|x| {
            match x {
                Ok(x) => x,
                Err(e) => {
                    println!("Error with {e}");
                    '!'
                }
            }
        })
        .map(char_to_priority);

    println!("Sum of similar values = {}", list.map(|x| x as u32).sum::<u32>());
}

fn line_to_similar_value_p1 (line: &[&str]) -> char {
    let line = line[0].trim();
    let (l, r) =  line.split_at(line.len() / 2);

    let hsl: HashSet<_> = l.chars().collect();
    let hsr: HashSet<_> = r.chars().collect();

    let mut int = hsl.intersection(&hsr);
    *int.next().expect("must have an intersection")
}
fn line_to_similar_value_p2 (lines: &[&str]) -> Result<char, String> {
    let hsl: HashSet<_> = lines[0].chars().collect();
    let hsm: HashSet<_> = lines[1].chars().collect();
    let hsr: HashSet<_> = lines[2].chars().collect();

    let lm_int: HashSet<_> = hsl.intersection(&hsm).copied().collect();
    let mut int = lm_int.intersection(&hsr);

    match int.next() {
        Some(x) => Ok(*x),
        None => Err(format!("{:?}", lines))
    }
}



fn char_to_priority (b: char) -> u8 {
    let b = b as u8;
    match b {
        b'a'..=b'z' => {
            b - b'a' + 1
        },
        b'A'..=b'Z' => {
            b - b'A' + 27
        },
        _ => 0
    }

}