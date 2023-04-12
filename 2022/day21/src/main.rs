use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map,
    sequence::tuple, IResult,
};
use std::collections::HashMap;
use std::fs;
use std::str;

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug)]
enum Monke {
    Number(i64),
    Operation {
        left: String,
        right: String,
        operator: Operator,
    },
}

fn solve(monkes: &HashMap<String, Monke>, monke: String) -> i64 {
    match monkes.get_key_value(&monke).unwrap().1 {
        Monke::Number(n) => n.clone(),
        Monke::Operation {
            left,
            right,
            operator,
        } => {
            let left = solve(monkes, left.clone());
            let right = solve(monkes, right.clone());
            match operator {
                Operator::Add => left + right,
                Operator::Sub => left - right,
                Operator::Mul => left * right,
                Operator::Div => left / right,
            }
        }
    }
}

fn solve_2(monkes: &HashMap<String, Monke>) -> i64 {
    let mut current_human = 1;
    let flip_sign = if diff_for_given_human(monkes, current_human) > 1 {
        -1
    } else {
        1
    };

    loop {
        let current_diff = diff_for_given_human(monkes, current_human) * flip_sign;

        if current_diff == 0 {
            return current_human;
        }

        let next_diff = diff_for_given_human(monkes, current_human * 2) * flip_sign;

        if next_diff > 0 {
            // We found the bounds. Binary search inside it
            let mut low = current_human;
            let mut high = current_human * 2;

            while low <= high {
                let middle = (((high + low) / 2) as f64).floor() as i64;

                let middle_diff = diff_for_given_human(monkes, middle) * flip_sign;

                if middle_diff == 0 {
                    // Sometimes many numbers can give the same solution (thanks to division)
                    for h in low..middle {
                        if diff_for_given_human(monkes, h) == 0 {
                            return h;
                        }
                    }
                    return middle;
                } else if middle_diff > 0 {
                    high = middle;
                } else {
                    low = middle;
                }
            }
        } else {
            current_human *= 2;
        }
    }
}

fn diff_for_given_human(monkes: &HashMap<String, Monke>, human: i64) -> i64 {
    let (left, right) = match monkes.get_key_value("root").unwrap().1 {
        Monke::Number(_) => unreachable!(),
        Monke::Operation { left, right, .. } => (left, right),
    };

    let mut monkes_temp: HashMap<String, Monke> = HashMap::new();
    monkes_temp.extend(monkes.into_iter().map(|(k, v)| {
        (
            k.clone(),
            if k == "humn" {
                Monke::Number(human)
            } else {
                v.clone()
            },
        )
    }));

    get_difference(&monkes_temp, left.clone(), right.clone())
}

fn get_difference(monkes: &HashMap<String, Monke>, key1: String, key2: String) -> i64 {
    solve(&monkes, key1.clone()) - solve(&monkes, key2.clone())
}

fn parse_line(line: &str) -> IResult<&str, (String, Monke)> {
    map(
        tuple((
            alpha1,
            tag(": "),
            alt((
                map(nom::character::complete::i64, Monke::Number),
                map(
                    tuple((
                        alpha1,
                        map(
                            alt((tag(" + "), tag(" - "), tag(" * "), tag(" / "))),
                            |s| match s {
                                " + " => Operator::Add,
                                " - " => Operator::Sub,
                                " * " => Operator::Mul,
                                " / " => Operator::Div,
                                _ => unreachable!(),
                            },
                        ),
                        alpha1,
                    )),
                    |(left, operator, right)| Monke::Operation {
                        left: String::from(left),
                        right: String::from(right),
                        operator,
                    },
                ),
            )),
        )),
        |(name, _, expression)| (String::from(name), expression),
    )(line)
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let monkes: HashMap<String, Monke> = input
        .lines()
        .map(|line| {
            let (_, (name, monke)) = parse_line(line).unwrap();
            (name, monke)
        })
        .collect::<HashMap<String, Monke>>();

    println!("{}", solve(&monkes, String::from("root")));
    println!("{}", solve_2(&monkes));
}
