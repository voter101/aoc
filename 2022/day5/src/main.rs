use std::fs;
use text_io::scan;

struct Instruction {
    count: usize,
    from: usize,
    to: usize
}

fn parse_input(input: &String) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut parsing_instructions = false;
    let mut stacks_lines: Vec<&str> = vec![];
    let mut instructions_lines: Vec<&str> = vec![];

    for line in input.lines() {
        if !parsing_instructions {
            if line == "" {
                parsing_instructions = true;
                stacks_lines.pop();
            } else {
                stacks_lines.push(line);
            }
        } else {
            instructions_lines.push(line);
        }
    }

    (create_initial_stacks(stacks_lines), create_list_of_instructions(instructions_lines))
}

fn create_initial_stacks(stack_lines: Vec<&str>) -> Vec<Vec<char>> {
    let stacks_count = (stack_lines[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![' '; 0];stacks_count];
    stack_lines.iter().enumerate().for_each(|(_,line)| {
        line.chars().skip(1).step_by(4).enumerate().for_each(|(stack_i, x)| {
            if x != ' ' {
                stacks[stack_i].insert(0,x);
            }
        });
    });
    stacks
}

fn create_list_of_instructions(orders_lines: Vec<&str>) -> Vec<Instruction> {
    orders_lines.iter().map(|line| {
        let mut instruction = Instruction {
            count: 0,
            from: 0,
            to: 0
        };
        scan!(line.bytes() => "move {} from {} to {}", instruction.count, instruction.from, instruction.to);
        instruction
    }).collect()
}

fn process_instructions_1(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    instructions.iter().for_each(|instruction| {
        for _ in 0..instruction.count {
            let popped = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(popped);
        }
    });
    String::from_iter(stacks.iter().map(|s| s.last().unwrap()))
}

fn process_instructions_2(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    instructions.iter().for_each(|instruction| {
        let new_len = stacks[instruction.from - 1].len() - instruction.count;
        let mut removed_elements: Vec<char> = stacks[instruction.from - 1].drain(new_len..).collect();
        stacks[instruction.to - 1].append(&mut removed_elements);
    });
    String::from_iter(stacks.iter().map(|s| s.last().unwrap()))
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let (stacks, instructions) = parse_input(&input);

    let result_1 = process_instructions_1(&mut stacks.clone(), &instructions);
    let result_2 = process_instructions_2(&mut stacks.clone(), &instructions);

    println!("{}", result_1);
    println!("{}", result_2);
}
