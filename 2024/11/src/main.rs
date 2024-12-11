use std::collections::HashMap;
use std::fs;

fn parse_input(input: String) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    input
        .split_whitespace()
        .for_each(|x| *res.entry(x.parse::<usize>().unwrap()).or_default() += 1);

    res
}

fn blink(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    for (k, v) in stones.clone() {
        if k == 0 {
            *res.entry(1).or_default() += v;
        } else {
            let num = k.to_string();
            if num.len() % 2 == 0 {
                let a = (&num[0..num.len() / 2]).parse::<usize>().unwrap();
                let b = (&num[num.len() / 2..]).parse::<usize>().unwrap();
                *res.entry(a).or_default() += v;
                *res.entry(b).or_default() += v;
            } else {
                *res.entry(k * 2024).or_default() += v;
            }
        }
    }

    res
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut stones = parse_input(input);

    let blinks_1 = 25;
    let blinks_2 = 75;

    for _ in 0..blinks_1 {
        stones = blink(stones);
    }

    println!("{}", stones.values().sum::<usize>());

    for _ in blinks_1..blinks_2 {
        stones = blink(stones);
    }

    println!("{}", stones.values().sum::<usize>());
}
