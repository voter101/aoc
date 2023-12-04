use std::cmp::min;
use std::fs;

fn parse_input(input: String) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let game_numbers = line.split(":").last().unwrap();
            let (winning_numbers_raw, numbers_raw) = game_numbers.split_once(" | ").unwrap();
            (
                parse_numbers(winning_numbers_raw),
                parse_numbers(numbers_raw),
            )
        })
        .collect::<_>()
}

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .split(" ")
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<_>()
}

fn number_of_won_numbers((winning_numbers, numbers): &(&Vec<u32>, &Vec<u32>)) -> u32 {
    winning_numbers
        .iter()
        .filter(|n| numbers.contains(n))
        .count() as u32
}

fn count_won_scratchcards(won_games: &Vec<u32>) -> u32 {
    let mut scratchcards: Vec<u32> = vec![1; won_games.len()];
    won_games.iter().enumerate().for_each(|(i, won_numbers)| {
        for j in (i + 1)..(min((i + 1) as u32 + won_numbers, scratchcards.len() as u32) as usize) {
            scratchcards[j] += scratchcards[i];
        }
    });
    scratchcards.iter().sum::<_>()
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("File not loaded");
    let games = parse_input(input);

    let won_games = games
        .iter()
        .map(|(winning_numbers, numbers)| number_of_won_numbers(&(winning_numbers, numbers)))
        .collect::<Vec<_>>();

    let games_worth = won_games
        .clone()
        .iter()
        .filter(|won_numbers| **won_numbers > 0)
        .map(|won_numbers| 2u32.pow(won_numbers - 1))
        .sum::<u32>();

    let won_scratchcards = count_won_scratchcards(&won_games);

    println!("{:?}", games_worth);
    println!("{:?}", won_scratchcards);
}
