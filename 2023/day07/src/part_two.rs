#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Debug)]
pub enum ShapeWithJoker {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Debug)]
enum HandWithJokerValue {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Clone, Debug)]
pub struct HandWithJoker {
    shapes: [ShapeWithJoker; 5],
    value: HandWithJokerValue,
}

impl Ord for HandWithJoker {
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

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandWithJoker {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for HandWithJoker {}

fn parse_line(line: &str) -> (HandWithJoker, u64) {
    let split = line.split_once(" ").unwrap();
    let hand = split.0;
    let value = split.1;

    (parse_hand(hand), value.parse().unwrap())
}

fn parse_hand(hand: &str) -> HandWithJoker {
    let mut collected: [u8; 13] = [0; 13];
    let mut shapes: [ShapeWithJoker; 5] = [ShapeWithJoker::Two; 5];
    hand.chars().enumerate().for_each(|(i, c)| {
        let shape = match c {
            '2' => ShapeWithJoker::Two,
            '3' => ShapeWithJoker::Three,
            '4' => ShapeWithJoker::Four,
            '5' => ShapeWithJoker::Five,
            '6' => ShapeWithJoker::Six,
            '7' => ShapeWithJoker::Seven,
            '8' => ShapeWithJoker::Eight,
            '9' => ShapeWithJoker::Nine,
            'T' => ShapeWithJoker::Ten,
            'J' => ShapeWithJoker::Joker,
            'Q' => ShapeWithJoker::Queen,
            'K' => ShapeWithJoker::King,
            'A' => ShapeWithJoker::Ace,
            _ => panic!("Invalid ShapeWithJoker"),
        };
        shapes[i] = shape;
        collected[shape as usize] += 1;
    });

    HandWithJoker {
        shapes,
        value: shapes_to_value(collected),
    }
}

fn shapes_to_value(collected_shapes: [u8; 13]) -> HandWithJokerValue {
    let jokers_count = collected_shapes[ShapeWithJoker::Joker as usize];
    let mut collected_without_jokers = collected_shapes.clone();
    collected_without_jokers[ShapeWithJoker::Joker as usize] = 0;

    if collected_shapes
        .iter()
        .any(|&x| x == 5 || x + jokers_count == 5)
    {
        HandWithJokerValue::Five
    } else if jokers_count == 4
        || collected_without_jokers
            .iter()
            .any(|&x| x == 4 || x + jokers_count == 4)
    {
        HandWithJokerValue::Four
    } else if collected_shapes.iter().any(|&x| x == 3) {
        if jokers_count == 2
            || collected_without_jokers
                .iter()
                .any(|&x| x == 2 || x + jokers_count == 2)
        {
            HandWithJokerValue::FullHouse
        } else {
            HandWithJokerValue::Three
        }
    } else if jokers_count == 3
        || collected_without_jokers
            .iter()
            .any(|&x| x + jokers_count == 3)
    {
        if collected_shapes.iter().filter(|&x| *x == 2).count() == 2 {
            HandWithJokerValue::FullHouse
        } else {
            HandWithJokerValue::Three
        }
    } else if collected_shapes.iter().filter(|&x| *x == 2).count() == 2
        || jokers_count >= 1 && collected_without_jokers.iter().any(|&x| x == 2)
    {
        HandWithJokerValue::TwoPair
    } else if collected_shapes.iter().any(|&x| x == 2) || jokers_count >= 1 {
        HandWithJokerValue::Pair
    } else {
        HandWithJokerValue::HighCard
    }
}
pub fn winnings_for_games(input: String) -> Vec<(HandWithJoker, u64)> {
    input.lines().map(|x| parse_line(x)).collect::<Vec<_>>()
}
