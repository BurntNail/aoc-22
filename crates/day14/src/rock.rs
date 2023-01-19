use std::fmt::Display;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self as cc},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

///Row, Col NOT x, y
pub type Coords = (usize, usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Rock,
    FallenSand,
    Nothing,
}

impl State {
    pub fn is_solid(self) -> bool {
        self != Self::Nothing
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Rock => '#',
            Self::FallenSand => '.',
            Self::Nothing => ' ',
        };
        write!(f, "{c}")
    }
}

pub struct Line(pub Vec<Coords>);

fn parse_coords(input: &str) -> IResult<&str, Coords> {
    let (input, (x, _, y)) = tuple((
        map(cc::u128, |x| x as usize),
        tag(","),
        map(cc::u128, |x| x as usize),
    ))(input)?;

    Ok((input, (y, x))) //to convert to row,col format
}

impl Line {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, v) = separated_list0(tag(" -> "), parse_coords)(input)?;

        Ok((input, Self(v)))
    }

    pub fn parse_all(input: &str) -> IResult<&str, Vec<Self>> {
        let v = input
            .lines()
            .map(|x| x.trim_end().trim())
            .map(|x| Self::parse(x).map(|x| x.1))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(("", v))
    }

    pub fn into_interior(self) -> Vec<Coords> {
        self.0
            .windows(2)
            .flat_map(|w| {
                let w: [Coords; 2] = w.try_into().expect("windows done messed up");
                let [(a_row, a_col), (b_row, b_col)] = w;

                match (a_row == b_row, a_col == b_col) {
                    (true, true) => vec![(a_row, a_col)],
                    (true, false) => {
                        //same row, different column
                        let start = a_col.min(b_col);
                        let end = a_col.max(b_col);
                        (start..=end).into_iter().map(|col| (a_row, col)).collect()
                    }
                    (false, true) => {
                        //different row, same column
                        let start = a_row.min(b_row);
                        let end = a_row.max(b_row);
                        (start..=end).into_iter().map(|row| (row, a_col)).collect()
                    }
                    (false, false) => panic!("yeet "),
                }
            })
            .unique()
            .collect()
    }
}
