use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input() -> Vec<(usize, usize)> {
    fs::read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(|range| {
            let (lo, hi) = range.split_once("-").unwrap();
            (
                lo.parse::<usize>().unwrap(),
                hi.trim().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn possible_halves_sequence_lengths(lo: usize, hi: usize) -> Vec<usize> {
    let lo_len = lo.to_string().len();
    let hi_len = hi.to_string().len();

    if hi_len < 2 {
        return vec![];
    }

    let mut res: Vec<usize> = vec![];

    for num_len in lo_len..=hi_len {
        if num_len % 2 == 0 {
            res.push(num_len / 2);
        }
    }

    res
}

fn possible_sequence_lengths(lo: usize, hi: usize) -> HashMap<usize, Vec<usize>> {
    let lo_len = lo.to_string().len();
    let hi_len = hi.to_string().len();

    if hi_len < 2 {
        return HashMap::new();
    }

    let mut res: HashMap<usize, Vec<usize>> = HashMap::new();

    for num_len in lo_len..=hi_len {
        res.insert(num_len, divisors(num_len));
    }

    res
}

fn divisors(num: usize) -> Vec<usize> {
    (1..=(num / 2 + 1))
        .filter_map(|i| {
            // Discard yourself too
            if num % i == 0 && i != num {
                return Some(i);
            }
            None
        })
        .collect()
}

// A and B variants are using a lot of duplication
fn invalid_ids_a(lo: usize, hi: usize) -> Vec<usize> {
    let mut results: Vec<usize> = vec![];
    let base: usize = 10;

    for seq in possible_halves_sequence_lengths(lo, hi) {
        'sequence_loop: for i in (base.pow(seq as u32 - 1))..base.pow(seq as u32) {
            let candidate = vec![i.to_string(); 2].join("").parse::<usize>().unwrap();

            if candidate > hi {
                break 'sequence_loop;
            }

            if candidate >= lo {
                results.push(candidate);
            }
        }
    }

    results
}

fn invalid_ids_b(lo: usize, hi: usize) -> HashSet<usize> {
    let mut results: HashSet<usize> = HashSet::new();
    let base: usize = 10;

    for (num_len, possible_seq) in possible_sequence_lengths(lo, hi) {
        for seq in possible_seq {
            'sequence_loop: for i in (base.pow(seq as u32 - 1))..base.pow(seq as u32) {
                let candidate = vec![i.to_string(); num_len / seq]
                    .join("")
                    .parse::<usize>()
                    .unwrap();

                if candidate > hi {
                    break 'sequence_loop;
                }

                if candidate >= lo {
                    results.insert(candidate);
                }
            }
        }
    }

    results
}

fn main() {
    let ranges = parse_input();

    let invalid_sum_a = ranges
        .iter()
        .map(|(lo, hi)| invalid_ids_a(*lo, *hi))
        .flatten()
        .sum::<usize>();

    println!("{}", invalid_sum_a);

    let invalid_sum_b = ranges
        .iter()
        .map(|(lo, hi)| invalid_ids_b(*lo, *hi))
        .flatten()
        .sum::<usize>();

    println!("{}", invalid_sum_b);
}
