use std::collections::HashMap;
use std::fs;

fn parse_input(input: String) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let split = line.split_once(" ").unwrap();

            let fields = split.0.chars().collect();
            let numbers = split
                .1
                .split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            (fields, numbers)
        })
        .collect()
}

fn expand_input(fields: &Vec<char>, numbers: &Vec<usize>) -> (Vec<char>, Vec<usize>) {
    let fields_expand = [fields; 5];
    let numbers_result = [numbers; 5]
        .iter()
        .flat_map(|e| e.clone())
        .map(|e: &usize| e.clone())
        .collect::<Vec<usize>>();

    let fields_result = fields_expand
        .iter()
        .map(|e| e.clone().clone())
        .reduce(|mut acc, field| {
            acc.extend(vec!['?']);
            acc.extend(field.clone());
            acc
        })
        .unwrap();

    (fields_result, numbers_result)
}

fn amount_of_solutions(
    fields: &Vec<char>,
    numbers: &Vec<usize>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if numbers.is_empty() {
        if fields_contain(&fields, '#') {
            return 0;
        } else {
            return 1;
        }
    }

    if fields.is_empty() {
        return 0;
    }

    let key = cache_key(fields, numbers);

    if cache.contains_key(&key) {
        return cache[&key];
    }

    if fields[0] == '?' {
        let skip_dot = amount_of_solutions(&fields[1..].to_vec(), numbers, cache);
        cache.insert(cache_key(&fields[1..].to_vec(), numbers), skip_dot);

        let potential_hash = !fields.iter().take(numbers[0]).any(|c| *c == '.')
            && ((fields.len() == numbers[0])
                || (fields.len() > numbers[0] && fields[numbers[0]] != '#'));

        let skip_hash = if potential_hash {
            let next_fields = if fields.len() > numbers[0] {
                fields[(numbers[0] + 1)..].to_vec()
            } else {
                vec![]
            };
            let val = amount_of_solutions(&next_fields, &numbers[1..].to_vec(), cache);
            cache.insert(cache_key(&next_fields, &numbers[1..].to_vec()), val);
            val
        } else {
            0
        };

        return skip_dot + skip_hash;
    } else {
        if fields[0] == '.' {
            let val = amount_of_solutions(&fields[1..].to_vec(), numbers, cache);
            cache.insert(cache_key(&fields[1..].to_vec(), numbers), val);
            return val;
        } else {
            let potential_hash = !fields.iter().take(numbers[0]).any(|c| *c == '.')
                && ((fields.len() == numbers[0])
                    || (fields.len() > numbers[0] && fields[numbers[0]] != '#'));

            return if potential_hash {
                let next_fields = if fields.len() > numbers[0] {
                    fields[(numbers[0] + 1)..].to_vec()
                } else {
                    vec![]
                };
                let val = amount_of_solutions(&next_fields, &numbers[1..].to_vec(), cache);
                cache.insert(cache_key(&next_fields, &numbers[1..].to_vec()), val);
                val
            } else {
                0
            };
        }
    }
}

fn cache_key(fields: &Vec<char>, numbers: &Vec<usize>) -> String {
    fields.iter().collect::<String>()
        + ":"
        + &numbers.iter().map(|n| n.to_string()).collect::<String>()
}

fn fields_contain(fields: &Vec<char>, field: char) -> bool {
    fields.iter().filter(|c| **c == field).count() > 0
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("File not loaded");
    let lines = parse_input(input);
    let result: usize = lines
        .iter()
        .map(|(fields, numbers)| {
            let mut cache: HashMap<String, usize> = HashMap::new();
            amount_of_solutions(fields, numbers, &mut cache)
        })
        .sum();

    println!("{:?}", result);

    let expanded_lines = lines
        .iter()
        .map(|(fields, numbers)| expand_input(fields, numbers))
        .collect::<Vec<(Vec<char>, Vec<usize>)>>();

    let result_expanded: usize = expanded_lines
        .iter()
        .map(|(fields, numbers)| {
            let mut cache: HashMap<String, usize> = HashMap::new();
            amount_of_solutions(fields, numbers, &mut cache)
        })
        .sum();

    println!("{:?}", result_expanded);
}
