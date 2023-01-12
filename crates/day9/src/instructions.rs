use std::{
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction(pub Direction, pub i32);

impl From<String> for Instruction {
    fn from(mut value: String) -> Self {
        use Direction::{Down, Left, Right, Up};
        let dir = match value.remove(0) {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => unreachable!("bad input"),
        };
        value.remove(0);
        Self(
            dir,
            i32::from(value.parse::<u16>().expect("unable to parse number")),
        )
    }
}

impl Instruction {
    #[allow(clippy::cast_sign_loss)] //checked in function
    pub fn to_singles(self) -> Vec<Direction> {
        if self.1 < 0 {
            vec![self.0.opposite(); self.1.unsigned_abs() as usize]
        } else {
            vec![self.0; self.1 as usize]
        }
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Program(Vec<Instruction>);

impl From<String> for Program {
    fn from(value: String) -> Self {
        Self(
            value
                .lines()
                .map(std::string::ToString::to_string)
                .map(Instruction::from)
                .collect(),
        )
    }
}

impl Program {
    pub fn into_singles(self) -> Vec<Direction> {
        self.0
            .into_iter()
            .flat_map(Instruction::to_singles)
            .collect()
    }
}

impl Deref for Program {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Program {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl IntoIterator for Program {
    type Item = Instruction;
    type IntoIter = IntoIter<Instruction>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
