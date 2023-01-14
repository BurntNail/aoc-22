use array2d::Array2D;

#[derive(Copy, Clone, Debug)]
pub enum Node {
    Start,
    End,
    Elevation(u8),
}

impl Node {
    pub const fn elevation(self) -> u8 {
        match self {
            Self::Start => 1,
            Self::End => 26,
            Self::Elevation(x) => x,
        }
    }
}

impl TryFrom<char> for Node {
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
pub struct Grid(Array2D<Node>);

impl TryFrom<String> for Grid {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut rows = vec![];
        for row in value.lines() {
            let row = row
                .chars()
                .map(Node::try_from)
                .collect::<Result<Vec<_>, _>>()?;
            rows.push(row);
        }
        let arr = Array2D::from_rows(&rows).map_err(|e| format!("{e:?}"))?;
        Ok(Self(arr))
    }
}

impl Grid {
    pub fn to_places_i_can_get_to(&self) -> Array2D<Vec<(usize, usize)>> {
        let row_len = self.0.num_rows();
        let col_len = self.0.num_columns();
        let mut edges = Array2D::filled_with(vec![], row_len, col_len);

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

                let elevation_max = self.0.get(row, col).unwrap().elevation() + 1;

                edges
                    .get_mut(row, col)
                    .unwrap()
                    .extend(deltas_to_check.into_iter().filter_map(|(dr, dc)| {
                        let check_row = (row as i32 + dr) as usize;
                        let check_col = (col as i32 + dc) as usize;

                        if self.0.get(check_row, check_col).unwrap().elevation() <= elevation_max {
                            Some((check_row, check_col))
                        } else {
                            None
                        }
                    }));
            }
        }
        edges
    }
}
