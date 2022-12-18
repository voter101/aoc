use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

fn parse_input(input: &String) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|c| {
            if c == '>' {
                Some(Direction::Right)
            } else if c == '<' {
                Some(Direction::Left)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn create_shapes() -> Vec<Vec<(u64, u64)>> {
    vec![
        vec![(2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(3, 2), (2, 1), (3, 1), (4, 1), (3, 0)],
        vec![(4, 2), (4, 1), (2, 0), (3, 0), (4, 0)],
        vec![(2, 3), (2, 2), (2, 1), (2, 0)],
        vec![(2, 1), (3, 1), (2, 0), (3, 0)],
    ]
}

fn spawn_block(block: &Vec<(u64, u64)>, board_height: u64) -> Vec<(u64, u64)> {
    let new_height = board_height + 3;
    block
        .iter()
        .map(|point| (point.0, point.1 + new_height))
        .collect()
}

fn move_block(
    direction: &Direction,
    block: &mut Vec<(u64, u64)>,
    board: &HashSet<(u64, u64)>,
) -> bool {
    match direction {
        Direction::Down => {
            if block
                .iter()
                .any(|point| point.1 == 0 || board.contains(&(point.0, point.1 - 1)))
            {
                return false;
            }

            for point in block.iter_mut() {
                point.1 -= 1;
            }
        }
        Direction::Left => {
            if block
                .iter()
                .any(|point| point.0 == 0 || board.contains(&(point.0 - 1, point.1)))
            {
                return false;
            }

            for point in block.iter_mut() {
                point.0 -= 1;
            }
        }
        Direction::Right => {
            if block
                .iter()
                .any(|point| point.0 == 6 || board.contains(&(point.0 + 1, point.1)))
            {
                return false;
            }

            for point in block.iter_mut() {
                point.0 += 1;
            }
        }
    }
    true
}

fn put_block_on_board(block: &Vec<(u64, u64)>, board: &mut HashSet<(u64, u64)>) {
    for point in block {
        board.insert(point.clone());
    }
}

fn simulate_board(to_drop: u64, directions: &Vec<Direction>) -> u64 {
    let mut directions_iter = directions.iter().cycle();
    let mut board: HashSet<(u64, u64)> = HashSet::new();
    let mut board_dim: (u64, u64) = (6, 0);
    let shapes = create_shapes();
    let mut shapes_iter = shapes.iter().cycle();

    let mut dropped: u64 = 0;
    let mut directions_i = 0;

    let mut cache: HashMap<(usize, usize, Vec<usize>), (u64, u64)> = HashMap::new();

    let mut cycles_height: u64 = 0;
    while dropped < to_drop {
        let block_base = shapes_iter.next().unwrap();
        let mut block = spawn_block(&(block_base.clone()), board_dim.1);

        loop {
            directions_i = (directions_i + 1) % directions.len();

            if let Some(direction) = directions_iter.next() {
                move_block(direction, &mut block, &board);
            }

            if !move_block(&Direction::Down, &mut block, &board) {
                break;
            }
        }

        put_block_on_board(&block, &mut board);

        let block_top_height = block.iter().max_by_key(|point| point.1).unwrap().1 + 1;

        if block_top_height > board_dim.1 {
            board_dim.1 = block_top_height;
        }

        // Hack: Check up to 50 (if available) rows... for similar looking fragments of the board
        //       There _might_ be some inputs where this WON'T work. But we can increase 50 until we
        //       get "stable" enough.
        // Idea: Maybe `directions.len() * 2`?
        let last_rows_val = ((board_dim.1 - 50)..board_dim.1)
            .into_iter()
            .map(|y| {
                (0..=board_dim.0)
                    .into_iter()
                    .map(|x| if board.contains(&(x, y)) { '1' } else { '0' })
                    .collect::<String>()
            })
            .map(|s| usize::from_str_radix(s.as_str(), 2).unwrap())
            .collect::<Vec<_>>();
        let cache_key = (
            directions_i,
            shapes.iter().position(|shape| shape == block_base).unwrap(),
            last_rows_val,
        );

        if cache.contains_key(&cache_key) {
            let cache_element = cache.get(&cache_key).unwrap();

            let blocks_in_cycle = dropped - cache_element.0;
            let cycle_height_change = board_dim.1 - cache_element.1;

            let cycles = ((to_drop - cache_element.0) / blocks_in_cycle) - 1;
            cycles_height += cycles * cycle_height_change;
            dropped += cycles * blocks_in_cycle;
        } else {
            cache.insert(cache_key, (dropped, board_dim.1));
        }
        dropped += 1;
    }
    board_dim.1 + cycles_height
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let directions = parse_input(&input);
    println!("{}", simulate_board(2022, &directions));
    println!("{}", simulate_board(1000000000000, &directions))
}
