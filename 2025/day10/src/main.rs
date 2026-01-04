use std::fs;

type DestinationStates = Vec<bool>;
type Button = Vec<usize>;
type Buttons = Vec<Button>;
type Joltages = Vec<usize>;

fn parse_input() -> Vec<(DestinationStates, Buttons, Joltages)> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let segments = line.split_whitespace().collect::<Vec<_>>();
            let raw_destination_states = segments.first().unwrap();
            let destination_states = raw_destination_states
                .trim_matches(['[', ']'])
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => false,
                })
                .collect::<Vec<bool>>();

            let buttons = segments[1..segments.len() - 1]
                .iter()
                .map(|segment| {
                    segment
                        .trim_matches(['(', ')'])
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            let raw_joltages = segments.last().unwrap();
            let joltages = raw_joltages
                .trim_matches(['{', '}'])
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            (destination_states, buttons, joltages)
        })
        .collect()
}

fn fewest_presses_to_destination(machine: &(DestinationStates, Buttons, Joltages)) -> usize {
    let mut state = vec![false; machine.0.len()];
    let mut best = usize::MAX;
    fewest_presses_aux(&machine.1, &machine.0, &mut state, 0, 0, &mut best);
    best
}

fn fewest_presses_aux(
    buttons: &[Button],
    dest: &[bool],
    state: &mut [bool],
    index: usize,
    presses: usize,
    best: &mut usize,
) {
    if presses >= *best {
        return;
    }

    if index == buttons.len() {
        if state == dest {
            *best = presses;
        }
        return;
    }

    fewest_presses_aux(buttons, dest, state, index + 1, presses, best);

    for &idx in &buttons[index] {
        state[idx] = !state[idx];
    }
    fewest_presses_aux(buttons, dest, state, index + 1, presses + 1, best);
    for &idx in &buttons[index] {
        state[idx] = !state[idx];
    }
}

fn z3_cli_input(machines: &[(DestinationStates, Buttons, Joltages)]) -> String {
    let mut out = String::new();
    out.push_str("(set-logic QF_LIA)\n");
    out.push_str("(set-option :produce-models true)\n");

    for (machine_idx, (dest, buttons, joltages)) in machines.iter().enumerate() {
        out.push_str(&format!("; machine {machine_idx}\n"));
        let button_vars = (0..buttons.len())
            .map(|i| format!("m{machine_idx}_b{i}"))
            .collect::<Vec<_>>();

        for var in &button_vars {
            out.push_str(&format!("(declare-const {var} Int)\n"));
        }
        for var in &button_vars {
            out.push_str(&format!("(assert (>= {var} 0))\n"));
        }

        for i in 0..dest.len() {
            let init_var = format!("m{machine_idx}_init{i}");
            out.push_str(&format!("(declare-const {init_var} Int)\n"));
            out.push_str(&format!("(assert (>= {init_var} 0))\n"));
            out.push_str(&format!("(assert (<= {init_var} 1))\n"));
            let sum = sum_for_index(buttons, &button_vars, i);
            let target = if dest[i] { 1 } else { 0 };
            out.push_str(&format!(
                "(assert (= (mod (+ {init_var} {sum}) 2) {target}))\n"
            ));
        }

        for i in 0..joltages.len() {
            let sum = sum_for_index(buttons, &button_vars, i);
            out.push_str(&format!("(assert (= {sum} {}))\n", joltages[i]));
        }
    }

    let total_sum_parts = machines
        .iter()
        .enumerate()
        .map(|(machine_idx, machine)| {
            let button_vars = (0..machine.1.len())
                .map(|i| format!("m{machine_idx}_b{i}"))
                .collect::<Vec<_>>();
            sum_expr(&button_vars)
        })
        .collect::<Vec<_>>();
    let total_sum_expr = sum_expr(&total_sum_parts);
    out.push_str(&format!("(minimize {total_sum_expr})\n"));
    out.push_str("(check-sat)\n");
    out.push_str(&format!("(eval {total_sum_expr})\n"));
    for (machine_idx, machine) in machines.iter().enumerate() {
        let button_vars = (0..machine.1.len())
            .map(|i| format!("m{machine_idx}_b{i}"))
            .collect::<Vec<_>>();
        let clicks_sum = sum_expr(&button_vars);
        out.push_str(&format!("(eval {clicks_sum})\n"));
    }

    out
}

fn sum_expr(vars: &[String]) -> String {
    match vars.len() {
        0 => "0".to_string(),
        1 => vars[0].clone(),
        _ => format!("(+ {})", vars.join(" ")),
    }
}

fn sum_for_index(buttons: &[Button], button_vars: &[String], index: usize) -> String {
    let vars = buttons
        .iter()
        .enumerate()
        .filter_map(|(button_idx, button)| {
            if button.contains(&index) {
                Some(button_vars[button_idx].clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    sum_expr(&vars)
}

fn main() {
    let machines = parse_input();
    let part2 = std::env::args().any(|arg| arg == "--part2");

    if part2 {
        // Usage: cargo run -- --part2 > input.smt2 && /opt/homebrew/bin/z3 input.smt2 | sed -n '2p'
        print!("{}", z3_cli_input(&machines));
    } else {
        println!(
            "{}",
            machines
                .iter()
                .map(|machine| fewest_presses_to_destination(&machine))
                .sum::<usize>()
        );
    }
}
