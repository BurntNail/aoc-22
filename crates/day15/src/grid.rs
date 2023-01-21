use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::i64, combinator::map, sequence::tuple, IResult,
};
use std::collections::HashSet;

pub type Int = i64;

///X, Y
pub type Coord = (Int, Int);
fn pi(input: &str) -> IResult<&str, Int> {
    i64(input)
}

#[derive(Clone, Debug)]
pub struct Grid(HashSet<(Coord, Coord)>);

impl Grid {
    fn row_col(input: &str) -> IResult<&str, Coord> {
        map(tuple((tag("x="), pi, tag(", y="), pi)), |(_, x, _, y)| {
            (x, y)
        })(input)
    }
    fn line(input: &str) -> IResult<&str, (Coord, Coord)> {
        map(
            tuple((
                tag("Sensor at "),
                Self::row_col,
                tag(": closest beacon is at "),
                Self::row_col,
            )),
            |(_, rc_1, _, rc_2)| (rc_1, rc_2),
        )(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let mut map = HashSet::new();

        for l in input.lines().map(|l| l.trim().trim_end()) {
            map.insert(Self::line(l)?.1);
        }

        Ok(("", Self(map)))
    }

    ///All covered coordinates
    pub fn to_no_in_row(self, row: Int) -> usize {
        let mut set = Vec::new();

        for ((sensor_x, sensor_y), beacon) in self.0 {
            let sensor = (sensor_x, sensor_y);

            println!("Checking {sensor:?} and {beacon:?}");

            let md = manhattan(sensor, beacon);

            for dx in -md..=md {
                let dy = sensor_y - row;

                let new = (sensor_x + dx, sensor_y + dy);

                if manhattan(sensor, new) > md {
                    continue;
                }

                set.push(new.0);
            }
        }

        set.into_iter().unique().count() - 1
    }
}

fn manhattan(a: Coord, b: Coord) -> Int {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
