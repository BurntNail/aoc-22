use crate::Coord;
use array2d::Array2D;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tree {
    Start,
    End,
    Elevation(u8),
}

impl Tree {
    pub const fn elevation(self) -> u8 {
        match self {
            Self::Start => 1,
            Self::End => 26,
            Self::Elevation(x) => x,
        }
    }
}

impl TryFrom<char> for Tree {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            'a'..='z' => Ok(Self::Elevation(value as u8 - b'a' + 1)),
            _ => Err(value.to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Grid(Array2D<Tree>);

impl TryFrom<String> for Grid {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut rows = vec![];
        for row in value.lines() {
            let row = row
                .chars()
                .map(Tree::try_from)
                .collect::<Result<Vec<_>, _>>()?;
            rows.push(row);
        }
        let arr = Array2D::from_rows(&rows).map_err(|e| format!("{e:?}"))?;
        Ok(Self(arr))
    }
}

impl Grid {
    #[allow(clippy::cast_sign_loss)]
    ///returns the position of the end as well
    pub fn to_places_i_can_get_to(&self) -> (Coord, Array2D<Vec<Coord>>) {
        let row_len = self.0.num_rows();
        let col_len = self.0.num_columns();

        let mut edges = Array2D::filled_with(vec![], row_len, col_len);
        let mut end = Default::default();

        for row in 0..row_len {
            for col in 0..col_len {
                let mut deltas_to_check = vec![];

                if row > 0 {
                    deltas_to_check.push((-1, 0));
                }
                if row < row_len - 1 {
                    deltas_to_check.push((1, 0));
                }
                if col > 0 {
                    deltas_to_check.push((0, -1));
                }
                if col < col_len - 1 {
                    deltas_to_check.push((0, 1));
                }

                let node_here = *self.0.get(row, col).unwrap();
                if node_here == Tree::End {
                    end = Some((row, col));
                }

                let elevation_min = node_here.elevation() - 1;

                edges
                    .get_mut(row, col)
                    .unwrap()
                    .extend(deltas_to_check.into_iter().filter_map(|(dr, dc)| {
                        let check_row = (row as i32 + dr) as usize;
                        let check_col = (col as i32 + dc) as usize;

                        if self.0.get(check_row, check_col).unwrap().elevation() >= elevation_min {
                            Some((check_row, check_col))
                        } else {
                            None
                        }
                    }));
            }
        }
        (end.unwrap(), edges)
    }

    pub fn get_starting_locations(&self, needs_to_be_start: bool) -> Vec<Coord> {
        let mut v = vec![];

        for row in 0..self.0.num_rows() {
            for col in 0..self.0.num_columns() {
                let this = *self.0.get(row, col).unwrap();
                if !needs_to_be_start && this == Tree::Elevation(1)
                    || needs_to_be_start && this == Tree::Start
                {
                    v.push((row, col));
                }
            }
        }

        v
    }
}
