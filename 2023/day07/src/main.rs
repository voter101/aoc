use std::fs;

use crate::part_two::winnings_for_games;

mod part_two;

#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Debug)]
enum Shape {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Debug)]
enum HandValue {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Clone, Debug)]
struct Hand {
    shapes: [Shape; 5],
    value: HandValue,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value == other.value {
            for i in 0..5 {
                if self.shapes[i] != other.shapes[i] {
                    return self.shapes[i].cmp(&other.shapes[i]);
                }
            }
            return std::cmp::Ordering::Equal;
        }
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Hand {}

fn parse_line(line: &str) -> (Hand, u64) {
    let split = line.split_once(" ").unwrap();
    let hand = split.0;
    let value = split.1;

    (parse_hand(hand), value.parse().unwrap())
}

fn parse_hand(hand: &str) -> Hand {
    let mut collected: [u8; 13] = [0; 13];
    let mut shapes: [Shape; 5] = [Shape::Two; 5];
    hand.chars().enumerate().for_each(|(i, c)| {
        let shape = match c {
            '2' => Shape::Two,
            '3' => Shape::Three,
            '4' => Shape::Four,
            '5' => Shape::Five,
            '6' => Shape::Six,
            '7' => Shape::Seven,
            '8' => Shape::Eight,
            '9' => Shape::Nine,
            'T' => Shape::Ten,
            'J' => Shape::Jack,
            'Q' => Shape::Queen,
            'K' => Shape::King,
            'A' => Shape::Ace,
            _ => panic!("Invalid shape"),
        };
        shapes[i] = shape;
        collected[shape as usize] += 1;
    });

    Hand {
        shapes,
        value: shapes_to_value(collected),
    }
}

fn shapes_to_value(collected_shapes: [u8; 13]) -> HandValue {
    if collected_shapes.iter().any(|&x| x == 5) {
        HandValue::Five
    } else if collected_shapes.iter().any(|&x| x == 4) {
        HandValue::Four
    } else if collected_shapes.iter().any(|&x| x == 3) {
        if collected_shapes.iter().any(|&x| x == 2) {
            HandValue::FullHouse
        } else {
            HandValue::Three
        }
    } else if collected_shapes.iter().filter(|&x| *x == 2).count() == 2 {
        HandValue::TwoPair
    } else if collected_shapes.iter().any(|&x| x == 2) {
        HandValue::Pair
    } else {
        HandValue::HighCard
    }
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("File not loaded");
    let mut hands_with_values: Vec<(Hand, u64)> = input
        .clone()
        .lines()
        .map(|x| parse_line(x))
        .collect::<Vec<_>>();
    let mut hands_with_jokers = winnings_for_games(input);

    hands_with_values.sort_by(|game1, game2| game1.0.cmp(&game2.0));
    hands_with_jokers.sort_by(|game1, game2| game1.0.cmp(&game2.0));

    let games_value = hands_with_values
        .iter()
        .enumerate()
        .map(|(i, (_, value))| value * (i as u64 + 1))
        .sum::<u64>();

    let games_with_joker_value = hands_with_jokers
        .iter()
        .enumerate()
        .map(|(i, (_, value))| value * (i as u64 + 1))
        .sum::<u64>();

    println!("{:?}", games_value);
    println!("{:?}", games_with_joker_value);
}
