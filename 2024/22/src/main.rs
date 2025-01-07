use std::collections::HashMap;
use std::fs;

fn iterate_secret(
    input: usize,
    iters: usize,
) -> (usize, HashMap<(isize, isize, isize, isize), usize>) {
    let mut res = input;
    let mut seen: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();

    let mut last_price = input;
    let mut last_prices_diffs: Vec<isize> = vec![];

    for i in 0..iters {
        let next = next_secret(res);
        let next_price = price(next);
        let diff = next_price as isize - last_price as isize;

        last_prices_diffs.push(diff);

        if i >= 4 {
            last_prices_diffs.remove(0);
            let seen_key = (
                last_prices_diffs[0],
                last_prices_diffs[1],
                last_prices_diffs[2],
                last_prices_diffs[3],
            );

            if !seen.contains_key(&seen_key) {
                seen.insert(seen_key, next_price);
            }
        }

        last_price = next_price;
        res = next;
    }

    (res, seen)
}

fn price(num: usize) -> usize {
    num % 10
}

fn next_secret(num: usize) -> usize {
    let mut res = num;
    res = (res ^ (res * 64)).rem_euclid(16777216);
    res = (res ^ (res / 32)).rem_euclid(16777216);
    res = (res ^ (res * 2048)).rem_euclid(16777216);
    res
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let res = input
        .iter()
        .map(|n| iterate_secret(*n, 2000))
        .collect::<Vec<_>>();

    let part_1 = res.iter().map(|(secret, _)| secret).sum::<usize>();

    println!("{:?}", part_1);

    let mut part_2_acc: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();

    res.iter().for_each(|(_, seen)| {
        seen.iter().for_each(|(k, v)| {
            part_2_acc.entry(*k).and_modify(|e| *e += v).or_insert(*v);
        });
    });

    let part_2: usize = part_2_acc.iter().map(|(_, v)| v).max().unwrap().clone();

    println!("{:?}", part_2);
}
