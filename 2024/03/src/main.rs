use regex::Regex;
use std::fs;

#[derive(Debug)]
enum Operation {
    Mul((usize, usize)),
    Do,
    DoNot,
}

fn parse_correct_operations(input: &String) -> Vec<Operation> {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(do\(\))|(don\'t\(\))").unwrap();

    re.captures_iter(input.as_str())
        .map(|captures| match captures.get(0).unwrap().as_str() {
            "do()" => Operation::Do,
            "don't()" => Operation::DoNot,
            _ => Operation::Mul((
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            )),
        })
        .collect::<Vec<Operation>>()
}

fn calculate(operations: &Vec<Operation>, only_mut: bool) -> usize {
    operations
        .iter()
        .fold((0, true), |(acc, enabled), e| match e {
            Operation::Do => (acc, true),
            Operation::DoNot => (acc, false),
            Operation::Mul((a, b)) => {
                let to_add = if enabled || only_mut { a * b } else { 0 };
                (acc + to_add, enabled)
            }
        })
        .0
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let operations = parse_correct_operations(&input);

    println!("{}", calculate(&operations, true));
    println!("{}", calculate(&operations, false));
}
