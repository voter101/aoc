use std::fs;
use std::vec;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");

    let mut count = 0;
    let mut results: Vec<i32> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            results.push(count);
            count = 0;
        } else { 
            count += line.parse::<i32>().unwrap();
        }
    }

    results.sort();
    results.reverse();

    let result_1: i32 = results[0];
    let result_2: i32 = results.iter().take(3).sum();

    println!("{}", result_1);
    println!("{}", result_2);
}
