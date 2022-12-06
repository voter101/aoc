use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let result_1 = input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .take_while(|window| {
            let set: HashSet<&char> = HashSet::from_iter(window.iter());
            set.len() != 4
        })
        .count() + 4;

    let result_2 = input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .take_while(|window| {
            let set: HashSet<&char> = HashSet::from_iter(window.iter());
            set.len() != 14
        })
        .count() + 14;

    println!("{}", result_1);
    println!("{}", result_2);
}
