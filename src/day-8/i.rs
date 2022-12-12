use std::{collections::HashSet, fs};

// mod util;
// use util::*;

#[derive(Debug)]
struct Tree {
    pub y: usize,
    pub x: usize,
    pub h: u8,
}

impl Tree {
    pub fn new(y: usize, x: usize, h: u8) -> Self {
        Self { y, x, h }
    }
}

fn main() {
    let mut total: u32 = 0;

    let input = fs::read_to_string("i.in").unwrap();
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    let mut forest: Vec<Vec<Tree>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tree::new(y, x, c.to_digit(10).unwrap() as u8))
                .collect()
        })
        .collect();

    let forest_len = forest.len();

    // start from bottom left
    let top_trees = forest.get(0).unwrap();
    let mut y_highest: Vec<u8> = top_trees.iter().map(|tree| tree.h).collect();

    total += top_trees.len() as u32;

    for y in 1..forest_len - 1 {
        let trees = forest.get(y).unwrap();
        let trees_above = forest.get(y - 1).unwrap();

        total += 1; // edge

        let mut x_highest: u8 = trees[0].h;

        for x in 1..trees.len() - 1 {
            let tree = trees.get(x).unwrap();
            let tree_above = trees_above.get(x).unwrap();
            let tree_to_left = trees.get(x - 1).unwrap();

            if tree_above.h > y_highest[x] {
                y_highest[x] = tree_above.h;
            }

            if tree_to_left.h > x_highest {
                x_highest = tree_to_left.h;
            }

            if tree.h > y_highest[x] || tree.h > x_highest {
                visible_trees.insert((tree.y, tree.x));
                total += 1;
            }
        }
    }

    // start from bottom right
    let bottom_trees = forest.get(forest_len - 1).unwrap();
    let mut y_highest: Vec<u8> = bottom_trees.iter().map(|tree| tree.h).collect();

    total += top_trees.len() as u32;

    for y in (1..forest_len - 1).rev() {
        let trees = forest.get(y).unwrap();
        let trees_below = forest.get(y + 1).unwrap();

        total += 1; // edge

        let trees_len = trees.len();
        let mut x_highest: u8 = trees[trees_len - 1].h;

        for x in (1..trees_len - 1).rev() {
            let tree = trees.get(x).unwrap();
            let tree_below = trees_below.get(x).unwrap();
            let tree_to_right = trees.get(x + 1).unwrap();

            if tree_below.h > y_highest[x] {
                y_highest[x] = tree_below.h;
            }

            if tree_to_right.h > x_highest {
                x_highest = tree_to_right.h;
            }

            if (tree.h > y_highest[x] || tree.h > x_highest) && visible_trees.insert((tree.y, tree.x))
            {
                total += 1;
            }
        }
    }

    println!("{total}");
}
