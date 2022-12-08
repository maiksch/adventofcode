use std::str::FromStr;

struct Grid {
    grid: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn get_height(&self, x: usize, y: usize) -> u32 {
        self.grid[y][x]
    }

    fn get_scenic_score(&self, x: usize, y: usize) -> usize {
        let mut score = (0, 0, 0, 0);
        let tree_height = self.get_height(x, y);

        // check left
        if x != 0 {
            for x_compare in (0..x).rev() {
                score.0 += 1;
                if self.get_height(x_compare, y) >= tree_height {
                    break;
                }
            }
        }

        // check up
        if y != 0 {
            for y_compare in (0..y).rev() {
                score.1 += 1;
                if self.get_height(x, y_compare) >= tree_height {
                    break;
                }
            }
        }

        // check right
        if x != self.width - 1 {
            for x_compare in x + 1..self.width {
                score.2 += 1;
                if self.get_height(x_compare, y) >= tree_height {
                    break;
                }
            }
        }

        // check down
        if y != self.height - 1 {
            for y_compare in y + 1..self.height {
                score.3 += 1;
                if self.get_height(x, y_compare) >= tree_height {
                    break;
                }
            }
        }

        score.0 * score.1 * score.2 * score.3
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree_height = self.get_height(x, y);
        let mut high;

        // check left
        high = (0..x)
            .map(|x| self.get_height(x, y))
            .max()
            .expect("has max from left");
        if tree_height > high {
            return true;
        }

        // check up
        high = (0..y)
            .map(|y| self.get_height(x, y))
            .max()
            .expect("has max from top");
        if tree_height > high {
            return true;
        }

        // check right
        high = (x + 1..self.width)
            .map(|x| self.get_height(x, y))
            .max()
            .expect("has max from right");
        if tree_height > high {
            return true;
        }

        // check down
        high = (y + 1..self.height)
            .map(|y| self.get_height(x, y))
            .max()
            .expect("has max from bottom");
        if tree_height > high {
            return true;
        }

        false
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Vec<u32>> = vec![];
        for y in s.lines() {
            let mut line = vec![];
            for x in y.chars() {
                let x = x.to_digit(10).expect("yep");
                line.push(x);
            }
            grid.push(line);
        }
        Ok(Grid {
            height: grid.len(),
            width: grid.first().expect("has first").len(),
            grid,
        })
    }
}

fn part_one(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Grid is valid");
    let mut visible_trees = grid.height * 2 + grid.width * 2 - 4;
    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            if grid.is_visible(x, y) {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn part_two(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Grid is valid");
    let mut highest_scenic_score = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let scenic_score = grid.get_scenic_score(x, y);
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    highest_scenic_score
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day_08.txt").expect("that input exists");
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
