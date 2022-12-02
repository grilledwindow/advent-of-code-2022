use std::fs;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn main() {
    let mut total = 0;

    let input = fs::read_to_string("i.in").unwrap();

    for line in input.lines() {
        let split = line.split(" ");
        let split = split.collect::<Vec<&str>>();

        let opponent = map_to_shape(split.get(0).unwrap());
        let outcome = split.get(1).unwrap();

        total += get_round_total(opponent, outcome);
    }
    println!("{total}");
}

fn get_round_total(opponent: i32, outcome: &str) -> i32 {
    match outcome {
        "Y" => opponent + 3,
        "X" => get_one_before(opponent) + 0,
        "Z" => get_one_after(opponent) + 6,
        _ => unimplemented!()
    }
}

fn get_one_before(x: i32) -> i32 {
    match x {
        ROCK => SCISSORS,
        _ => x - 1
    }
}

fn get_one_after(x: i32) -> i32 {
    match x {
        SCISSORS => ROCK,
        _ => x + 1
    }
}

fn map_to_shape(c: &str) -> i32 {
    match c {
        "A" => ROCK,
        "B" => PAPER,
        "C" => SCISSORS, 
        _ => unimplemented!(),
    }
}
