use std::collections::HashSet;
use std::fs;

type Board = Vec<Vec<char>>;
type Coords = (usize, usize);
type SymbolPartNumbersAndGearRatio = (Vec<u32>, u32);

fn parse_input(input: String) -> Board {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_numbers_and_gear_ratios_for_row(
    row: usize,
    board: &Board,
) -> Vec<SymbolPartNumbersAndGearRatio> {
    board[row]
        .iter()
        .enumerate()
        .map(|(column, c)| {
            if c.is_numeric() || c == &'.' {
                (vec![0 as u32], 0 as u32) as SymbolPartNumbersAndGearRatio
            } else {
                analyze_symbol((row, column), board)
            }
        })
        .collect()
}

fn analyze_symbol((row, column): Coords, board: &Board) -> SymbolPartNumbersAndGearRatio {
    let numbers = numbers_around_symbol((row, column), board);
    let is_gear: bool = board[row][column] == '*';

    let gear_ratio = if is_gear && numbers.len() == 2 {
        numbers.iter().product()
    } else {
        0
    };

    (numbers, gear_ratio)
}

fn numbers_around_symbol((row, column): Coords, board: &Board) -> Vec<u32> {
    let mut numbers_ranges: HashSet<(Coords, Coords)> = HashSet::new();

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 || is_out_of_bound((row, column), (i, j), board) {
                continue;
            }

            match look_for_number(
                (((row as i32) + i) as usize, ((column as i32) + j) as usize),
                board,
            ) {
                Some(range) => {
                    numbers_ranges.insert(range);
                }
                _ => (),
            }
        }
    }
    parse_numbers(&numbers_ranges, board)
}

fn parse_numbers(ranges: &HashSet<(Coords, Coords)>, board: &Board) -> Vec<u32> {
    ranges
        .iter()
        .map(|(beginning, end)| {
            let mut number = 0;
            for i in (beginning.1)..=(end.1) {
                number = number * 10 + board[beginning.0][i].to_digit(10).unwrap();
            }
            number
        })
        .collect::<Vec<u32>>()
}

fn look_for_number((row, column): Coords, board: &Board) -> Option<(Coords, Coords)> {
    if !board[row][column].is_numeric() {
        return None;
    }

    let mut beginning: Coords = (row, column);
    let mut end: Coords = (row, column);

    for i in (0..(beginning.1)).rev() {
        if !board[beginning.0][i].is_numeric() {
            break;
        }
        beginning.1 = i;
    }

    for i in (end.1 + 1)..board.len() {
        if !board[end.0][i].is_numeric() {
            break;
        }
        end.1 = i;
    }

    Some((beginning, end))
}

fn is_out_of_bound(
    (row, column): Coords,
    (modifier_row, modifier_column): (i32, i32),
    board: &Board,
) -> bool {
    let new_row = row as i32 + modifier_row;
    let new_column = column as i32 + modifier_column;

    new_row < 0
        || new_column < 0
        || new_row >= board.len() as i32
        || new_column >= board[0].len() as i32
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("File not loaded");

    let board = parse_input(input);
    let part_numbers_and_gear_ratios: Vec<SymbolPartNumbersAndGearRatio> = (0..board.len())
        .flat_map(|row| part_numbers_and_gear_ratios_for_row(row, &board))
        .collect::<_>();

    let part_numbers_sum: u32 = part_numbers_and_gear_ratios
        .clone()
        .iter()
        .map(|(numbers, _)| numbers.iter().sum::<u32>())
        .sum();

    let gear_ratios_sum: u32 = part_numbers_and_gear_ratios
        .iter()
        .map(|(_, ratio)| ratio)
        .sum();

    println!("{}", part_numbers_sum);
    println!("{}", gear_ratios_sum);
}
