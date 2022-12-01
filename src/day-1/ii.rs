use std::fs;

fn main() {
    let mut current_total_calories = 0;
    let mut highest_calories = [0, 0, 0];

    let input = fs::read_to_string("i.in").unwrap();

    for line in input.lines() {
        if let Ok(current_calories) = line.parse::<i32>() {
            current_total_calories += current_calories;
        } else {
            for i in 0..highest_calories.len() {
                if current_total_calories > highest_calories[i] {
                    shift_right_by_one(&mut highest_calories, i);
                    highest_calories[i] = current_total_calories;
                    break;
                }
            }
            current_total_calories = 0;
        }
    }

    let mut top_three_elves_total_calories = 0;
    for calories in highest_calories {
        top_three_elves_total_calories += calories;
    }

    println!("{top_three_elves_total_calories}");
}

fn shift_right_by_one(array: &mut [i32], index: usize) {
    for i in index..array.len() - 1 {
        array[i + 1] = array[i];
    }
}