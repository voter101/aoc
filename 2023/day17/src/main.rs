use std::collections::{HashMap, VecDeque};
use std::fs;

type Board = Vec<Vec<usize>>;
type Coords = (i32, i32);
type Point = (Coords, usize, Direction);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Horizontal,
    Vertical,
}

fn parse_input(input: String) -> Board {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn min_path(min_step: usize, max_step: usize, board: &Board) -> usize {
    let mut cache: HashMap<(Coords, Direction), usize> = HashMap::new();
    cache.insert(((0, 0), Direction::Horizontal), 0);
    cache.insert(((0, 0), Direction::Vertical), 0);

    let mut queue: VecDeque<Point> = VecDeque::from(vec![
        ((0, 0), 0, Direction::Horizontal),
        ((0, 0), 0, Direction::Vertical),
    ]);

    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();

        for neighbour in neighbours(point, min_step, max_step, board) {
            if !cache.contains_key(&(neighbour.0, neighbour.2)) {
                cache.insert((neighbour.0, neighbour.2), neighbour.1);
                queue.push_back(neighbour);
            } else {
                let current_cost = cache.get(&(neighbour.0, neighbour.2)).unwrap();
                if neighbour.1 < *current_cost {
                    cache.insert((neighbour.0, neighbour.2), neighbour.1);
                    queue.push_back(neighbour);
                }
            }
        }
    }

    let min_horizontal = cache
        .get(&(
            (board.len() as i32 - 1, board[0].len() as i32 - 1),
            Direction::Horizontal,
        ))
        .unwrap_or(&usize::max_value());
    let min_vertical = cache
        .get(&(
            (board.len() as i32 - 1, board[0].len() as i32 - 1),
            Direction::Vertical,
        ))
        .unwrap_or(&usize::max_value());

    std::cmp::min(min_horizontal.clone(), min_vertical.clone())
}

fn neighbours(point: Point, min_step: usize, max_step: usize, board: &Board) -> Vec<Point> {
    match point.2 {
        Direction::Horizontal => horizontal_neighbours(point, min_step, max_step, board),
        Direction::Vertical => vertical_neighbours(point, min_step, max_step, board),
    }
}

fn horizontal_neighbours(
    point: Point,
    min_step: usize,
    max_step: usize,
    board: &Board,
) -> Vec<Point> {
    let mut result: Vec<Point> = vec![];

    for i in min_step..=max_step {
        let (row, col) = point.0;

        let up = (row - i as i32, col);
        let down = (row + i as i32, col);

        if is_in_bounds(up, board) {
            let path_cost: usize = (up.0..point.0 .0)
                .map(|r| board[r as usize][col as usize])
                .sum();

            result.push((up, point.1 + path_cost, Direction::Vertical));
        }

        if is_in_bounds(down, board) {
            let path_cost: usize = ((point.0 .0 + 1)..=down.0)
                .map(|r| board[r as usize][col as usize])
                .sum();
            result.push((down, point.1 + path_cost, Direction::Vertical));
        }
    }

    result
}

fn vertical_neighbours(
    point: Point,
    min_step: usize,
    max_step: usize,
    board: &Board,
) -> Vec<Point> {
    let mut result: Vec<Point> = vec![];

    for i in min_step..=max_step {
        let (row, col) = point.0;

        let left = (row, col - i as i32);
        let right = (row, col + i as i32);

        if is_in_bounds(left, board) {
            let path_cost: usize = (left.1..point.0 .1)
                .map(|c| board[row as usize][c as usize])
                .sum();

            result.push((left, point.1 + path_cost, Direction::Horizontal));
        }

        if is_in_bounds(right, board) {
            let path_cost: usize = ((point.0 .1 + 1)..=right.1)
                .map(|c| board[row as usize][c as usize])
                .sum();
            result.push((right, point.1 + path_cost, Direction::Horizontal));
        }
    }

    result
}

fn is_in_bounds(coords: Coords, board: &Board) -> bool {
    let (row, col) = coords;

    row >= 0 && row < board.len() as i32 && col >= 0 && col < board[0].len() as i32
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input");
    let board = parse_input(input);

    let result_1 = min_path(1, 3, &board);
    println!("{}", result_1);

    let result_2 = min_path(4, 10, &board);
    println!("{}", result_2);
}
