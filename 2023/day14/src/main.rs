use std::fs;

type Board = Vec<Vec<Element>>;

#[derive(Clone, Copy, PartialEq)]
enum Element {
    Empty,
    Round,
    Square,
}

fn parse_input(input: &String) -> Board {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Element::Empty,
                    'O' => Element::Round,
                    '#' => Element::Square,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn tilt_north(board: &Board) -> Board {
    let mut new_board: Board = board.clone();

    let columns = board[0].len();

    for c in 0..columns {
        for i in 0..new_board.len() {
            if let Element::Round = new_board[i][c] {
                for j in (0..i).rev() {
                    if let Element::Empty = new_board[j][c] {
                        new_board[j][c] = Element::Round;
                        new_board[j + 1][c] = Element::Empty;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    new_board
}

fn board_total_load(board: &Board) -> usize {
    let rows = board.len();
    board
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|e| **e == Element::Round).count() * (rows - i))
        .sum()
}

fn spin_board(board: &Board) -> Board {
    let length = board.len();
    let mut new_board: Board = board.clone();

    for c in 0..length {
        new_board[c] = board.iter().map(|row| row[c]).rev().collect();
    }

    new_board
}

fn cycle_board(board: &Board, cycle_count: u64) -> Board {
    let mut board_cache: Vec<Board> = vec![];
    let mut new_board = board.clone();

    for i in 0..cycle_count {
        for _ in 0..4 {
            new_board = spin_board(&tilt_north(&new_board));
        }

        if let Some(index) = board_cache.iter().position(|b| *b == new_board) {
            let cycle_length = i - index as u64;
            let cycle_index = (cycle_count - i - 1) % cycle_length;
            return board_cache[index + cycle_index as usize].clone();
        }

        board_cache.push(new_board.clone());
    }

    new_board
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("File not loaded");
    let board = parse_input(&input);
    let tilted_board = tilt_north(&board);

    println!("{}", board_total_load(&tilted_board));

    let board_cycled = cycle_board(&board, 1000000000);

    println!("{}", board_total_load(&board_cycled));
}
