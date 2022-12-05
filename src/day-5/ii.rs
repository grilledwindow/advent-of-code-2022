use std::fs;

fn main() {
    let input = fs::read_to_string("i.in").unwrap();
    let mut crates = get_crates();

    for cc in &crates {
        let c = cc.last().unwrap();
        print!("{c}");
    }
    println!();
    for line in input.lines() {
        let split = line.split(" ");
        let split = split.collect::<Vec<&str>>();
        let qty = split.get(1).unwrap().parse::<usize>().unwrap();
        let from = split.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = split.get(5).unwrap().parse::<usize>().unwrap() - 1;
        move_crate(&mut crates, qty, from, to);
    }
    for cc in &crates {
        let c = cc.last().unwrap();
        print!("{c}");
    }
    println!();
}

fn move_crate(crates: &mut Vec<Vec<char>>, qty: usize, from: usize, to: usize) {
    let src = crates.get_mut(from).unwrap();
    let src_len = src.len();
    let moving: Vec<_> = src.split_off(src_len - qty);
    crates.get_mut(to).unwrap().extend(moving);
}

fn get_crates() -> Vec<Vec<char>> {
    vec![
        vec!['Q', 'F', 'M', 'R', 'L', 'W', 'C', 'V'],
        vec!['D', 'Q', 'L'],
        vec!['P', 'S', 'R', 'G', 'W', 'C', 'N', 'B'],
        vec!['L', 'C', 'D', 'H', 'B', 'Q', 'G'],
        vec!['V', 'G', 'L', 'F', 'Z', 'S'],
        vec!['D', 'G', 'N', 'P'],
        vec!['D', 'Z', 'P', 'V', 'F', 'C', 'W'],
        vec!['C', 'P', 'D', 'M', 'S'],
        vec!['Z', 'N', 'W', 'T', 'V', 'M', 'P', 'C'],
    ]
    
}