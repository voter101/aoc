use std::collections::VecDeque;
use std::{fs, ops::BitXor};
use text_io::scan;

#[derive(Clone)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<usize>,
    current_position: usize,
    output: Vec<usize>,
}

impl Computer {
    fn new(register_a: usize, register_b: usize, register_c: usize, program: Vec<usize>) -> Self {
        Computer {
            register_a,
            register_b,
            register_c,
            program,
            current_position: 0,
            output: vec![],
        }
    }

    fn run(&mut self) {
        while self.current_position < self.program.len() {
            let instruction = self.program[self.current_position];
            let operand = self.program[self.current_position + 1];

            let combo_operand = match operand {
                0 | 1 | 2 | 3 => operand,
                4 => self.register_a,
                5 => self.register_b,
                6 => self.register_c,
                _ => unreachable!(),
            };

            let mut advance = true;

            match instruction {
                0 => self.register_a = self.register_a / (2 as usize).pow(combo_operand as u32),
                1 => self.register_b = self.register_b.bitxor(operand),
                2 => self.register_b = combo_operand.rem_euclid(8),
                3 => {
                    if self.register_a != 0 {
                        advance = false;
                        self.current_position = operand;
                    }
                }
                4 => self.register_b = self.register_b.bitxor(self.register_c),
                5 => self.output.push(combo_operand.rem_euclid(8)),
                6 => self.register_b = self.register_a / (2 as usize).pow(combo_operand as u32),
                7 => self.register_c = self.register_a / (2 as usize).pow(combo_operand as u32),
                _ => {}
            };

            if advance {
                self.current_position += 2;
            }
        }
    }

    fn output_result(&self) -> String {
        self.output
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn is_output_equal_program(&self) -> bool {
        self.output == self.program
    }
}

fn parse_input(input: String) -> Computer {
    let lines = input.lines().collect::<Vec<&str>>();
    let register_a;
    let register_b;
    let register_c;

    let raw_program: String;

    scan!(lines[0].bytes() => "Register A: {}", register_a);
    scan!(lines[1].bytes() => "Register B: {}", register_b);
    scan!(lines[2].bytes() => "Register C: {}", register_c);
    scan!(lines[4].bytes() => "Program: {}", raw_program);

    let program = raw_program
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    Computer::new(register_a, register_b, register_c, program)
}

fn find_register_with_program_equal_output(computer: &Computer) -> usize {
    let mut computer_base = computer.clone();

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((0, 0));

    while !queue.is_empty() {
        let (reg_a, digit) = queue.pop_front().unwrap();

        // Key trick - look for results going from right to left. The previous digits will be stable
        //
        // We are only interested in the next digit. Building on the previous A value. For each A value we have 8 values to check.
        let target = computer_base.program[computer_base.program.len() - digit - 1];

        for i in 0..=7 {
            let candidate = (reg_a << 3) | i;
            computer_base.register_a = candidate;
            computer_base.output = vec![];
            computer_base.current_position = 0;

            computer_base.run();

            if computer_base.output[0] == target {
                // There are many possibile choices. Let's pick the first one - I think it will be the lowest
                if digit == computer_base.program.len() - 1
                    && computer_base.is_output_equal_program()
                {
                    return candidate;
                } else {
                    queue.push_back((candidate, digit + 1));
                }
            }
        }
    }

    panic!("No solution found")
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut computer = parse_input(input);

    computer.run();

    println!("{}", computer.output_result());
    println!("{}", find_register_with_program_equal_output(&computer));
}
