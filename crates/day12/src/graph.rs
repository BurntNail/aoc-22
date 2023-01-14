use crate::Coord;
use array2d::Array2D;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
pub struct NodeRecord(Option<Coord>);

pub struct Graph {
    can_reach: Array2D<Vec<Coord>>,
    start: Coord,
    end: Coord,
}

impl Graph {
    pub const fn new(can_reach: Array2D<Vec<Coord>>, start: Coord, end: Coord) -> Self {
        Self {
            can_reach,
            start,
            end,
        }
    }

    pub fn find_end(self) -> usize {
        let mut visited = HashSet::new();
        let mut current = HashMap::new();
        let mut steps = 0;

        visited.insert(self.start);
        current.insert(self.start, NodeRecord(None));

        while !visited.contains(&self.end) {
            steps += 1;

            for cell in visited.clone() {
                for neighbour in self.can_reach.get(cell.0, cell.1).unwrap() {
                    if !visited.contains(neighbour) {
                        visited.insert(*neighbour);
                        current.insert(*neighbour, NodeRecord(Some(*neighbour)));
                    }
                }
            }
        }

        steps
    }
}
