use std::fs;

fn main() {
    let mut current_total_calories = 0;
    let mut highest_calories = 0;

    let input = fs::read_to_string("i.in").unwrap();

    for line in input.lines() {
        if let Ok(current_calories) = line.parse::<i32>() {
            current_total_calories += current_calories;
        } else {
            if current_total_calories > highest_calories {
                highest_calories = current_total_calories;
            }
            current_total_calories = 0;
        }
    }
    println!("{highest_calories}");
}
