use std::collections::{HashSet, VecDeque};
use std::fs;

enum Field {
    Empty,
    SplitHorizontal,
    SplitVertical,
    MirrorSlash,
    MirrorBackslash,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Position {
    pos: (usize, usize),
    direction: Direction,
}

fn parse_input(input: &String) -> Vec<Vec<Field>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Field::Empty,
                    '-' => Field::SplitHorizontal,
                    '|' => Field::SplitVertical,
                    '/' => Field::MirrorSlash,
                    '\\' => Field::MirrorBackslash,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn shine_light_through(board: &Vec<Vec<Field>>, start_pos: &Position) -> Vec<Vec<bool>> {
    let mut light_board = vec![vec![false; board.len()]; board.len()];
    let mut queue: VecDeque<Position> = VecDeque::from(vec![start_pos.clone()]);
    let mut visited: HashSet<Position> = HashSet::new();

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();

        if visited.contains(&pos) {
            continue;
        }

        light_board[pos.pos.0][pos.pos.1] = true;
        visited.insert(pos);

        match board[pos.pos.0][pos.pos.1] {
            Field::Empty => {
                if let Some(next_position) = try_going(board, &pos, pos.direction) {
                    queue.push_back(next_position);
                }
            }
            Field::SplitHorizontal => match pos.direction {
                Direction::Up | Direction::Down => {
                    if let Some(next_position) = try_going(board, &pos, Direction::Left) {
                        queue.push_back(next_position);
                    }
                    if let Some(next_position) = try_going(board, &pos, Direction::Right) {
                        queue.push_back(next_position);
                    }
                }
                _ => {
                    if let Some(next_position) = try_going(board, &pos, pos.direction) {
                        queue.push_back(next_position)
                    }
                }
            },
            Field::SplitVertical => match pos.direction {
                Direction::Left | Direction::Right => {
                    if let Some(next_position) = try_going(board, &pos, Direction::Up) {
                        queue.push_back(next_position);
                    }
                    if let Some(next_position) = try_going(board, &pos, Direction::Down) {
                        queue.push_back(next_position);
                    }
                }
                _ => {
                    if let Some(next_position) = try_going(board, &pos, pos.direction) {
                        queue.push_back(next_position)
                    }
                }
            },
            Field::MirrorSlash => {
                let next_dir: Direction = match pos.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                if let Some(next_position) = try_going(board, &pos, next_dir) {
                    queue.push_back(next_position)
                }
            }
            Field::MirrorBackslash => {
                let next_dir: Direction = match pos.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                if let Some(next_position) = try_going(board, &pos, next_dir) {
                    queue.push_back(next_position)
                }
            }
        }
    }

    light_board
}

fn try_going(
    board: &Vec<Vec<Field>>,
    position: &Position,
    direction: Direction,
) -> Option<Position> {
    if is_in_bounds(
        board,
        &Position {
            pos: position.pos,
            direction,
        },
    ) {
        let next_pos = match direction {
            Direction::Up => (position.pos.0 - 1, position.pos.1),
            Direction::Down => (position.pos.0 + 1, position.pos.1),
            Direction::Left => (position.pos.0, position.pos.1 - 1),
            Direction::Right => (position.pos.0, position.pos.1 + 1),
        };

        Some(Position {
            pos: next_pos,
            direction,
        })
    } else {
        None
    }
}

fn is_in_bounds(board: &Vec<Vec<Field>>, position: &Position) -> bool {
    let pos = position.pos;
    match position.direction {
        Direction::Up => pos.0 > 0,
        Direction::Down => pos.0 < board.len() - 1,
        Direction::Left => pos.1 > 0,
        Direction::Right => pos.1 < board.len() - 1,
    }
}

fn shined_fields_count(light_board: &Vec<Vec<bool>>) -> usize {
    light_board
        .iter()
        .map(|row| row.iter().filter(|&x| *x).count())
        .sum::<usize>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input");
    let board = parse_input(&input);

    let energized_fields = shine_light_through(
        &board,
        &Position {
            pos: (0, 0),
            direction: Direction::Right,
        },
    );

    let energized_count = shined_fields_count(&energized_fields);

    println!("{}", energized_count);

    let mut current_max_count = energized_count;

    for i in 0..board.len() {
        for pos in [
            Position {
                pos: (i, 0),
                direction: Direction::Right,
            },
            Position {
                pos: (i, board.len() - 1),
                direction: Direction::Left,
            },
            Position {
                pos: (0, i),
                direction: Direction::Down,
            },
            Position {
                pos: (board.len() - 1, i),
                direction: Direction::Up,
            },
        ] {
            let val = shined_fields_count(&shine_light_through(&board, &pos));
            if val > current_max_count {
                current_max_count = val;
            }
        }
    }

    println!("{}", current_max_count);
}
