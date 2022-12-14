use std::cmp::Ordering;
use std::fs;

#[derive(Clone, PartialEq)]
enum Value {
    Nested(Vec<Value>),
    Number(usize),
}

fn parse_line(line: &str) -> Value {
    let chars: Vec<_> = line.chars().collect();
    parse_list(chars).0
}

fn parse_list(chars: Vec<char>) -> (Value, usize) {
    let mut i = 1;
    let mut list_content: Vec<Value> = vec![];
    let mut number_acc: Vec<char> = vec![];

    while i < chars.len() {
        match chars[i] {
            ']' => {
                if !number_acc.is_empty() {
                    list_content.push(Value::Number(
                        String::from_iter(number_acc).parse().unwrap(),
                    ));
                }
                break;
            }
            ',' => {
                if !number_acc.is_empty() {
                    list_content.push(Value::Number(
                        String::from_iter(number_acc).parse().unwrap(),
                    ));
                    number_acc = vec![];
                }
            }
            '[' => {
                let (list, characters_to_skip) =
                    parse_list(chars[i..].iter().map(|e| e.clone()).collect());
                i += characters_to_skip;
                list_content.push(list);
            }
            x => {
                number_acc.push(x);
            }
        };
        i += 1;
    }

    (Value::Nested(list_content), i)
}

fn packet_order(left: Value, right: Value) -> Ordering {
    match (left.clone(), right.clone()) {
        (Value::Number(x), Value::Number(y)) => x.cmp(&y),
        (Value::Number(x), Value::Nested(y)) => {
            packet_order(Value::Nested(vec![Value::Number(x)]), Value::Nested(y))
        }
        (Value::Nested(x), Value::Number(y)) => {
            packet_order(Value::Nested(x), Value::Nested(vec![Value::Number(y)]))
        }
        (Value::Nested(x), Value::Nested(y)) => {
            if x.len() == 0 && y.len() == 0 {
                Ordering::Equal
            } else if x.len() == 0 {
                Ordering::Less
            } else if x.len() > 0 && y.len() == 0 {
                Ordering::Greater
            } else {
                match packet_order(x[0].clone(), y[0].clone()) {
                    Ordering::Equal => {
                        if x.len() > 1 && y.len() == 1 {
                            Ordering::Greater
                        } else if x.len() == 1 && y.len() > 1 {
                            Ordering::Less
                        } else {
                            packet_order(
                                Value::Nested(x[1..].to_vec()),
                                Value::Nested(y[1..].to_vec()),
                            )
                        }
                    }
                    order => order,
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let mut pairs: Vec<(Value, Value)> = vec![];

    input.split("\n\n").for_each(|lines_pair| {
        let lines: Vec<&str> = lines_pair.lines().collect();
        let left = lines[0];
        let right = lines[1];
        pairs.push((parse_line(left), parse_line(right)));
    });

    let result_1 = pairs
        .iter()
        .enumerate()
        .map(|(i, (left, right))| {
            if packet_order(left.clone(), right.clone()) == Ordering::Less {
                i + 1
            } else {
                0
            }
        })
        .sum::<usize>();

    let mut flat_packets: Vec<Value> = pairs.iter().fold(vec![], |mut acc, el| {
        acc.push(el.0.clone());
        acc.push(el.1.clone());
        acc
    });
    // Add "dividers"
    let divider_1 = Value::Nested(vec![Value::Nested(vec![Value::Number(2)])]);
    let divider_2 = Value::Nested(vec![Value::Nested(vec![Value::Number(6)])]);
    flat_packets.push(divider_1.clone());
    flat_packets.push(divider_2.clone());
    flat_packets.sort_by(|a, b| packet_order(a.clone(), b.clone()));

    let result_2 = (flat_packets.iter().position(|e| e.clone() == divider_1).unwrap() + 1) * (flat_packets.iter().position(|e| e.clone() == divider_2).unwrap() + 1);

    println!("{}", result_1);
    println!("{}", result_2);
}
