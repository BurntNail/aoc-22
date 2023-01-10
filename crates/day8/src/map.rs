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

                let mut score = 1; //assumption - everyone can see at least 1 tree. likely unimportant for top scoring tree

                let range_left = 0..col;
                let range_right = col + 1..cl;
                if col != 0 {
                    let left_score = range_left
                        .clone()
                        .rev()
                        .map(|col_test| self.0[(row, col_test)] < height)
                        .position(|x| !x)
                        .unwrap_or_else(|| range_left.count());
                    if left_score != 0 {
                        score *= left_score;
                    }
                }
                if col != cl - 1 {
                    let right_score = range_right
                        .clone()
                        .map(|col_test| self.0[(row, col_test)] < height)
                        .position(|x| !x)
                        .unwrap_or_else(|| range_right.count());
                    if right_score != 0 {
                        score *= right_score;
                    }
                }

                let range_up = 0..row;
                let range_down = row + 1..rl;
                if row != 0 {
                    let up_score = range_up
                        .clone()
                        .rev()
                        .map(|row_test| self.0[(row_test, col)] < height)
                        .position(|x| !x)
                        .unwrap_or_else(|| range_up.count());
                    if up_score != 0 {
                        score *= up_score;
                    }
                }
                if row != rl - 1 {
                    let down_score = range_down
                        .clone()
                        .map(|row_test| self.0[(row_test, col)] < height)
                        .position(|x| !x)
                        .unwrap_or_else(|| range_down.count());
                    if down_score != 0 {
                        score *= down_score;
                    }
                }

                println!("{row},{col} is {score}");

                sc.push(score);
            }
        }

        sc
    }
}
