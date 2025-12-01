use std::fs;

fn parse_input() -> Vec<i16> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let val = l[1..].parse::<i16>().unwrap();
            if l.starts_with("L") {
                return -val;
            }
            val
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = parse_input();
    let mut current_pos: i16 = 50;
    let mut finishes_on_zero = 0;
    let mut crossing_zero = 0;

    for instruction in input {
        let init = current_pos;

        crossing_zero += instruction.abs() / 100;
        current_pos = (current_pos + instruction).rem_euclid(100);

        if init != 0 && current_pos != 0 {
            if (instruction < 0 && current_pos > init) || (instruction > 0 && current_pos < init) {
                crossing_zero += 1;
            }
        }

        if current_pos == 0 {
            finishes_on_zero += 1;
        }
    }

    println!("{}", finishes_on_zero);
    println!("{}", crossing_zero + finishes_on_zero);
}
