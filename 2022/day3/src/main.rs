use std::collections::HashSet;
use std::fs;

fn ascii_u32_to_priority(value: u32) -> u32 {
    // a-z
    if value >= 97 {
        value - 96
    // A-Z
    } else {
        value - 64 + 26
    }
}

fn part_1(input: &String) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let first_half: HashSet<char> = line.chars().take(line.len() / 2).collect();
        let second_half: HashSet<char> = line.chars().skip(line.len() / 2).collect();
        let value = *first_half.intersection(&second_half).collect::<Vec<&char>>()[0] as u32;
        result += ascii_u32_to_priority(value);
    }
    result
}

fn part_2(input: &String) -> u32 {
    let mut result = 0;

    for lines in input.lines().collect::<Vec<_>>().chunks(3) {
        let value = lines
            .iter()
            .map(|l| l.chars().collect::<HashSet<char>>())
            .reduce(|acc, item| acc.intersection(&item).cloned().collect())
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>()[0] as u32;
    
        result += ascii_u32_to_priority(value);
    }
    result
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}
