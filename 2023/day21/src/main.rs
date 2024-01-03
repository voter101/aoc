use std::collections::{HashSet, VecDeque};
use std::fs;

enum Field {
    Rock,
    Empty,
}

fn parse_input(input: String) -> (Vec<Vec<Field>>, (usize, usize)) {
    let mut start: (usize, usize) = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => Field::Rock,
                    '.' => Field::Empty,
                    'S' => {
                        start = (row, col);
                        Field::Empty
                    }
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<Field>>()
        })
        .collect::<Vec<Vec<Field>>>();

    (map, start)
}

fn fill_map(map: &Vec<Vec<Field>>, start: (usize, usize), step_count: usize) -> usize {
    let mut answer: HashSet<(usize, usize)> = HashSet::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize, usize)> =
        VecDeque::from([(start.0, start.1, step_count)]);

    while !queue.is_empty() {
        let (row, col, steps) = queue.pop_front().unwrap();

        if steps % 2 == 0 {
            answer.insert((row, col));
        }

        if steps == 0 {
            continue;
        }

        [
            (row as i32 - 1, col as i32),
            (row as i32 + 1, col as i32),
            (row as i32, col as i32 - 1),
            (row as i32, col as i32 + 1),
        ]
        .iter()
        .filter(|(r, c)| can_reach(*r, *c, map))
        .for_each(|(row, col)| {
            if !seen.contains(&(*row as usize, *col as usize)) {
                seen.insert((*row as usize, *col as usize));
                queue.push_back((*row as usize, *col as usize, steps - 1));
            }
        });
    }

    answer.len()
}

fn can_reach(row: i32, col: i32, map: &Vec<Vec<Field>>) -> bool {
    row >= 0
        && col >= 0
        && row < map.len() as i32
        && col < map[0].len() as i32
        && match map[row as usize][col as usize] {
            Field::Rock => false,
            Field::Empty => true,
        }
}

// Inspired by https://www.youtube.com/watch?v=9UOMZSL0JTg
fn solve_2(map: &Vec<Vec<Field>>, start: (usize, usize), steps_count: usize) -> usize {
    let map_len = map.len();
    let grid_width: usize = steps_count / map_len - 1;

    let odd_grids = (grid_width / 2 * 2 + 1).pow(2);
    let even_grids = ((grid_width + 1) / 2 * 2).pow(2);

    let points_on_odd_grid = fill_map(map, start, map_len * 2 + 1);
    let points_on_even_grid = fill_map(map, start, map_len * 2);

    let tip_top = fill_map(map, (map_len - 1, start.1), map_len - 1);
    let tip_right = fill_map(map, (start.0, 0), map_len - 1);
    let tip_bottom = fill_map(map, (0, start.1), map_len - 1);
    let tip_left = fill_map(map, (start.0, map_len - 1), map_len - 1);

    let small_overhead_top_right = fill_map(map, (map_len - 1, 0), map_len / 2 - 1);
    let small_overhead_top_left = fill_map(map, (map_len - 1, map_len - 1), map_len / 2 - 1);
    let small_overhead_bottom_right = fill_map(map, (0, 0), map_len / 2 - 1);
    let small_overhead_bottom_left = fill_map(map, (0, map_len - 1), map_len / 2 - 1);

    let big_overhead_top_right = fill_map(map, (map_len - 1, 0), map_len * 3 / 2 - 1);
    let big_overhead_top_left = fill_map(map, (map_len - 1, map_len - 1), map_len * 3 / 2 - 1);
    let big_overhead_bottom_right = fill_map(map, (0, 0), map_len * 3 / 2 - 1);
    let big_overhead_bottom_left = fill_map(map, (0, map_len - 1), map_len * 3 / 2 - 1);

    odd_grids * points_on_odd_grid
        + even_grids * points_on_even_grid
        + tip_top
        + tip_right
        + tip_bottom
        + tip_left
        + (grid_width + 1)
            * (small_overhead_top_left
                + small_overhead_top_right
                + small_overhead_bottom_left
                + small_overhead_bottom_right)
        + grid_width
            * (big_overhead_top_left
                + big_overhead_top_right
                + big_overhead_bottom_left
                + big_overhead_bottom_right)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input");
    let (map, start) = parse_input(input.clone());
    let result_1 = fill_map(&map, start, 64);

    println!("{}", result_1);

    if input.lines().count() == 131 {
        let result_2 = solve_2(&map, start, 26501365);

        println!("{}", result_2);
    }
}
