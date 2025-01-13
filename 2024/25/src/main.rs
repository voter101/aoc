use std::fs;

fn parse_input(input: String) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut keys = vec![];
    let mut locks = vec![];

    input.split("\n\n").for_each(|group| {
        let is_lock = group.starts_with("#");
        let mut element: Vec<usize> = vec![0; 5];
        group.lines().for_each(|line| {
            line.chars().enumerate().for_each(|(i, c)| {
                if c == '#' {
                    element[i] += 1;
                }
            })
        });

        if is_lock {
            locks.push(element);
        } else {
            keys.push(element);
        }
    });
    (keys, locks)
}

fn fitting_pairs(keys: &Vec<Vec<usize>>, locks: Vec<Vec<usize>>) -> usize {
    keys.iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| {
                    for i in 0..lock.len() {
                        if key[i] + lock[i] > 7 {
                            return false;
                        }
                    }
                    true
                })
                .count()
        })
        .sum::<usize>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let (keys, locks) = parse_input(input);
    println!("{}", fitting_pairs(&keys, locks));
}
