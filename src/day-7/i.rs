use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

mod util;
use util::*;

mod dir;
use dir::{Cwd, Dir};

fn main() {
    let root = Rc::new(RefCell::new(Dir::new(None, 0, HashMap::new())));
    let mut current_dir = root.clone();

    let mut total = 0;

    let _input = fs::read_to_string("i.in").unwrap();

    for line in _input.lines() {
        let split = line.split(" ");
        let split = split.collect::<Vec<&str>>();

        let mut i = 0;
        let first = *split.get(0).unwrap();
        match first {
            // command
            "$" => {
                let command = *split.get(1).unwrap();
                i += 1;
                if i == 2 {
                    break;
                }
                match command {
                    "ls" => {
                        // we can ignore
                    }
                    "cd" => {
                        let cd_target = *split.get(2).unwrap();
                        match cd_target {
                            ".." => {
                                let parent = current_dir.borrow().root.clone().unwrap();
                                current_dir = parent;
                            }
                            _ => {
                                let child = current_dir.borrow_mut().dirs.entry(cd_target).or_default().clone();
                                current_dir = child;
                            }
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            // directory
            "dir" => {
                let dir_name = *split.get(1).unwrap();
                current_dir
                    .borrow_mut()
                    .add_dir(dir_name, Dir::new(None, 0, HashMap::new()));
            }
            // file
            _ => {
                let size = first.parse::<u32>().unwrap();
                let file_name = *split.get(1).unwrap();
                current_dir.borrow_mut().add_dir(file_name, Dir::new(Some(current_dir.clone()), size, HashMap::new()));
            }
        }
    }
    println!("{total}");
    println!("{current_dir:#?}");
}

// // fn update_dirs(root: &mut Dir, )
// use std::ref;

// use std::cell::RefCell;

// #[derive(Debug)]
// struct N {
//     pub root: Option<Box<RefCell<N>>>,
//     pub size: u32,
//     pub n: Option<Vec<N<>>>,
// }

// impl N {
//     fn update_n(&mut self, n: Option<Vec<N>>) {
//         self.n = n;
//     }
// }

// fn main() {
//     let mut n = N { root: None, size: 1, n: None };
//     n.update_n(None);
//     n.update_n(Some(vec![N { root: Some(RefCell::, size: 3, n: None }]));
//     dbg!(n);
// }
