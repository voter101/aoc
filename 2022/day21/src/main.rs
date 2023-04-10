use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map,
    sequence::tuple, IResult,
};
use std::collections::HashMap;
use std::fs;
use std::str;

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
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

    let result = solve(&monkes, String::from("root"));

    println!("{}", result);
}
