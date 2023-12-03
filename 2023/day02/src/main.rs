use std::cmp::max;
use std::fs;

const LIMIT_RED: u32 = 12;
const LIMIT_GREEN: u32 = 13;
const LIMIT_BLUE: u32 = 14;

fn parse_segments(segments: Vec<&str>) -> Vec<(u32, u32, u32)> {
    segments
        .iter()
        .map(|s| {
            let elements: Vec<&str> = s.split(", ").map(|e| e.trim()).collect::<_>();
            let mut red: u32 = 0;
            let mut green: u32 = 0;
            let mut blue: u32 = 0;

            for element in elements {
                let (count, color) = element.split_once(" ").unwrap();
                let c = count.parse::<u32>().unwrap();
                match color {
                    "red" => red = c,
                    "green" => green = c,
                    "blue" => blue = c,
                    _ => unreachable!(),
                }
            }
            (red, green, blue)
        })
        .collect::<_>()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");

    let parsed_games = input
        .lines()
        .map(|line| line.split(":").last().unwrap())
        .map(|line| {
            let segments_raw = line.split(";").map(|e| e.trim()).collect::<Vec<&str>>();
            parse_segments(segments_raw)
        });

    let game_1_result: usize = parsed_games
        .clone()
        .map(|segments| {
            for segment in segments {
                let (red, green, blue) = segment;
                if red > LIMIT_RED || green > LIMIT_GREEN || blue > LIMIT_BLUE {
                    return false;
                }
            }
            return true;
        })
        .enumerate()
        .filter(|(_, e)| e.clone() == true)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let game_2_result: u32 = parsed_games
        .map(|segments| {
            let mut minimum_dice_set = (0, 0, 0);

            for (r, g, b) in segments {
                minimum_dice_set = (
                    max(minimum_dice_set.0, r),
                    max(minimum_dice_set.1, g),
                    max(minimum_dice_set.2, b),
                );
            }

            minimum_dice_set.0 * minimum_dice_set.1 * minimum_dice_set.2
        })
        .sum();

    println!("{}", game_1_result);
    println!("{}", game_2_result);
}
