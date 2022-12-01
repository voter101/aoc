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

    let result: i32 = results.iter().take(3).sum();

    println!("{}", result);
}
