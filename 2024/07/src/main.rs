use std::fs;

fn parse_input(input: String) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (result, rest) = line.split_once(": ").unwrap();

            let elems: Vec<usize> = rest
                .split(" ")
                .map(|e| e.parse::<usize>().unwrap())
                .collect();

            (result.parse::<usize>().unwrap(), elems)
        })
        .collect::<Vec<_>>()
}

fn possible_permutations(
    (target, operations): &(usize, Vec<usize>),
    use_extra_operator: bool,
) -> usize {
    if operations[0] > *target {
        return 0;
    }

    if operations.len() == 1 {
        if operations[0] == *target {
            return 1;
        } else {
            return 0;
        }
    }

    let tail = &operations[2..].to_vec();

    let regular_operations = possible_permutations(
        &(
            *target,
            [vec![operations[0] + operations[1]], tail.clone()].concat(),
        ),
        use_extra_operator,
    ) + possible_permutations(
        &(
            *target,
            [vec![operations[0] * operations[1]], tail.clone()].concat(),
        ),
        use_extra_operator,
    );

    let extra_operation = if use_extra_operator {
        let a = operations[0].to_string();
        let b = operations[1].to_string();
        possible_permutations(
            &(
                *target,
                [
                    vec![format!("{a}{b}").parse::<usize>().unwrap()],
                    tail.clone(),
                ]
                .concat(),
            ),
            use_extra_operator,
        )
    } else {
        0
    };

    regular_operations + extra_operation
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let operations = parse_input(input);
    let result_1 = operations
        .iter()
        .filter(|o| possible_permutations(o, false) > 0)
        .map(|o| o.0)
        .sum::<usize>();

    println!("{}", result_1);

    let result_1 = operations
        .iter()
        .filter(|o| possible_permutations(o, true) > 0)
        .map(|o| o.0)
        .sum::<usize>();

    println!("{}", result_1);
}
