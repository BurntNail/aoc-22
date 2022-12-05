#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub const fn score_vs(self, o: Self) -> i32 {
        use RPS::{Paper, Rock, Scissors};

        match (self, o) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
            (Rock, Rock) | (Scissors, Scissors) | (Paper, Paper) => 3,
            (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => 0,
        }
    }
    pub const fn to_i32(self) -> i32 {
        //Would've liked to use Into::into but wasn't cooperating
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    pub const fn to_rps(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }

    ///Assuming AX = Rock, BY = Paper, CZ = Scissors
    pub const fn to_two_rpss((a, b): (char, char)) -> (Self, Self) {
        let m = Self::to_rps;
        (m(a), m(b))
    }
    pub fn move_from_wdl((a, b): (Self, char)) -> Self {
        let points_target = match b {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => unreachable!(),
        };

        for t in [Self::Rock, Self::Paper, Self::Scissors] {
            if t.score_vs(a) == points_target {
                return t;
            }
        }

        unreachable!()
    }
}

fn main() {
    let input = include_str!("input.txt");

    let (scores, scores_wdl) = input
        .lines()
        .map(|l| {
            let chars: Vec<char> = l.trim().chars().collect();
            let (their, my) = RPS::to_two_rpss((chars[0], chars[2]));
            let my_wdl = RPS::move_from_wdl((their, chars[2]));

            (
                my.score_vs(their) + my.to_i32(),
                my_wdl.score_vs(their) + my_wdl.to_i32(),
            )
        })
        .reduce(|accum, item| (accum.0 + item.0, accum.1 + item.1))
        .unwrap();

    println!("Total Score: {}", scores);
    println!("Total Score WDL: {}", scores_wdl);
}
