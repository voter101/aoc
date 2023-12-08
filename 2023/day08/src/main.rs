use num::integer::lcm;
use std::collections::HashMap;
use std::fs;
use text_io::scan;

enum Instruction {
    Left,
    Right,
}

fn parse_input(input: String) -> (Vec<Instruction>, HashMap<String, (String, String)>) {
    let split = input.split_once("\n\n").unwrap();
    let instructions_raw = split.0;
    let rules_raw = split.1;

    let instructions: Vec<Instruction> = instructions_raw
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => {
                unreachable!()
            }
        })
        .collect();
    let mut rules: HashMap<String, (String, String)> = HashMap::new();

    rules_raw.lines().for_each(|line| {
        let source: String;
        let left: String;
        let right: String;
        scan!(line.bytes() => "{} = ({}, {})", source, left, right);
        rules.insert(source, (left, right));
    });

    (instructions, rules)
}

fn path_to_finish_length(
    instructions: &Vec<Instruction>,
    rules: &HashMap<String, (String, String)>,
) -> u32 {
    let mut result: u32 = 0;
    let mut current = "AAA".to_string();

    let mut instructions_cycle = instructions.iter().cycle();

    while current != "ZZZ" {
        match instructions_cycle.next().unwrap() {
            Instruction::Left => {
                let (left, _) = rules.get(&current).unwrap();
                current = left.to_string();
            }
            Instruction::Right => {
                let (_, right) = rules.get(&current).unwrap();
                current = right.to_string();
            }
        };
        result = result + 1;
    }

    result
}

fn path_to_simultenaous_length(
    instructions: &Vec<Instruction>,
    rules: &HashMap<String, (String, String)>,
) -> u64 {
    let mut paths: Vec<(&String, u64, bool)> = rules
        .iter()
        .map(|(k, _)| k)
        .filter(|k| k.ends_with("A"))
        .map(|k| (k, 0, false))
        .collect::<Vec<_>>();

    let mut instructions_cycle = instructions.iter().cycle();

    while !paths.iter().all(|p| p.2) {
        let instruction = instructions_cycle.next().unwrap();

        for (i, p) in paths.clone().iter().enumerate() {
            if p.2 {
                continue;
            }

            if p.0.ends_with("Z") {
                paths[i] = (p.0, p.1, true);
                continue;
            }

            let val = match instruction {
                Instruction::Left => {
                    let (left, _) = rules.get(p.0).unwrap();
                    left
                }
                Instruction::Right => {
                    let (_, right) = rules.get(p.0).unwrap();
                    right
                }
            };
            paths[i] = (val, p.1 + 1, false);
        }
    }

    paths.iter().map(|p| p.1).fold(1, |acc, x| lcm(acc, x))
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let (instructions, rules) = parse_input(input);

    println!("{:?}", path_to_finish_length(&instructions, &rules));
    println!("{:?}", path_to_simultenaous_length(&instructions, &rules));
}
