use std::fs;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone, Copy, Debug)]
enum OperatorValue {
    Usize(usize),
    OwnSelf,
}

#[derive(Clone, Debug)]
struct Monke {
    items: Vec<usize>,
    inspect_operation: (Operator, OperatorValue),
    test_div: usize,
    test_addr: (usize, usize),
    inspect_count: usize,
}

fn monke_lines(input: String) -> Vec<Monke> {
    let mut monkes_input: Vec<Vec<String>> = vec![vec![]];
    let mut current_monke: usize = 0;

    for line in input.lines() {
        if line == "" {
            current_monke += 1;
            monkes_input.push(vec![]);
        } else {
            monkes_input[current_monke].push(String::from(line));
        }
    }

    monkes_input
        .iter()
        .map(|input| create_monke(input))
        .collect()
}

fn create_monke(input: &Vec<String>) -> Monke {
    let items = input[1][18..]
        .split(", ")
        .map(|str| str.parse::<usize>().unwrap())
        .collect();
    let inspect_operator = match input[2].chars().nth(23).unwrap() {
        '*' => Operator::Multiply,
        '+' => Operator::Add,
        _ => panic!(),
    };
    let inspect_operator_value: OperatorValue = match &input[2][25..] {
        "old" => OperatorValue::OwnSelf,
        x => OperatorValue::Usize(String::from(x).parse::<usize>().unwrap()),
    };
    let inspect_operation = (inspect_operator, inspect_operator_value);
    let test_div = String::from(&input[3][21..]).parse::<usize>().unwrap();
    let test_true_i = String::from(&input[4][29..]).parse::<usize>().unwrap();
    let test_false_i = String::from(&input[5][30..]).parse::<usize>().unwrap();
    let test_addr = (test_true_i, test_false_i);

    Monke {
        items,
        inspect_operation,
        test_div,
        test_addr,
        inspect_count: 0,
    }
}

fn run_round(monkes: &mut Vec<Monke>, worry_level_div: bool) {
    let monke_product: usize = monkes.iter().map(|m| m.test_div).product();

    for monke_i in 0..monkes.len() {
        monkes[monke_i].inspect_count += monkes[monke_i].items.len();
        for item_i in 0..monkes[monke_i].items.len() {
            let operation_ingredient = match monkes[monke_i].inspect_operation.1 {
                OperatorValue::OwnSelf => monkes[monke_i].items[item_i].clone(),
                OperatorValue::Usize(x) => x,
            };

            let new_value = match monkes[monke_i].inspect_operation.0 {
                Operator::Add => {
                    if worry_level_div {
                        ((monkes[monke_i].items[item_i].clone()) + (operation_ingredient)) / 3
                    } else {
                        (monkes[monke_i].items[item_i].clone() % monke_product)
                            + (operation_ingredient)
                    }
                }
                Operator::Multiply => {
                    if worry_level_div {
                        (monkes[monke_i].items[item_i].clone() * (operation_ingredient)) / 3
                    } else {
                        (monkes[monke_i].items[item_i].clone() % monke_product)
                            * (operation_ingredient)
                    }
                }
            };

            if new_value.clone() % monkes[monke_i].test_div == 0 {
                let i = monkes[monke_i].test_addr.0;
                monkes[i].items.push(new_value);
            } else {
                let i = monkes[monke_i].test_addr.1;
                monkes[i].items.push(new_value);
            }
        }
        monkes[monke_i].items = vec![];
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let mut monkes_1 = monke_lines(input.clone());
    let mut monkes_2 = monke_lines(input);

    for _ in 0..20 {
        run_round(&mut monkes_1, true);
    }

    for _ in 0..10000 {
        run_round(&mut monkes_2, false);
    }

    monkes_1.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkes_2.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));

    let result_1 = monkes_1[0].inspect_count * monkes_1[1].inspect_count;
    let result_2 = monkes_2[0].inspect_count * monkes_2[1].inspect_count;

    println!("{}", result_1);
    println!("{}", result_2);
}
