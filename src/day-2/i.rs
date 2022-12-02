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

        let opponent = map_to_number(split.get(0).unwrap());
        let me = map_to_number(split.get(1).unwrap());

        let outcome = get_outcome(opponent, me);
        let current_total = me + outcome;

        total += current_total;
    }
    println!("{total}");
}

fn get_outcome(opponent: i32, me: i32) -> i32 {
    if opponent == me {
        3
    } else if opponent == ROCK {
        if me == PAPER {
            6
        } else {
            0
        }
    } else if opponent == PAPER {
        if me == SCISSORS {
            6
        } else {
            0
        }
    } else {
        if me == ROCK {
            6
        } else {
            0
        }
    }
}

fn map_to_number(c: &str) -> i32 {
    match c {
        "A" | "X" => 1, // rock
        "B" | "Y" => 2, // paper
        "C" | "Z" => 3, // scissors
        _ => unimplemented!(),
    }
}
