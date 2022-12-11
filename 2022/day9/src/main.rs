use std::fs;
use std::collections::HashSet;
use text_io::scan;

fn make_moves(moves: &Vec<(String, i32)>, rope: &mut Vec<(i32, i32)>) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for m in moves {
        for _ in 0..m.1 {
            match m.0.as_str() {
                "U" => {
                    rope[0].1 += 1;
                },
                "D" => {
                    rope[0].1 -= 1;
                },
                "L" => {
                    rope[0].0 -= 1;
                },
                "R" => {
                    rope[0].0 += 1;
                },
                _ => unreachable!()
            };

            for i in 1..rope.len() {
                if let Some(coordinates) = move_tail(&rope[i], &rope[i - 1]) {
                    rope[i] = coordinates;
                } else {
                    break;
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn move_tail(tail: &(i32, i32), head: &(i32, i32)) -> Option<(i32, i32)> {
    let x_diff = tail.0 - head.0;
    let y_diff = tail.1 - head.1;
    let x_disconnected = x_diff.abs() > 1;
    let y_disconnected = y_diff.abs() > 1;

    if x_disconnected && y_disconnected {
        Some(((head.0 + x_diff.clamp(-1, 1)), (head.1 + y_diff.clamp(-1, 1))))
    } else if x_disconnected {
        Some(((head.0 + x_diff.clamp(-1, 1)), head.1))
    } else if y_disconnected {
        Some((head.0, (head.1 + y_diff.clamp(-1, 1))))
    } else {
        None
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");

    let mut rope_1: Vec<(i32, i32)> = vec![(0,0); 2];
    let mut rope_2: Vec<(i32, i32)> = vec![(0,0); 10];

    let instructions: Vec<(String, i32)> = input.lines().map(|line| {
        let direction: String;
        let count: u32;
        scan!(line.bytes() => "{} {}", direction, count);
        (direction, count as i32)
    }).collect();

    let result_1 = make_moves(&instructions, &mut rope_1);
    let result_2 = make_moves(&instructions, &mut rope_2);

    println!("{}", result_1);
    println!("{}", result_2);
}
