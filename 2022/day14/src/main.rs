use std::cmp::max;
use std::collections::HashSet;
use std::fs;

fn create_board(input: String) -> (HashSet<(i32, i32)>, (i32, i32)) {
    let mut stones_map: HashSet<(i32, i32)> = HashSet::new();
    let mut board_size: (i32, i32) = (500, 0);

    for line in input.lines() {
        for window in line
            .split(" -> ")
            .map(|s| String::from(s))
            .collect::<Vec<String>>()
            .windows(2)
        {
            let from = input_str_to_coords(window[0].clone());
            let to = input_str_to_coords(window[1].clone());

            let x_max = max(from.0, to.0);
            let y_max = max(from.1, to.1);
            if x_max > board_size.0 {
                board_size.0 = x_max;
            }
            if y_max > board_size.1 {
                board_size.1 = y_max;
            }

            if from.0 == to.0 {
                let range = if from.1 > to.1 {
                    to.1..=from.1
                } else {
                    from.1..=to.1
                };
                for y in range {
                    stones_map.insert((from.0, y));
                }
            } else {
                let range = if from.0 > to.0 {
                    to.0..=from.0
                } else {
                    from.0..=to.0
                };
                for x in range {
                    stones_map.insert((x, from.1));
                }
            }
        }
    }
    (stones_map, board_size)
}

fn input_str_to_coords(input: String) -> (i32, i32) {
    let elements = input.split(",").collect::<Vec<&str>>();
    (
        elements[0].parse::<i32>().unwrap(),
        elements[1].parse::<i32>().unwrap(),
    )
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let (mut stones_map, board_size) = create_board(input);
    let sand_start: (i32, i32) = (500, 0);
    let mut sand_pieces_counter = 0;
    let mut new_sand_piece = sand_start.clone();

    loop {
        if new_sand_piece.1 > board_size.1 {
            break;
        }

        match stones_map.get(&(new_sand_piece.0, new_sand_piece.1 + 1)) {
            Some(_) => match stones_map.get(&(new_sand_piece.0 - 1, new_sand_piece.1 + 1)) {
                Some(_) => match stones_map.get(&(new_sand_piece.0 + 1, new_sand_piece.1 + 1)) {
                    Some(_) => {
                        stones_map.insert((new_sand_piece.0, new_sand_piece.1));
                        new_sand_piece = sand_start.clone();
                        sand_pieces_counter += 1;
                    }
                    None => {
                        new_sand_piece.0 += 1;
                        new_sand_piece.1 += 1;
                    }
                },
                None => {
                    new_sand_piece.0 -= 1;
                    new_sand_piece.1 += 1;
                }
            },
            None => {
                new_sand_piece.1 += 1;
            }
        };
    }

    println!("{}", sand_pieces_counter);

    for x in (-board_size.0 * 10)..(board_size.0 * 10) {
        stones_map.insert((x, board_size.1 + 2));
    }

   new_sand_piece = sand_start.clone();

    loop {
        match stones_map.get(&(new_sand_piece.0, new_sand_piece.1 + 1)) {
            Some(_) => match stones_map.get(&(new_sand_piece.0 - 1, new_sand_piece.1 + 1)) {
                Some(_) => match stones_map.get(&(new_sand_piece.0 + 1, new_sand_piece.1 + 1)) {
                    Some(_) => {
                        sand_pieces_counter += 1;
                        if new_sand_piece == sand_start {
                            break;
                        }
                        stones_map.insert((new_sand_piece.0, new_sand_piece.1));
                        new_sand_piece = sand_start.clone();
                    }
                    None => {
                        new_sand_piece.0 += 1;
                        new_sand_piece.1 += 1;
                    }
                },
                None => {
                    new_sand_piece.0 -= 1;
                    new_sand_piece.1 += 1;
                }
            },
            None => {
                new_sand_piece.1 += 1;
            }
        };
    }

    println!("{}", sand_pieces_counter);
}
