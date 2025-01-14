use std::collections::{HashMap, HashSet};
use std::fs;
use text_io::scan;

type Registers = HashMap<String, usize>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Operator {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Gate {
    left: String,
    right: String,
    output: String,
    operator: Operator,
}

impl Gate {
    fn try_resolving(&self, registers: &Registers) -> Option<usize> {
        let left = match registers.get(&self.left) {
            Some(value) => *value,
            None => return None,
        };

        let right = match registers.get(&self.right) {
            Some(value) => *value,
            None => return None,
        };

        let result = match self.operator {
            Operator::AND => left & right,
            Operator::OR => left | right,
            Operator::XOR => left ^ right,
        };

        Some(result)
    }
}

fn parse_input(input: String) -> (Registers, Vec<Gate>) {
    let (part_a, part_b) = input.split_once("\n\n").unwrap();

    let mut registers: Registers = HashMap::new();

    for line in part_a.lines() {
        let register_name: String;
        let register_value: usize;
        scan!(line.bytes() => "{}: {}", register_name, register_value);
        registers.insert(register_name, register_value);
    }

    let gates = part_b
        .lines()
        .map(|line| {
            let left: String;
            let right: String;
            let operator_raw: String;
            let output: String;

            scan!(line.bytes() => "{} {} {} -> {}", left, operator_raw, right, output);

            let operator: Operator = match operator_raw.as_str() {
                "AND" => Operator::AND,
                "OR" => Operator::OR,
                "XOR" => Operator::XOR,
                _ => unreachable!("Invalid operator"),
            };

            Gate {
                left,
                right,
                output,
                operator,
            }
        })
        .collect::<Vec<_>>();
    (registers, gates)
}

fn process_gates(init_registers: &Registers, init_gates: &Vec<Gate>) -> Option<String> {
    let mut registers = init_registers.clone();
    let mut gates = init_gates.clone();

    while !gates.is_empty() {
        let mut changed = false;
        for gate in gates.clone().iter() {
            if let Some(result) = gate.try_resolving(&registers) {
                changed = true;
                registers.insert(gate.output.clone(), result);
                gates.retain(|g| g.output != gate.output);
            }
        }

        if !changed {
            return None;
        }
    }

    Some(extract_register_bytes("z".to_string(), &registers))
}

fn extract_register_bytes(prefix: String, registers: &Registers) -> String {
    let mut res: String = String::from("");
    let mut i = 0;

    loop {
        let key = format!("{}{:0>2}", prefix, i);
        if let Some(value) = registers.get(&key) {
            res.push_str(&value.to_string());
        } else {
            break;
        }
        i += 1;
    }

    res
}

fn find_lowest_broken_bit(input_length: usize, start_n: usize, gates: &Vec<Gate>) -> Option<usize> {
    let mut empty_registers = HashMap::new();
    for i in 0..input_length {
        empty_registers.insert(format!("x{:0>2}", i), 0);
        empty_registers.insert(format!("y{:0>2}", i), 0);
    }

    for i in start_n..input_length {
        for x in 0..=1 {
            for y in 0..=1 {
                for carry in 0..=1 {
                    let mut registers: Registers = empty_registers.clone();
                    if x == 1 {
                        registers.insert(format!("x{:0>2}", i), 1);
                    }
                    if y == 1 {
                        registers.insert(format!("y{:0>2}", i), 1);
                    }

                    if carry == 1 && i > 0 {
                        registers.insert(format!("x{:0>2}", i - 1), 1);
                        registers.insert(format!("y{:0>2}", i - 1), 1);
                    }

                    let (x_b, y_b) = extract_register_inputs(input_length, &registers);
                    let gates_clone = gates.clone();

                    if let Some(num) = process_gates(&registers, &gates_clone) {
                        if val(num) != add(x_b, y_b) {
                            return Some(i);
                        }
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    Some(input_length + 1)
}

fn extract_register_inputs(input_length: usize, registers: &Registers) -> (String, String) {
    let mut x_b: String = String::from("");
    let mut y_b: String = String::from("");

    for i in 0..input_length {
        x_b.push_str(&registers.get(&format!("x{:0>2}", i)).unwrap().to_string());
        y_b.push_str(&registers.get(&format!("y{:0>2}", i)).unwrap().to_string());
    }

    (x_b, y_b)
}

fn add(a: String, b: String) -> usize {
    val(a) + val(b)
}

fn val(v: String) -> usize {
    usize::from_str_radix(&v.chars().rev().collect::<String>(), 2).unwrap()
}

fn swap_ops(gates: &mut Vec<Gate>, a: usize, b: usize) {
    let tmp = gates[a].output.to_string();
    gates[a].output = gates[b].output.to_string();
    gates[b].output = tmp;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let (registers, gates) = parse_input(input);
    let result_1 = process_gates(&registers, &gates).unwrap();

    println!(
        "{}",
        usize::from_str_radix(&result_1.chars().rev().collect::<String>(), 2).unwrap()
    );

    let mut lowest_broken_bit = find_lowest_broken_bit(result_1.len() - 1, 0, &gates).unwrap();
    let mut gates = gates.clone();
    let mut res: Vec<String> = Vec::new();

    let mut broken_pairs: HashSet<(usize, usize)> = HashSet::new();

    'outer: loop {
        for i in 0..gates.len() {
            for ii in (i + 1)..gates.len() {
                if broken_pairs.contains(&(i, ii)) {
                    continue;
                }
                swap_ops(&mut gates, i, ii);

                if let Some(broken_bits) =
                    find_lowest_broken_bit(result_1.len() - 1, lowest_broken_bit - 1, &gates)
                {
                    if broken_bits < lowest_broken_bit {
                        broken_pairs.insert((i, ii));
                    }

                    // Input specific
                    if broken_bits > lowest_broken_bit + 4 {
                        lowest_broken_bit = broken_bits;
                        res.push(gates[i].output.to_string());
                        res.push(gates[ii].output.to_string());
                        if broken_bits == result_1.len() {
                            break 'outer;
                        }
                        continue 'outer;
                    }
                }
                swap_ops(&mut gates, i, ii);
            }
        }
    }
    res.sort();
    println!("{:?}", res.join(","));
}
