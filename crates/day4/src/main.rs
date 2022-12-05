use std::ops::Range;

peg::parser!( grammar pair_range_parser() for str {
    rule number() -> u32 = n:$(['0'..='9']+) {n.parse().expect("invalid number")}
    rule range() -> Range<u32> = l:number() "-" r:number() { l..(r + 1) }
    pub rule pair_range() -> (Range<u32>, Range<u32>) = l:range() "," r:range() { (l,r) }
});
use pair_range_parser::pair_range;

fn main() {
    let input = include_str!("input.txt");
    let covered = input
        .lines()
        .map(|l| pair_range(l.trim()).expect("unable to parse"))
        .map(overlap_at_all) //swap for p1 vs p2
        .map(|b| b as u32)
        .sum::<u32>();

    println!("Pair sums is {covered}");
}

fn fully_overlap((t, b): (Range<u32>, Range<u32>)) -> bool {
    (t.start >= b.start && t.end <= b.end) || (t.start <= b.start && t.end >= b.end)
}
fn overlap_at_all((t, mut b): (Range<u32>, Range<u32>)) -> bool {
    fully_overlap((t.clone(), b.clone())) || t.clone().any(|x| b.contains(&x)) || b.any(|x| t.contains(&x))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn parser_test () {
        assert_eq!(pair_range("0-1,4-6"), Ok((0..2, 4..7)));
        assert_eq!(pair_range("69-420,24-55"), Ok((69..421, 24..56)));
    }
}
