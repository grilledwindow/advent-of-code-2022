use std::{collections::HashSet, fs};

fn main() {
    const STEP: usize = 3;

    let input = fs::read_to_string("i.in").unwrap();
    let len = input.lines().count();
    let mut lines = input.lines();

    let total = (0..len).step_by(STEP).fold(0, |acc, _| {
        let mut priorities_collection: Vec<HashSet<u32>> = (0..STEP)
            .map(|_| {
                lines
                    .next()
                    .unwrap()
                    .chars()
                    .into_iter()
                    .map(|c| map_to_number(c))
                    .collect::<HashSet<u32>>()
            })
            .collect();

        let mut intersections = priorities_collection.get(0).unwrap().to_owned();
        priorities_collection
            .iter_mut()
            .skip(1)
            .for_each(|priorities| {
                intersections.retain(|n| priorities.contains(n));
            });
        acc + intersections.into_iter().next().unwrap()
    });
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
