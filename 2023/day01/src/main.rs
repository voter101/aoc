use std::collections::HashMap;
use std::fs;

fn extract_digits(input: &String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| {
            let mut first_digit: Option<char> = None;
            let mut last_digit: Option<char> = None;

            for c in line.chars() {
                if c.is_numeric() {
                    if first_digit.is_none() {
                        first_digit = Some(c);
                    }
                    last_digit = Some(c);
                }
            }
            vec![
                first_digit.unwrap_or_else(|| '0'),
                last_digit.unwrap_or_else(|| '0'),
            ]
        })
        .collect()
}

fn substring(input: &str, start: usize, end: usize) -> &str {
    input.get(start..end).unwrap()
}

fn replace_words_with_numbers(input: &String) -> String {
    let numbers: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    input
        .lines()
        .map(|line| {
            let mut digits = vec![];
            for i in 0..line.len() {
                let c = line.chars().nth(i).unwrap();
                if c.is_numeric() {
                    digits.push(c);
                } else {
                    for j in (i + 2)..=(i + 5) {
                        if j <= line.len() {
                            match numbers.get(substring(line, i, j)) {
                                Some(digit) => digits.push(digit.clone()),
                                _ => {}
                            }
                        }
                    }
                }
            }
            digits.push('\n');
            String::from_iter(digits.iter())
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let digits: Vec<Vec<char>> = extract_digits(&input);
    let numbers: Vec<u32> = digits
        .iter()
        .map(|d| String::from_iter(d.iter()).parse::<u32>().unwrap())
        .collect();
    let sum: u32 = numbers.iter().sum();

    let digits_with_words: Vec<Vec<char>> = extract_digits(&replace_words_with_numbers(&input));
    let numbers_with_words: Vec<u32> = digits_with_words
        .iter()
        .map(|d| String::from_iter(d.iter()).parse::<u32>().unwrap())
        .collect();
    let sum_with_words: u32 = numbers_with_words.iter().sum();

    println!("{:?}", sum);
    println!("{:?}", sum_with_words);
}
