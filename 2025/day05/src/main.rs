use std::fs;
use std::ops::RangeInclusive;

fn is_overlapping(r_1: &RangeInclusive<usize>, r_2: &RangeInclusive<usize>) -> bool {
    r_1.start() <= r_2.end() && r_2.start() <= r_1.end()
}

fn merge(r_1: &RangeInclusive<usize>, r_2: &RangeInclusive<usize>) -> RangeInclusive<usize> {
    (*r_1.start().min(r_2.start()))..=(*r_1.end().max(r_2.end()))
}

fn parse_input() -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let input = fs::read_to_string("input.txt").unwrap();

    let (ranges_input, ingredients_input) = input.split_once("\n\n").unwrap();

    let ranges = ranges_input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("-").unwrap();
            (l.parse::<usize>().unwrap())..=(r.parse::<usize>().unwrap())
        })
        .collect();

    let ingredients = ingredients_input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    (ranges, ingredients)
}

fn unique_ranges_count(ranges: &Vec<RangeInclusive<usize>>) -> usize {
    let mut ranges_current = ranges.clone();

    'overlaps: loop {
        for i in 0..ranges_current.len() {
            for j in (i + 1)..ranges_current.len() {
                if is_overlapping(&ranges_current[i], &ranges_current[j]) {
                    ranges_current.push(merge(&ranges_current[i], &ranges_current[j]));
                    ranges_current.remove(j);
                    ranges_current.remove(i);
                    continue 'overlaps;
                }
            }
        }

        break;
    }

    let max = ranges_current
        .into_iter()
        .map(|r| r.clone().count())
        .sum::<usize>();

    max
}

fn main() {
    let (fresh_ranges, ingredients) = parse_input();

    let fresh_ingredients = ingredients
        .iter()
        .filter(|i| fresh_ranges.iter().any(|r| r.contains(i)))
        .count();

    println!("{}", fresh_ingredients);

    let part_2 = unique_ranges_count(&fresh_ranges);

    println!("{}", part_2);
}
