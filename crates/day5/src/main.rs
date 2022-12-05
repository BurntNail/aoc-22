peg::parser!(grammar crates_list() for str {
    rule crate_contents() -> Option<char> = ['['|' '] c:['A'..='Z']? " "?  [']'|' '] { c }
    pub rule crates () -> Vec<Option<char>> = l:(crate_contents() ** " ") {l}
});
peg::parser!(grammar movement() for str {
    rule number() -> u32 = n:$(['0'..='9']+) {n.parse().expect("invalid number")}
    pub rule movement () -> (usize, usize, usize) = "move " n:number() " from " a:number() " to " b:number() { (n as usize, a as usize - 1, b as usize - 1) } //Minus one as input.txt is human indexed
});
use crates_list::crates;
use movement::movement;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>();

    let no_crate_rows = {
        let mut ans = None;
        for (i, l) in input.iter().enumerate() {
            if l.contains(" 1") {
                ans = Some(i);
                break;
            } else if l.contains("move") {
                break; //uh oh
            }
        }
        ans.expect("unable to find end of buckets")
    };

    let mut buckets: Vec<Vec<char>> = vec![Vec::with_capacity(no_crate_rows); 9];
    for crate_instr in &input[0..no_crate_rows] {
        let hb: Vec<Option<char>> = crates(crate_instr)?;
        for (i, el) in hb.into_iter().enumerate() {
            if let Some(el) = el {
                buckets[i].push(el);
            }
        }
    }

    for b in buckets.iter_mut() {
        b.reverse();
    }

    for move_instr in &input[(no_crate_rows + 2)..] {
        let (no, from, to): (usize, usize, usize) = movement(move_instr)?;
        let mut taken = (0..no)
            .into_iter()
            .map(|_| {
                buckets[from]
                    .pop()
                    .ok_or_else(|| color_eyre::eyre::eyre!("Empty Stack"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        // taken.reverse(); //For P1, comment this line
        buckets[to].append(&mut taken);
    }

    for mut b in buckets.into_iter() {
        // let mut b: Vec<char> = b.into_iter().flatten().collect();
        print!("{}", b.pop().expect("element to pop"));
    }

    Ok(())
}
