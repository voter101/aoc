use std::fs;

fn parse_line_multiple_numbers(line: String) -> Vec<u64> {
    line.split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn parse_line_single_number(line: String) -> u64 {
    line.split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

// Shameful brute force
fn number_of_ways_to_beat_record(time: u64, record: u64) -> u64 {
    (1..time)
        .into_iter()
        .filter(|t| (t * (time - t)) > record)
        .count() as u64
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("File not loaded");
    let lines = input
        .clone()
        .lines()
        .map(|x| parse_line_multiple_numbers(x.to_string()))
        .collect::<Vec<Vec<u64>>>();

    let times = &lines[0];
    let records = &lines[1];

    let lines_single_number = input
        .lines()
        .map(|x| parse_line_single_number(x.to_string()))
        .collect::<Vec<u64>>();

    let record_time = lines_single_number[0];
    let record_distance = lines_single_number[1];

    let ways_to_win_multiple = times
        .iter()
        .zip(records.iter())
        .map(|(t, r)| number_of_ways_to_beat_record(*t, *r))
        .product::<u64>();

    let ways_to_win_record = number_of_ways_to_beat_record(record_time, record_distance);

    println!("{}", ways_to_win_multiple);
    println!("{}", ways_to_win_record);
}
