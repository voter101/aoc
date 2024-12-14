use std::fs;
use text_io::scan;

#[derive(Debug)]
struct Machine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize_location: (usize, usize),
}

fn parse_input(input: &String, artificial_prize_modifier: usize) -> Vec<Machine> {
    let mut res = vec![];

    let mut lines = input.lines().filter(|line| !line.is_empty()).peekable();

    while lines.peek().is_some() {
        let machine = lines.by_ref().take(3).collect::<Vec<_>>();
        let mut button_a: (usize, usize) = (0, 0);
        let mut button_b: (usize, usize) = (0, 0);
        let mut prize: (usize, usize) = (0, 0);
        scan!(machine[0].bytes() => "Button A: X+{}, Y+{}", button_a.0, button_a.1);
        scan!(machine[1].bytes() => "Button B: X+{}, Y+{}", button_b.0, button_b.1);
        scan!(machine[2].bytes() => "Prize: X={}, Y={}", prize.0, prize.1);

        prize.0 += artificial_prize_modifier;
        prize.1 += artificial_prize_modifier;

        res.push(Machine {
            button_a,
            button_b,
            prize_location: prize,
        });
    }

    res
}

fn prize_cost(machine: &Machine) -> Option<usize> {
    match possible_solution(&machine) {
        Some((x, y)) => Some(x * 3 + y),
        None => None,
    }
}

fn possible_solution(machine: &Machine) -> Option<(usize, usize)> {
    // Using Cramer's rule - thanks ChatGPT
    //
    // Expressing B = (top / bottom). Bottom cannot be 0 and the result must be a natural number.
    let top_b = machine.prize_location.0 as isize * machine.button_a.1 as isize
        - machine.prize_location.1 as isize * machine.button_a.0 as isize;
    let bottom_b = machine.button_b.0 as isize * machine.button_a.1 as isize
        - machine.button_b.1 as isize * machine.button_a.0 as isize;

    if bottom_b != 0 && top_b % bottom_b == 0 {
        let b = top_b / bottom_b;

        let top_a = machine.prize_location.0 as isize - b * machine.button_b.0 as isize;
        let bottom_a = machine.button_a.0 as isize;

        if bottom_a != 0 && top_a % bottom_a == 0 {
            let a = top_a / bottom_a;
            return Some((a as usize, b as usize));
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let machines = parse_input(&input, 0);
    let machines_inflated = parse_input(&input, 10000000000000);

    let result_1 = machines
        .iter()
        .map(|machine| prize_cost(machine).unwrap_or_default())
        .sum::<usize>();
    println!("{}", result_1);

    let result_2 = machines_inflated
        .iter()
        .map(|machine| prize_cost(machine).unwrap_or_default())
        .sum::<usize>();
    println!("{}", result_2);
}
