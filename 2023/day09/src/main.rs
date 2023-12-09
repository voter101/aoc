use std::fs;

fn next_value(sensor_values: &Vec<i32>) -> i32 {
    let mut collected = vec![];

    if sensor_values.len() == 1 {
        return sensor_values.first().unwrap().clone();
    }

    for i in 1..sensor_values.len() {
        collected.push(sensor_values[i] - sensor_values[i - 1]);
    }

    sensor_values.last().unwrap() + next_value(&collected)
}

fn prev_value(sensor_values: &Vec<i32>) -> i32 {
    let mut collected = vec![];

    if sensor_values.len() == 1 {
        return sensor_values.first().unwrap().clone();
    }

    for i in 1..sensor_values.len() {
        collected.push(sensor_values[i] - sensor_values[i - 1]);
    }

    sensor_values.first().unwrap() - prev_value(&collected)
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let sensor_values = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let values_sum = sensor_values.iter().map(|x| next_value(x)).sum::<i32>();
    let begining_sum = sensor_values.iter().map(|x| prev_value(x)).sum::<i32>();

    println!("{}", values_sum);
    println!("{}", begining_sum);
}
