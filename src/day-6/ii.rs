use std::fs;

mod util;
use util::*;

fn main() {
    let mut total = 14;

    let mut marker = fs::read_to_string("i.in").unwrap().chars().collect::<Vec<char>>();
    let rest: Vec::<char> = marker.split_off(14);

    for c in rest {
        total += 1;
        shift_left(&mut marker, c);
        if is_unique(&mut marker) {
            break
        }
    }
    println!("{total}");
}
