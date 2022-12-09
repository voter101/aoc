use std::cmp::{min};
use std::fs;

fn visible_trees (board: &Vec<Vec<usize>>, height: usize, width: usize) -> usize {
    let mut visibility_board: Vec<Vec<bool>> = vec![vec![false; width.clone()]; height.clone()];
    for row_x in 1..height - 1 {
        let mut curr_maximum = board[row_x][0];
        for row_y in 1..width - 1 {
            if board[row_x][row_y] > curr_maximum {
                visibility_board[row_x][row_y] = true;
                curr_maximum = board[row_x][row_y];
            }
        }

        curr_maximum = board[row_x][width - 1];
        for row_y in (1..width - 1).rev() {
            if board[row_x][row_y] > curr_maximum {
                visibility_board[row_x][row_y] = true;
                curr_maximum = board[row_x][row_y];
            }
        }
    }

    for row_y in 1..width - 1 {
        let mut curr_maximum = board[0][row_y];
        for row_x in 1..height - 1 {
            if board[row_x][row_y] > curr_maximum {
                visibility_board[row_x][row_y] = true;
                curr_maximum = board[row_x][row_y];
            }
        }

        curr_maximum = board[height - 1][row_y];
        for row_x in (1..height - 1).rev() {
            if board[row_x][row_y] > curr_maximum {
                visibility_board[row_x][row_y] = true;
                curr_maximum = board[row_x][row_y];
            }
        }
    }

    let mut result = height * 2 + width * 2 - 4;

    for row in visibility_board.iter() {
        for flag in row {
            if flag.clone() {
                result += 1;
            }
        }
    }
    result
}

fn max_scenic_tree(board: &Vec<Vec<usize>>, height: usize, width: usize) -> usize {
    let mut current_max = 0;
    
    for x in 1..height - 1 {
        for y in 1..width - 1 {
            let val = board[x][y];
            let mut current = 1;

            // Take while will skip 1 if it does stop not on the edge
            current *= min(
                (0..x).rev().take_while(|i| board[i.clone()][y] < val).count() + 1,
                (0..x).count()
            );
            current *= min(
                (x+1..height).take_while(|i| board[i.clone()][y] < val).count() + 1, 
                (x+1..height).count()
            );
            current *= min(
                (0..y).rev().take_while(|i| board[x][i.clone()] < val ).count() + 1, 
                (0..y).count()
            );
            current *= min(
                (y+1..width).take_while(|i| board[x][i.clone()] < val ).count() + 1, 
                (y+1..width).count()
            );

            if current > current_max {
                current_max = current;
            }
        }
    }
    

    current_max
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let height = input.lines().count();
    let width = input.lines().last().unwrap().len();
    let mut board: Vec<Vec<usize>> = vec![vec![0 as usize; width.clone()]; height.clone()];

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            board[x][y] = (c.to_digit(10).unwrap()) as usize;
        }
    }


    let result_1 = visible_trees(&board, height, width);
    let result_2 = max_scenic_tree(&board, height, width);

    println!("{:?}", result_1);
    println!("{:?}", result_2);
}
