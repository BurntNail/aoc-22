use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::i64, combinator::map, sequence::tuple, IResult,
};
use std::{
    collections::HashSet,
    ops::{Range, RangeInclusive},
};

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

    // ///All covered coordinates
    // pub fn to_no_in_row(self, row: Int) -> usize {
    //     let mut set = Vec::new();
    //
    //     for ((sensor_x, sensor_y), beacon) in self.0 {
    //         let sensor = (sensor_x, sensor_y);
    //
    //         println!("Checking {sensor:?} and {beacon:?}");
    //
    //         let md = manhattan(sensor, beacon);
    //
    //         let min_y = (sensor_y - md);
    //         let max_y = (sensor_y + md);
    //
    //         let iter = (min_y..=max_y).flat_map(|covered_y| {
    //             if covered_y % 10_000 == 0 {
    //                 println!("Doing {covered_y}");
    //             }
    //             let min_x = (sensor_x - md);
    //             let max_x = (sensor_x + md);
    //             (min_x..=max_x)
    //                 .map(move |covered_x| (covered_x, covered_y))
    //                 .filter(|covered| manhattan(*covered, sensor) <= md)
    //         });
    //
    //         set.extend(iter);
    //     }
    //
    //     set.into_iter().filter(|(_x, y)| y == &row).unique().count() - 1
    // }
    //
    // ///All covered coordinates
    // pub fn find_empty(self, max: Int) -> Coord {
    //     let mut set = vec![];
    //
    //     for ((sensor_x, sensor_y), beacon) in self.0 {
    //         let sensor = (sensor_x, sensor_y);
    //
    //         println!("Checking {sensor:?} and {beacon:?}");
    //
    //         let md = manhattan(sensor, beacon);
    //
    //         let min_y = (sensor_y - md).clamp(0, max);
    //         let max_y = (sensor_y + md).clamp(0, max);
    //
    //         let iter = (min_y..=max_y)
    //             .filter(|covered_y| covered_y < &0 || covered_y > &max)
    //             .flat_map(|covered_y| {
    //                 let min_x = (sensor_x - md).clamp(0, max);
    //                 let max_x = (sensor_x + md).clamp(0, max);
    //                 (min_x..=max_x)
    //                     .map(move |covered_x| (covered_x, covered_y))
    //                     .filter(|(covered_x, _y)| covered_x > &max || covered_x < &0)
    //                     .filter(|covered| manhattan(*covered, sensor) <= md)
    //             });
    //
    //         set.extend(iter);
    //     }
    // }

    fn ranges(
        self,
        y: Int,
        x_range: Option<RangeInclusive<Int>>,
    ) -> impl Iterator<Item = RangeInclusive<Int>> {
        //https://fasterthanli.me/series/advent-of-code-2022/part-15#range-based-strategy
        let mut ranges = vec![];

        for (sensor, beacon) in self.0 {
            let md = manhattan(sensor, beacon);

            let y_dist = (y - sensor.1).abs();
            if y_dist > md {
                continue;
            }

            let d = md - y_dist;
            let middle = sensor.0;

            let trial_r = (middle - d)..=(middle + d);
            let fits_in_range = {
                if let Some(x_range) = x_range.clone() {
                    let r =
                        *trial_r.start().max(x_range.start())..=*trial_r.end().min(x_range.end());
                    r.start() < r.end()
                } else {
                    true
                }
            };
            if fits_in_range {
                ranges.push(trial_r);
            }
        }
        ranges.sort_by_key(|r| *r.start());

        ranges.into_iter().coalesce(|a, b| {
            if b.start() - 1 < *a.end() {
                if b.end() > a.end() {
                    Ok(*a.start()..=*b.end())
                } else {
                    Ok(a)
                }
            } else {
                Err((a, b))
            }
        })
    }

    pub fn to_num_in_row(self, y: Int) -> usize {
        let beacon_x = self
            .0
            .iter()
            .filter_map(|(_, (beacon_x, beacon_y))| {
                if *beacon_y == y {
                    Some(*beacon_x)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        self.ranges(y, None)
            .map(|r| {
                (r.end() - r.start() + 1) as usize
                    - beacon_x.iter().filter(|x| r.contains(x)).count()
            })
            .sum::<usize>()
    }

    pub fn beacon_position(
        self,
        x_range: RangeInclusive<Int>,
        mut y_range: RangeInclusive<Int>,
    ) -> Option<Coord> {
        y_range.find_map(|y| {
            self.clone()
                .ranges(y, Some(x_range.clone()))
                .nth(1)
                .map(|range| (range.start() - 1, y))
        })
    }
}

fn manhattan(a: Coord, b: Coord) -> Int {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
