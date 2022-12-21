use std::fs;
use std::fmt;

#[derive(Debug)]
struct TreeGrid {
    size: usize,
    grid: Vec<u64>
}

impl TreeGrid {
    fn new() -> TreeGrid {
        TreeGrid {
            size: 0,
            grid: Vec::new()
        }
    }

    fn insert_item(&mut self, item: u64) {
        self.grid.push(item);
    }

    fn get_item(&self, x: usize, y: usize) -> u64 {
        self.grid[x * self.size + y]
    }
}

struct VisibilityGrid {
    size: usize,
    grid: Vec<bool>
}

impl VisibilityGrid {
    fn new(size: usize) -> VisibilityGrid {
        VisibilityGrid { 
            size,
            grid: vec![false; size * size]
        }
    }

    fn init(&mut self) {
        (0..self.size).for_each(|i| {
            self.set_visible(0, i);
            self.set_visible(self.size - 1, i);
            self.set_visible(i, 0);
            self.set_visible(i, self.size - 1)
        })
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        self.grid[x * self.size + y] 
    }

    fn set_visible(&mut self, x: usize, y: usize) {
        self.grid[x * self.size + y] = true;
    }

    fn get_visible_count(&self) -> u64 {
        self.grid.iter().map(|x| if *x { 1 } else { 0 }).sum()
    }
}

impl fmt::Debug for VisibilityGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (0..self.size).for_each(|x| {
            (0..self.size).for_each(|y| {
                let marker = if self.is_visible(x, y) { 'O' } else {'X'};
                if y != self.size - 1 {
                    write!(f, "{}", marker).unwrap();
                } else {
                        writeln!(f, "{}", marker).unwrap(); 
                }
            })
        });
        fmt::Result::Ok(())
    }
}

fn read_tree_grid(contents: &str) -> TreeGrid {
    let mut tree_grid = TreeGrid::new();
    contents.split('\n').for_each(|row| {
        row.chars().for_each(|x| tree_grid.insert_item(x.to_digit(10).unwrap() as u64));
        tree_grid.size += 1;
    });
    tree_grid
}

fn set_visible_trees(i: usize, tree_grid: &TreeGrid, visibility_grid: &mut VisibilityGrid) {
    let mut max_left = tree_grid.get_item(i, 0);
    let mut max_right = tree_grid.get_item(i, tree_grid.size - 1);
    let mut max_top = tree_grid.get_item(0, i);
    let mut max_bot = tree_grid.get_item(tree_grid.size - 1, i);
    (1..tree_grid.size - 1).for_each(|mut j| {
        if tree_grid.get_item(i, j) > max_left {
            visibility_grid.set_visible(i, j);
            max_left = tree_grid.get_item(i, j);
        }
        if tree_grid.get_item(j, i) > max_top {
            visibility_grid.set_visible(j, i);
            max_top = tree_grid.get_item(j, i);
        }

        j = tree_grid.size - j - 1;
        if tree_grid.get_item(i, j) > max_right {
            visibility_grid.set_visible(i, j);
            max_right = tree_grid.get_item(i, j);
        }
        if tree_grid.get_item(j, i) > max_bot {
            visibility_grid.set_visible(j, i);
            max_bot = tree_grid.get_item(j, i);
        }
    });
}

fn count_visible_trees(tree_grid: &TreeGrid, row: usize, col: usize) -> u64 {
    let mut visible_trees = 1u64;
    let cur_val = tree_grid.get_item(row, col);
    // check up
    visible_trees *= (0..row).rev().enumerate().skip_while(|(_, x)| {
        tree_grid.get_item(*x, col) < cur_val
    }).next().unwrap_or((row - 1, 0)).0 as u64 + 1;

    // check down
    visible_trees *= (row + 1..tree_grid.size).enumerate().skip_while(|(_, x)| {
        tree_grid.get_item(*x, col) < cur_val
    }).next().unwrap_or((tree_grid.size - row - 2, 0)).0 as u64 + 1;

    // check left
    visible_trees *= (0..col).rev().enumerate().skip_while(|(_, y)| {
        tree_grid.get_item(row, *y) < cur_val
    }).next().unwrap_or((col - 1, 0)).0 as u64 + 1;

    // check right
    visible_trees *= (col + 1..tree_grid.size).enumerate().skip_while(|(_, y)| {
        tree_grid.get_item(row, *y) < cur_val
    }).next().unwrap_or((tree_grid.size - col - 2, 0)).0 as u64 + 1;
    
    visible_trees
}

fn main() {
    let input_file_path = "src/bin/day_8/full.input";
    let contents = fs::read_to_string(input_file_path).unwrap();
    let tree_grid = read_tree_grid(&contents);
    println!("Part 1: {}", part_1(&tree_grid));
    println!("Part 2: {}", part_2(&tree_grid));
}

fn part_1(tree_grid: &TreeGrid) -> u64 {
    let mut visibility_grid = VisibilityGrid::new(tree_grid.size);
    visibility_grid.init();
    (1..tree_grid.size - 1).for_each(|row| {
        set_visible_trees(row, tree_grid, &mut visibility_grid);
    });
    visibility_grid.get_visible_count()
}

fn part_2(tree_grid: &TreeGrid) -> u64 {
    (1..tree_grid.size - 1).map(|x| {
        (1..tree_grid.size - 1).map(|y| {
            count_visible_trees(tree_grid, x, y)
        }).max().unwrap()
    }).max().unwrap()
}