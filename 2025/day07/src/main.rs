use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Field {
    Empty,
    Splitter,
}

fn parse_input() -> ((usize, usize), Vec<Vec<Field>>) {
    let mut start: (usize, usize) = (0, 0);

    let fields: Vec<Vec<Field>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        Field::Empty
                    }
                    '^' => Field::Splitter,
                    '.' => Field::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (start, fields)
}

fn count_splits(start: (usize, usize), board: &Vec<Vec<Field>>) -> usize {
    let mut beams: HashSet<(usize, usize)> = HashSet::from_iter(vec![start].iter().cloned());
    let mut current_level = 0;

    let mut splits = 0;

    while current_level < board.len() - 1 {
        let mut new_beams: HashSet<(usize, usize)> = HashSet::new();
        for (b_x, b_y) in beams.iter() {
            let (x, y) = (b_x.clone(), b_y + 1);
            if board[y][x] == Field::Splitter {
                new_beams.insert((x - 1, y));
                new_beams.insert((x + 1, y));
                splits += 1;
            } else {
                new_beams.insert((x, y));
            }
        }
        beams = new_beams;
        current_level += 1;
    }

    splits
}

fn count_timelines(
    pos: (usize, usize),
    board: &Vec<Vec<Field>>,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some((x, y)) = next_split_for(pos, board) {
        if let Some(cached_res) = cache.get(&(x, y)) {
            return cached_res.clone();
        }

        let res =
            count_timelines((x - 1, y), board, cache) + count_timelines((x + 1, y), board, cache);
        cache.insert((x, y), res);
        res
    } else {
        // Falls to the ground, no splitter in sight
        1
    }
}

fn next_split_for((x, y): (usize, usize), board: &Vec<Vec<Field>>) -> Option<(usize, usize)> {
    for i in (y + 1)..(board.len()) {
        if board[i][x] == Field::Splitter {
            return Some((x, i));
        }
    }

    None
}

fn main() {
    let (start, board) = parse_input();
    let splits = count_splits(start, &board);
    println!("{}", splits);

    let mut timelines_cache: HashMap<(usize, usize), usize> = HashMap::new();
    let timelines = count_timelines(start, &board, &mut timelines_cache);
    println!("{}", timelines)
}
