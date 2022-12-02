#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn score_vs (&self, o: RPS) -> i32 {
        use RPS::{Rock, Paper, Scissors};

        match (*self, o) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
            (Rock, Rock) | (Scissors, Scissors) | (Paper, Paper) => 3,
            (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => 0
        }
    }
    fn to_i32 (&self) -> i32 { //Would've liked to use Into::into but wasn't cooperating
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn to_rps (c: char) -> Self {
        match c {
            'A' | 'X' => {
                Self::Rock
            },
            'B' | 'Y' => {
                Self::Paper
            },
            'C' | 'Z' => {
                Self::Scissors
            },
            _ => unreachable!()
        }
    }

    ///Assuming AX = Rock, BY = Paper, CZ = Scissors
    fn to_two_rpss ((a, b): (char, char)) -> (Self, Self) {
        let m  = Self::to_rps;
        (m(a), m(b))
    }
    fn to_two_rpss_wdl ((a, b): (char, char)) -> (Self, Self) {
        let a = Self::to_rps(a);
        let points_target = match b {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => unreachable!()
        };

        for t in [Self::Rock, Self::Paper, Self::Scissors] {
            if t.score_vs(a) == points_target {
                return (a, t);
            }
        }

        unreachable!()
    }
}

fn main() {
    let input = include_str!("input.txt");

    let scores: Vec<i32> = input.lines().map(|l| {
        let mut chars: Vec<char> = l.trim().chars().collect();
        let (their, my) = RPS::to_two_rpss((chars.remove(0), chars.remove(1)));

        my.score_vs(their) + my.to_i32()
    }).collect();
    let scores_wdl: Vec<i32> = input.lines().map(|l| {
        let mut chars: Vec<char> = l.trim().chars().collect();
        let (their, my) = RPS::to_two_rpss_wdl((chars.remove(0), chars.remove(1)));

        my.score_vs(their) + my.to_i32()
    }).collect();

    println!("Total Score: {}", scores.iter().sum::<i32>());
    println!("Total Score WDL: {}", scores_wdl.iter().sum::<i32>());
}
