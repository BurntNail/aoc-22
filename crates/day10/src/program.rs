#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    NoOp,
    PreAdd,
    AddX(i32),
}
impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match &value[0..4] {
            "noop" => Self::NoOp,
            "addx" => Self::AddX(value[5..].parse().expect("failed to parse")),
            _ => panic!("failed to parse instruction"),
        }
    }
}
impl Instruction {
    pub fn to_preadd(self) -> Vec<Self> {
        match self {
            Self::NoOp => vec![Self::NoOp],
            Self::AddX(x) => vec![Self::PreAdd, Self::AddX(x)],
            Self::PreAdd => vec![Self::PreAdd],
        }
    }
}

pub struct Program {
    x: i32,
    instructions: Vec<Instruction>,
}
impl From<String> for Program {
    fn from(value: String) -> Self {
        Self {
            x: 1,
            instructions: value
                .lines()
                .map(Instruction::from)
                .flat_map(Instruction::to_preadd)
                .collect(),
        }
    }
}

impl Program {
    //returns the state of the register x at all times
    pub fn run(mut self) -> Vec<i32> {
        let mut signals = vec![];

        for instr in self.instructions {
            signals.push(self.x);
            match instr {
                Instruction::NoOp | Instruction::PreAdd => {}
                Instruction::AddX(change) => self.x += change,
            }
        }

        signals
    }

    pub fn to_signal_strengths(x: Vec<i32>, start: usize, multiples: usize) -> Vec<i32> {
        let mut signals = Vec::with_capacity((x.len() - start) / multiples + 1);

        for (i, x) in x.into_iter().enumerate() {
            let i = i + 1; //account for silly humans
            if i < start {
                continue;
            }
            if i == start || (i > start && (i - start) % multiples == 0) {
                signals.push(x * i as i32);
            }
        }

        signals
    }

    pub fn part_2 (x: Vec<i32>, width: usize) {

        let mut screen_pos = 0;

        for x in x {

            if (screen_pos as i32 - x).abs() <= 1 {
                print!("#")
            } else {
                print!(".")
            }

            if screen_pos == width - 1 {
                println!();
                screen_pos = 0;
            } else {
                screen_pos += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn from_string_instr_tester() {
        let pf = |x| Instruction::from(x);
        assert_eq!(pf("addx 1"), Instruction::AddX(1));
        assert_eq!(pf("addx 654"), Instruction::AddX(654));
        assert_eq!(pf("addx -1"), Instruction::AddX(-1));
        assert_eq!(pf("addx -112"), Instruction::AddX(-112));
        assert_eq!(pf("noop"), Instruction::NoOp);
    }
}
