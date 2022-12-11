use std::fs;

#[derive(Clone, Debug)]
enum Instruction {
    Noop, 
    AddStart(i32),
    AddEnd(i32)
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");

    let mut register_value: i32 = 1;
    let mut val_on_cycle = vec![1; 240];
    let mut display = vec![vec!['.'; 40]; 6];

    let mut instructions: Vec<Instruction> = input.lines().map(|line| {
        if line == "noop" {
            Instruction::Noop
        } else {
            let to_add: i32 = line[5..].parse().unwrap();
            Instruction::AddStart(to_add)
        }
    }).collect();

    let mut current_instruction: Instruction = Instruction::Noop;

    for i in 0..240 {

        match current_instruction {
            Instruction::Noop => {
                current_instruction = instructions[0].clone();
                instructions.remove(0);
            },
            Instruction::AddStart(val) => {
                current_instruction = Instruction::AddEnd(val);
            },
            Instruction::AddEnd(val) => {
                register_value += val; 
                current_instruction = instructions[0].clone();
                instructions.remove(0);
            }
        };
        
        val_on_cycle[i] = register_value; 

        let row = i / 40;
        let pixel_position = i % 40;

        if ((register_value - 1)..=(register_value + 1)).contains(&(pixel_position as i32)) {
        display[row][pixel_position] = '#';
        }
    }

    let mut result_1 = 0;
    for cycle in [20, 60, 100, 140, 180, 220] {
        let val = val_on_cycle[cycle - 1];
        result_1 += val * (cycle as i32);
    }

    println!("{}", result_1);

    for row in display {
        println!("{}", row.iter().collect::<String>());
    }
}
