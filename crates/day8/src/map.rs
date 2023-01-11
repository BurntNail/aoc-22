use array2d::Array2D;

type IntItem = u8;

#[derive(Clone, Debug)]
pub struct Grid(Array2D<IntItem>);

impl From<String> for Grid {
    fn from(value: String) -> Self {
        let mut grid = vec![];
        for line in value.lines() {
            let line: Vec<IntItem> = line
                .chars()
                .map(|x| x.to_string().parse().expect("failure to parse char"))
                .collect();
            grid.push(line);
        }
        Self(Array2D::from_rows(&grid).expect("failure to convert to arr2d"))
    }
}

impl Grid {
    pub fn can_see_edge(&self) -> usize {
        let rl = self.0.row_len();
        let cl = self.0.column_len();

        let mut count = rl * 2 + cl * 2 - 4;

        for col in 1..cl - 1 {
            for row in 1..rl - 1 {
                let height = self.0[(row, col)];

                if height == 0 {
                    continue;
                }

                let mut works = false;

                //go horizontally
                let mut range_left = 0..col;
                let mut range_right = col + 1..cl;
                works |= range_left.all(|col_test| self.0[(row, col_test)] < height);
                works |= range_right.all(|col_test| self.0[(row, col_test)] < height);

                //go vertically
                let mut range_up = 0..row;
                let mut range_down = row + 1..rl;
                works |= range_up.all(|row_test| self.0[(row_test, col)] < height);
                works |= range_down.all(|row_test| self.0[(row_test, col)] < height);

                if works {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn scenic_scores(&self) -> Vec<usize> {
        let rl = self.0.row_len();
        let cl = self.0.column_len();

        let mut sc = vec![];
        for col in 0..cl {
            for row in 0..rl {
                let height = self.0[(row, col)];

                if height == 0 {
                    sc.push(0);
                    continue;
                }

                let mut score = 1;

                let test_row = |row_test: usize| self.0[(row_test, col)] < height;
                let test_col = |col_test: usize| self.0[(row, col_test)] < height;

                let mut test_get_score = |range: Vec<usize>, test_func: &dyn Fn(usize) -> bool| {
                    let this_score = range
                        .clone()
                        .into_iter()
                        .map(test_func)
                        .position(|x| !x)
                        .map(|x| x + 1)
                        .unwrap_or_else(|| range.len());
                    if this_score != 0 {
                        score *= this_score;
                    }
                };

                test_get_score((0..col).rev().into_iter().collect::<Vec<_>>(), &test_col); //left
                test_get_score(((col + 1)..cl).into_iter().collect::<Vec<_>>(), &test_col); //right
                test_get_score((0..row).rev().into_iter().collect::<Vec<_>>(), &test_row); //up
                test_get_score(((row + 1)..rl).into_iter().collect::<Vec<_>>(), &test_row); //down

                sc.push(score);
            }
        }

        sc
    }
}
