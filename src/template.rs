use std::fs;

mod util;
use util::*;

fn main() {
    let mut total = 0;

    let input = fs::read_to_string("i.in").unwrap();

    for line in input.lines() {
        let split = line.split(" ");
        let split = split.collect::<Vec<&str>>();
    }
    println!("{total}");
}
