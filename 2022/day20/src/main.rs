use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let numbers = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let numbers_len = numbers.len();

    let mut numbers_moving = numbers.clone();

    for curr in numbers {
        if curr != 0 {
            let index = numbers_moving.iter().position(|&e| e == curr).unwrap() as i32;
            let potential_index = index + curr;
            numbers_moving.remove(index as usize);
            if potential_index == 0 {
                numbers_moving.push(curr);
            } else {
                let new_index_ = potential_index.rem_euclid((numbers_len - 1) as i32);
                let new_index: usize = new_index_ as usize;

                numbers_moving.insert(new_index, curr);
            }
        }
    }

    let mut numbers_cycle = numbers_moving.iter().cycle();
    loop {
        if numbers_cycle.next().unwrap().clone() == 0 {
            break;
        }
    }
    let result: i32 = [
        numbers_cycle.nth(999).unwrap().clone(),
        numbers_cycle.nth(1000).unwrap().clone(),
        numbers_cycle.nth(1000).unwrap().clone(),
    ]
    .iter()
    .sum();
    println!("{}", result);
}
