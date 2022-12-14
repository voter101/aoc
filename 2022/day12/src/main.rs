use std::collections::{HashMap, VecDeque};
use std::fs;

fn shortest_path_for_start(
    terrain: HashMap<(usize, usize), usize>,
    start: (usize, usize),
    end: (usize, usize),
    board_size: (usize, usize),
) -> usize {
    let mut nodes_cost: HashMap<(usize, usize), usize> = HashMap::from([(start.clone(), 0)]);
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::from([start.clone()]);

    while !to_visit.is_empty() {
        let visiting = to_visit.pop_back().unwrap();
        let height = terrain.get(&visiting).unwrap();
        let val = nodes_cost.get(&visiting).unwrap().clone();
        let mut positions_to_check: Vec<(usize, usize)> = vec![];

        if visiting.0 > 0 {
            positions_to_check.push((visiting.0 - 1, visiting.1));
        }
        if visiting.0 < board_size.0 {
            positions_to_check.push((visiting.0 + 1, visiting.1));
        }
        if visiting.1 > 0 {
            positions_to_check.push((visiting.0, visiting.1 - 1));
        }
        if visiting.1 < board_size.1 {
            positions_to_check.push((visiting.0, visiting.1 + 1));
        }
        positions_to_check = positions_to_check
            .iter()
            .filter(|pos| terrain.get(pos).unwrap().clone() <= height + 1)
            .cloned()
            .collect::<Vec<(usize, usize)>>();

        for position in positions_to_check {
            let new_cost = val + 1;

            let current_cost = nodes_cost.get(&position);

            match current_cost {
                Some(val) => {
                    if new_cost < val.clone() {
                        nodes_cost.entry(position).and_modify(|c| *c = new_cost);
                        to_visit.push_back(position);
                    }
                }
                None => {
                    to_visit.push_back(position);
                    nodes_cost.insert(position, new_cost);
                }
            };
        }
    }

    match nodes_cost.get(&end) {
        Some(val) => val.clone(),
        None => 999999
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");

    let mut terrain: HashMap<(usize, usize), usize> = HashMap::new();
    let mut start: (usize, usize) = (0, 0);
    let mut alternative_starts: Vec<(usize, usize)> = vec![];
    let mut end: (usize, usize) = (0, 0);

    let mut board_size = (0, 0);

    for (y, line) in input.lines().enumerate() {
        if y > board_size.1 {
            board_size.1 = y;
        }
        for (x, c) in line.chars().enumerate() {
            if x > board_size.0 {
                board_size.0 = x;
            }
            let val = match c {
                'S' => {
                    start = (x, y);
                    'a'
                }
                'E' => {
                    end = (x, y);
                    'z'
                }
                'a' => {
                    alternative_starts.push((x, y));
                    'a'
                }
                x => x,
            } as usize;
            terrain.insert((x, y), val);
        }
    }

    let shortest_path_first = shortest_path_for_start(terrain.clone(), start, end, board_size);
    // This is super slow, but servicable for the amount of data in the set.
    //
    // The alternative: Reverse the BFS without looking for the end and then iterate
    //                  through all positions in `terrain` that have the lowest
    //                  height. Caveat: make sure the movements is legal.
    let min_path = alternative_starts
        .iter()
        .map(|s| shortest_path_for_start(terrain.clone(), s.clone(), end, board_size))
        .min()
        .unwrap();

    println!("{}", shortest_path_first);
    println!("{}", min_path);
}
