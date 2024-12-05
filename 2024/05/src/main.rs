use std::collections::HashSet;
use std::fs;

struct Rule {
    before: usize,
    after: usize,
}

type Update = Vec<usize>;

fn parse_input(input: String) -> (Vec<Rule>, Vec<Update>) {
    let (rules_raw, updates_raw) = input.split_once("\n\n").unwrap();

    (parse_rules(rules_raw), parse_updates(updates_raw))
}

fn parse_rules(input: &str) -> Vec<Rule> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            Rule {
                before: a.parse::<usize>().unwrap(),
                after: b.parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn parse_updates(input: &str) -> Vec<Update> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|e| e.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn filter_updates(update: &Update, rules: &Vec<Rule>, get_incorrect: bool) -> bool {
    let mut seen: HashSet<usize> = HashSet::new();

    for u in update {
        for rule in rules.iter().filter(|r| r.before == *u || r.after == *u) {
            if seen.contains(&rule.after) {
                return get_incorrect;
            }
        }

        seen.insert(*u);
    }
    true && !get_incorrect
}

fn correct_update(update: &Update, rules: &Vec<Rule>) -> Update {
    let mut left: Update = update.clone();
    let mut res: Update = vec![];

    while !left.is_empty() {
        let index = left
            .iter()
            .position(|e| {
                left.iter()
                    .all(|ee| e == ee || !rules.iter().any(|r| r.before == *ee && r.after == *e))
            })
            .unwrap();
        res.push(left[index]);
        left.remove(index);
    }

    res
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let (rules, updates) = parse_input(input);
    let correct_updates = updates
        .iter()
        .filter(|u| filter_updates(u, &rules, false))
        .collect::<Vec<_>>();
    let result_1: usize = correct_updates.iter().map(|u| u[u.len() / 2]).sum();

    println!("{}", result_1);

    let incorrect_updates = updates
        .iter()
        .filter(|u| filter_updates(u, &rules, true))
        .map(|u| correct_update(u, &rules))
        .collect::<Vec<_>>();
    let result_2: usize = incorrect_updates.iter().map(|u| u[u.len() / 2]).sum();

    println!("{}", result_2);
}
