use std::env;
use std::collections::HashSet;

use log::debug;

use utils::{read_input, set_logging_level};

#[derive(Debug)]
struct Forest {
    trees: Vec<usize>,
    width: usize
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Tree {
    row: usize,
    col: usize
}

impl Forest {
    pub fn new() -> Forest {
        Forest {trees: Vec::new(), width: 0}
    }

    pub fn add_row(&mut self, line: &str) {
        let line_width = line.len() as usize;
        if self.width == 0 {
            self.width = line_width;
        } else if (self.width > 0) & (self.width != line_width) {
            panic!("Mismatching line width");
        }

        for c in line.chars() {
            let n: usize = c.to_digit(10).unwrap() as usize;
            self.trees.push(n)
        }
    }

    pub fn tree_loc(&self, row: &usize, col: &usize) -> &usize {
        // https://stackoverflow.com/a/2151141/14536215
        // https://en.wikipedia.org/wiki/Row-_and_column-major_order
        &self.trees[(self.width * row + col)]
    }

    pub fn height(&self) ->usize {
        self.trees.len() / self.width
    }

    fn visible_trees_row(&self, row: &usize) -> HashSet<Tree> {
        let mut visible_trees: HashSet<Tree> = HashSet::new();
        let mut tree_i: &usize;
        // Forwards
        let mut highest_tree: &usize = self.tree_loc(row, &0);
        visible_trees.insert(Tree {row: *row, col: 0});
        for col in 1..self.width {
            tree_i = self.tree_loc(row, &col);

            if tree_i > highest_tree {
                visible_trees.insert(Tree {row: *row, col: col});
                highest_tree = tree_i;
            }

            if highest_tree == &9 {
                break;
            }
        }
        // Backwards
        let mut highest_tree: &usize = self.tree_loc(row, &(self.width - 1));
        visible_trees.insert(Tree {row: *row, col: (self.width - 1)});
        for col in (0..self.width - 1).rev() {
            tree_i = self.tree_loc(&row, &col);

            if tree_i > highest_tree {
                visible_trees.insert(Tree {row: *row, col: col});
                highest_tree = tree_i;
            }

            if highest_tree == &9 {
                break;
            }
        }
        debug!("row {} visible: {:?}", row, visible_trees);
        return visible_trees;
    }

    fn visible_trees_col(&self, col: &usize) -> HashSet<Tree> {
        let mut visible_trees: HashSet<Tree> = HashSet::new();
        let mut tree_i: &usize;
        // Forwards
        let mut highest_tree: &usize = self.tree_loc(&0, col);
        visible_trees.insert(Tree {row: 0, col: *col});
        for row in 1..self.height() {
            tree_i = self.tree_loc(&row, col);

            if tree_i > highest_tree {
                visible_trees.insert(Tree {row: row, col: *col});
                highest_tree = tree_i;
            }

            if highest_tree == &9 {
                break;
            }
        }
        // Backwards
        let mut highest_tree: &usize = self.tree_loc(&(self.height() - 1), col);
        visible_trees.insert(Tree {row: (self.height() - 1), col: *col});
        for row in (0..self.height() - 1).rev() {
            tree_i = self.tree_loc(&row, col);

            if tree_i > highest_tree {
                visible_trees.insert(Tree {row: row, col: *col});
                highest_tree = tree_i;
            }

            if highest_tree == &9 {
                break;
            }
        }
        debug!("col {} visible: {:?}", col, visible_trees);
        return visible_trees;
    }

    pub fn visible_trees(&self) -> usize {
        let mut visible_trees: HashSet<Tree> = HashSet::new();
        for row in 0..self.height() {
            visible_trees.extend(self.visible_trees_row(&row));
        }
        for col in 0..self.width {
            visible_trees.extend(self.visible_trees_col(&col));
        }
        return visible_trees.len();
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
    let mut forest = Forest::new();

    for line in input.lines() {
        forest.add_row(line)
    }
    debug!("{:?}", forest);
    debug!("N trees: {}", forest.trees.len());
    debug!("(3, 4) {}", forest.tree_loc(&3, &4));
    debug!("(4, 4) {}", forest.tree_loc(&4, &4));
    println!("Width: {}", forest.width);
    println!("Height: {}", forest.height());
    let visible = forest.visible_trees();
    println!("Visible trees: {visible}")
}
