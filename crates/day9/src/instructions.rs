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
#[derive(Copy, Clone, Debug)]
pub struct Instruction(Direction, usize);

impl From<String> for Instruction {
    fn from(mut value: String) -> Self {
        use Direction::*;
        let dir = match value.remove(0) {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => unreachable!("bad input"),
        };
        value.remove(0);
        Self(dir, value.parse().expect("unable to parse number"))
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
                .map(|s| s.to_string())
                .map(Instruction::from)
                .collect(),
        )
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
