use std::{collections::HashSet, fs};

fn main() {
    let mut total = 0;

    let input = fs::read_to_string("i.in").unwrap();

    for line in input.lines() {
        let mut seen: HashSet<u32> = HashSet::new();
        let mut chars = line.chars();
        let half_breakboint = line.chars().count() / 2;

        for _ in 0..half_breakboint {
            let n = map_to_number(chars.next().unwrap());
            seen.insert(n);
        }

        loop {
            let n = map_to_number(chars.next().unwrap());
            if seen.contains(&n) {
                total += n;
                break;
            }
        }
    }
    println!("{total}");
}

fn map_to_number(c: char) -> u32 {
    let n = c as u32;
    if n <= 90 {
        // capital letter
        n - 65 + 27
    } else {
        // small letter
        n - 97 + 1
    }
}
