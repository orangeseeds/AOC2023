use std::fs;

#[derive(Debug)]
struct Cell {
    val: i64,
    visited: bool,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    fn build(content: &str) -> Result<Grid, String> {
        let mut rows = Vec::new();
        for ln in content.lines() {
            let mut cols: Vec<Cell> = Vec::new();
            for c in ln.chars() {
                let val: i64 = match c {
                    '.' => -1,
                    token if token.is_ascii_digit() => {
                        if let Some(n) = token.to_digit(10) {
                            n.into()
                        } else {
                            return Err(format!("Error occured when parsing {token}"));
                        }
                    }
                    '*' => -3,
                    _ => -2,
                };
                cols.push(Cell {
                    val: val,
                    visited: false,
                });
            }
            rows.push(cols);
        }
        return Ok(Grid { grid: rows });
    }

    fn get_full_num(&mut self, i: usize, j: usize) -> i64 {
        if self.grid[i][j].visited || self.grid[i][j].val < 0 {
            return 0;
        }
        let (mut r, mut l) = (j, j - 1);
        let mut full_num = Vec::new();

        while r < self.grid[i].len() && self.grid[i][r].val >= 0 {
            self.grid[i][r].visited = true;
            full_num.push(self.grid[i][r].val);
            r += 1
        }
        while l >= 0 && self.grid[i][l].val >= 0 {
            self.grid[i][l].visited = true;
            full_num.insert(0, self.grid[i][l].val);
            if l == 0 {
                break;
            }
            l -= 1;
        }
        let mut sum = 0;

        // println!("{:?}", full_num);
        for n in full_num {
            sum = sum * 10 + n;
        }
        // println!("{sum}");
        return sum;
    }

    fn search_cogs(&mut self) -> Vec<Vec<(usize, usize)>> {
        let mut cells = Vec::new();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let mut cogs: Vec<(usize, usize)> = Vec::new();
                if col.val == -3 {
                    // println!("symbol at {i},{j}");

                    if i > 0 {
                        cogs.push((i - 1, j)); // Top
                        if j > 0 {
                            cogs.push((i - 1, j - 1)); // Top Left
                        }
                        if j < self.grid[i - 1].len() - 1 {
                            cogs.push((i - 1, j + 1)); // Top Right
                        }
                    }

                    if i < self.grid.len() - 1 {
                        cogs.push((i + 1, j)); // Bottom
                        if j > 0 {
                            cogs.push((i + 1, j - 1)); // Bottom Left
                        }
                        if j < self.grid[i + 1].len() - 1 {
                            cogs.push((i + 1, j + 1)); // Bottom Right
                        }
                    }

                    if j > 0 {
                        cogs.push((i, j - 1)); // Left
                    }
                    if j < self.grid[i].len() - 1 {
                        cogs.push((i, j + 1)); // Right
                    }

                    if cogs.len() > 1 {
                        cells.push(cogs);
                    }
                }
            }
        }
        return cells;
    }

    fn search(&mut self) -> Vec<(usize, usize)> {
        let mut adj_cells = Vec::new();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col.val == -2 || col.val == -3 {
                    // println!("symbol at {i},{j}");

                    if i > 0 {
                        adj_cells.push((i - 1, j)); // Top
                        if j > 0 {
                            adj_cells.push((i - 1, j - 1)); // Top Left
                        }
                        if j < self.grid[i - 1].len() - 1 {
                            adj_cells.push((i - 1, j + 1)); // Top Right
                        }
                    }

                    if i < self.grid.len() - 1 {
                        adj_cells.push((i + 1, j)); // Bottom
                        if j > 0 {
                            adj_cells.push((i + 1, j - 1)); // Bottom Left
                        }
                        if j < self.grid[i + 1].len() - 1 {
                            adj_cells.push((i + 1, j + 1)); // Bottom Right
                        }
                    }

                    if j > 0 {
                        adj_cells.push((i, j - 1)); // Left
                    }
                    if j < self.grid[i].len() - 1 {
                        adj_cells.push((i, j + 1)); // Right
                    }
                }
            }
        }
        return adj_cells;
    }
}

pub fn solve_part_two() {
    let path: &str = "./data/three.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    match Grid::build(&contents) {
        Ok(mut val) => {
            // println!("{:?}", val.grid);
            let mut sum = 0;
            let adj_cells = val.search_cogs();
            for pos in adj_cells {
                let mut prod = 1;
                let mut count = 0;
                for p in pos {
                    let val = val.get_full_num(p.0, p.1);
                    if val != 0 {
                        count += 1;
                        prod *= val;
                    }
                }
                if count > 1 {
                    sum += prod;
                }
            }
            println!("Day three part_two sum: {}", sum);
        }
        Err(err) => {
            println!("Error: {err}");
        }
    };
}

pub fn solve_part_one() {
    let path: &str = "./data/three.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    match Grid::build(&contents) {
        Ok(mut val) => {
            let mut sum = 0;
            let adj_cells = val.search();
            for pos in adj_cells {
                let val = val.get_full_num(pos.0, pos.1);
                sum += val;
            }
            println!("Day three part_one sum: {}", sum);
        }
        Err(err) => {
            println!("Error: {err}");
        }
    };
}
