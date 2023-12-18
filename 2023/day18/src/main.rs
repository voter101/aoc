use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Order {
    length: i64,
    direction: Direction,
}

type Coords = (i64, i64);

fn parse_input(input: &str) -> Vec<Order> {
    input
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();

            let direction = match split[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            let steps = split[1].parse::<i64>().unwrap();

            Order {
                direction,
                length: steps,
            }
        })
        .collect()
}

fn parse_input_correct(input: &String) -> Vec<Order> {
    input
        .lines()
        .map(|line| {
            let input = line.split_whitespace().last().unwrap();
            let digits = input.chars().skip(2).take(5).collect::<String>();
            let direction = match input
                .chars()
                .rev()
                .skip(1)
                .take(1)
                .collect::<String>()
                .as_str()
            {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => unreachable!(),
            };

            Order {
                length: i64::from_str_radix(digits.as_str(), 16).unwrap(),
                direction,
            }
        })
        .collect()
}

// Shoelace formula
fn calculate_area(orders: &Vec<Order>) -> i64 {
    let mut area: i64 = 0;
    let mut border: i64 = 0;
    let mut current_coords: Coords = (0, 0);
    let mut next_coords: Coords = (0, 0);

    for order in orders {
        border += order.length;
        match order.direction {
            Direction::Up => next_coords.1 = next_coords.1 - order.length,
            Direction::Down => next_coords.1 = next_coords.1 + order.length,
            Direction::Left => next_coords.0 = next_coords.0 - order.length,
            Direction::Right => next_coords.0 = next_coords.0 + order.length,
        }

        area = area + (current_coords.0 - next_coords.0) * (current_coords.1 + next_coords.1);
        current_coords = next_coords.clone();
    }

    i64::abs((area + border) / 2 + 1)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input");
    let orders_initial = parse_input(&input);
    let orders_correct = parse_input_correct(&input);

    let area_initial = calculate_area(&orders_initial);
    println!("{}", area_initial);

    let area_correct = calculate_area(&orders_correct);
    println!("{}", area_correct);
}
