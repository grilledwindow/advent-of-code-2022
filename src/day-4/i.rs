use std::fs;

fn main() {
    let mut total = 0;

    let input = fs::read_to_string("i.in").unwrap();

    for line in input.lines() {
        let pair_strs: Vec<&str> = line.split(",").collect();

        let first_split: Vec<i32> = pair_strs
            .get(0)
            .unwrap()
            .split("-")
            .map(|c| c.parse::<i32>().unwrap())
            .collect();
        let first_start = first_split.get(0).unwrap();
        let first_end = first_split.get(1).unwrap();

        let second_split: Vec<i32> = pair_strs
            .get(1)
            .unwrap()
            .split("-")
            .map(|c| c.parse::<i32>().unwrap())
            .collect();
        let second_start = second_split.get(0).unwrap();
        let second_end = second_split.get(1).unwrap();

        if (first_start >= second_start && first_end <= second_end)
            || (second_start >= first_start && second_end <= first_end)
        {
            total += 1;
        }
    }
    println!("{total}");
}
