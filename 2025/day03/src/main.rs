use std::fs;

fn parse_input() -> Vec<Vec<u8>> {
    // fs::read_to_string("example.txt")
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - 48).collect())
        .collect()
}

fn max_bank_joltage_for_n_batteries(bank: &Vec<u8>, batteries: usize) -> usize {
    let mut res: Vec<u8> = vec![];
    let mut latest_largest: (isize, u8) = (-1, 0);

    for battery in (0..batteries).rev() {
        let mut max_curr = 0;
        for i in ((latest_largest.0 + 1) as usize)..(bank.len() - (battery)) {
            if bank[i] > max_curr {
                max_curr = bank[i];
                latest_largest = (i as isize, bank[i]);
            }
        }
        res.push(max_curr);
    }

    res.iter()
        .rev()
        .enumerate()
        .map(|(i, digit)| (10 as usize).pow(i as u32) * *digit as usize)
        .sum::<usize>()
}

fn main() {
    let banks = parse_input();
    let total_outputs_2 = banks
        .iter()
        .map(|b| max_bank_joltage_for_n_batteries(b, 2))
        .sum::<usize>();

    println!("{}", total_outputs_2);

    let total_outputs_12 = banks
        .iter()
        .map(|b| max_bank_joltage_for_n_batteries(b, 12))
        .sum::<usize>();

    println!("{}", total_outputs_12);
}
