use std::fs;

fn decode(numbers_input: &Vec<(usize, i64)>, rounds: usize, key: usize) -> i64 {
    let mut numbers: Vec<(usize, i64)> = numbers_input
        .clone()
        .iter_mut()
        .map(|(index, val)| (index.clone(), val.clone() * key as i64))
        .collect();
    let numbers_len = numbers.len();

    for _ in 0..rounds {
        for index in 0..numbers_len {
            let current_index = numbers.iter().position(|&e| e.0 == index).unwrap();
            let potential_index = current_index as i64 + numbers[current_index].1;
            let new_index = potential_index.rem_euclid(numbers_len as i64 - 1);
            let val = numbers.remove(current_index as usize);
            numbers.insert(new_index as usize, val);
        }
    }

    let zero = numbers.iter().position(|&e| e.1 == 0).unwrap();
    return [1000, 2000, 3000]
        .iter()
        .map(|shift| numbers[(zero + shift) % numbers_len].1)
        .sum::<i64>();
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let numbers = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();

    println!("{}", decode(&numbers, 1, 1));
    println!("{}", decode(&numbers, 10, 811589153));
}
