use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Comp {
    Less,
    Bigger,
}

#[derive(Debug)]
enum Rule {
    Comparison(char, Comp, usize, String),
    Address(String),
    Accept,
    Reject,
}

type Rules = HashMap<String, Vec<Rule>>;

fn parse_input(input: &String) -> (Rules, Vec<HashMap<char, usize>>) {
    let segments: Vec<&str> = input.split("\n\n").collect();

    (parse_rules(segments[0]), parse_parts(segments[1]))
}

fn parse_rules(input: &str) -> Rules {
    input
        .lines()
        .map(|rule| {
            let (name, non_trimmed_rules) = rule.split_once("{").unwrap();
            let rules: Vec<Rule> = non_trimmed_rules[0..(non_trimmed_rules.len() - 1)]
                .split(",")
                .map(|rule| {
                    match rule {
                        "A" => return Rule::Accept,
                        "R" => return Rule::Reject,
                        _ => (),
                    }

                    if rule.contains(":") {
                        let (comparison, destination) = rule.split_once(":").unwrap();
                        if rule.contains("<") {
                            let (symbol, value) = comparison.split_once("<").unwrap();
                            return Rule::Comparison(
                                symbol.chars().next().unwrap(),
                                Comp::Less,
                                value.parse::<usize>().unwrap(),
                                String::from(destination),
                            );
                        } else {
                            let (symbol, value) = comparison.split_once(">").unwrap();
                            return Rule::Comparison(
                                symbol.chars().next().unwrap(),
                                Comp::Bigger,
                                value.parse::<usize>().unwrap(),
                                String::from(destination),
                            );
                        }
                    } else {
                        return Rule::Address(rule.to_string());
                    }
                })
                .collect();
            (String::from(name), rules)
        })
        .collect()
}

fn parse_parts(input: &str) -> Vec<HashMap<char, usize>> {
    input
        .lines()
        .map(|line| {
            let trimmed_line = &line[1..(line.len() - 1)];
            trimmed_line
                .split(",")
                .map(|part| {
                    let split = part.split_once("=").unwrap();
                    let symbol = split.0.chars().next().unwrap();
                    let value = split.1.parse::<usize>().unwrap();
                    (symbol, value)
                })
                .collect()
        })
        .collect()
}

fn is_part_set_accepted(parts: &HashMap<char, usize>, rule: &str, rules: &Rules) -> bool {
    match rule {
        "A" => return true,
        "R" => return false,
        _ => (),
    }

    let conditions = rules.get(rule).unwrap();

    for rule in conditions {
        match rule {
            Rule::Comparison(symbol, comp, value, destination) => match comp {
                Comp::Less => {
                    if parts.get(symbol).unwrap() < value {
                        return is_part_set_accepted(parts, destination, rules);
                    }
                }
                Comp::Bigger => {
                    if parts.get(symbol).unwrap() > value {
                        return is_part_set_accepted(parts, destination, rules);
                    }
                }
            },
            Rule::Address(destination) => return is_part_set_accepted(parts, destination, rules),
            Rule::Accept => return true,
            Rule::Reject => return false,
        }
    }

    true
}

type Range = (usize, usize);
type Ranges = (Range, Range, Range, Range);

fn acceptable_ranges(rules: &Rules) -> usize {
    let mut solutions: Vec<Ranges> = vec![];

    let mut stack: Vec<(&str, Ranges)> = vec![("in", ((1, 4000), (1, 4000), (1, 4000), (1, 4000)))];

    while !stack.is_empty() {
        let (rule, mut ranges) = stack.pop().unwrap().clone();

        if rule == "A" {
            solutions.push(ranges);
            continue;
        }

        if rule == "R" {
            continue;
        }

        let conditions = rules.get(rule).unwrap();

        for condition in conditions.iter() {
            match condition {
                Rule::Address(destination) => stack.push((destination, ranges)),
                Rule::Comparison(symbol, comp, value, destination) => match comp {
                    Comp::Less => match symbol {
                        'x' => {
                            if ranges.0 .1 < *value {
                                stack.push((destination, ranges));
                            } else if ranges.0 .0 < *value {
                                stack.push((
                                    destination,
                                    ((ranges.0 .0, *value - 1), ranges.1, ranges.2, ranges.3),
                                ));
                                ranges.0 .0 = *value;
                            }
                        }
                        'm' => {
                            if ranges.1 .1 < *value {
                                stack.push((destination, ranges));
                            } else if ranges.1 .0 < *value {
                                stack.push((
                                    destination,
                                    (ranges.0, (ranges.1 .0, *value - 1), ranges.2, ranges.3),
                                ));
                                ranges.1 .0 = *value;
                            }
                        }
                        'a' => {
                            if ranges.2 .1 < *value {
                                stack.push((destination, ranges));
                            } else if ranges.2 .0 < *value {
                                stack.push((
                                    destination,
                                    (ranges.0, ranges.1, (ranges.2 .0, *value - 1), ranges.3),
                                ));
                                ranges.2 .0 = *value;
                            }
                        }
                        's' => {
                            if ranges.3 .1 < *value {
                                stack.push((destination, ranges));
                            } else if ranges.3 .0 < *value {
                                stack.push((
                                    destination,
                                    (ranges.0, ranges.1, ranges.2, (ranges.3 .0, *value - 1)),
                                ));
                                ranges.3 .0 = *value;
                            }
                        }
                        _ => (),
                    },
                    Comp::Bigger => match symbol {
                        'x' => {
                            if ranges.0 .0 > *value {
                                stack.push((destination, ranges));
                            } else if ranges.0 .1 > *value {
                                stack.push((
                                    destination,
                                    ((*value + 1, ranges.0 .1), ranges.1, ranges.2, ranges.3),
                                ));
                                ranges.0 .1 = *value;
                            }
                        }
                        'm' => {
                            if ranges.1 .0 > *value {
                                stack.push((destination, ranges));
                            } else if ranges.1 .1 > *value {
                                stack.push((
                                    destination,
                                    (ranges.0, (*value + 1, ranges.1 .1), ranges.2, ranges.3),
                                ));
                                ranges.1 .1 = *value;
                            }
                        }
                        'a' => {
                            if ranges.2 .0 > *value {
                                stack.push((destination, ranges));
                            } else if ranges.2 .1 > *value {
                                stack.push((
                                    destination,
                                    (ranges.0, ranges.1, (*value + 1, ranges.2 .1), ranges.3),
                                ));
                                ranges.2 .1 = *value;
                            }
                        }
                        's' => {
                            if ranges.3 .0 > *value {
                                stack.push((destination, ranges));
                            } else if ranges.3 .1 > *value {
                                stack.push((
                                    destination,
                                    (ranges.0, ranges.1, ranges.2, (*value + 1, ranges.3 .1)),
                                ));
                                ranges.3 .1 = *value;
                            }
                        }
                        _ => (),
                    },
                },
                Rule::Accept => stack.push(("A", ranges)),
                Rule::Reject => stack.push(("R", ranges)),
            }
        }
    }

    solutions
        .iter()
        .filter(|(x, m, a, s)| x.0 < x.1 && m.0 < m.1 && a.0 < a.1 && s.0 < s.1)
        .map(|((x1, x2), (m1, m2), (a1, a2), (s1, s2))| {
            (x2 - x1 + 1) * (m2 - m1 + 1) * (a2 - a1 + 1) * (s2 - s1 + 1)
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let (rules, parts) = parse_input(&input);
    let accepted_parts = parts
        .iter()
        .filter(|r| is_part_set_accepted(r, "in", &rules))
        .map(|e| e.clone())
        .collect::<Vec<HashMap<char, usize>>>();
    let parts_value = accepted_parts
        .iter()
        .map(|part| part.iter().map(|(_, val)| val).sum::<usize>())
        .sum::<usize>();

    println!("{}", parts_value);

    println!("{}", acceptable_ranges(&rules));
}
