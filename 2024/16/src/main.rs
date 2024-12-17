use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

type Coords = (usize, usize);

#[derive(PartialEq)]
enum Fields {
    Wall,
    Path,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Map {
    fields: Vec<Vec<Fields>>,
    start: Coords,
    end: Coords,
}

fn parse_input(input: String) -> Map {
    let mut fields: Vec<Vec<Fields>> = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    input.lines().enumerate().for_each(|(y, line)| {
        let mut row: Vec<Fields> = vec![];

        line.chars().enumerate().for_each(|(x, c)| match c {
            '#' => row.push(Fields::Wall),
            '.' => row.push(Fields::Path),
            'S' => {
                row.push(Fields::Path);
                start = (x, y);
            }
            'E' => {
                row.push(Fields::Path);
                end = (x, y);
            }
            _ => {}
        });

        fields.push(row);
    });

    Map { fields, start, end }
}

impl Map {
    fn neighbours(&self, pos: Coords, dir: Direction) -> Vec<(Coords, Direction)> {
        if self.fields[pos.1][pos.0] == Fields::Wall {
            return vec![];
        }

        let to_check = match dir {
            Direction::Up => vec![
                ((0, -1), Direction::Up),
                ((1, 0), Direction::Right),
                ((-1, 0), Direction::Left),
            ],
            Direction::Right => vec![
                ((1, 0), Direction::Right),
                ((0, -1), Direction::Up),
                ((0, 1), Direction::Down),
            ],
            Direction::Down => vec![
                ((0, 1), Direction::Down),
                ((1, 0), Direction::Right),
                ((-1, 0), Direction::Left),
            ],
            Direction::Left => vec![
                ((-1, 0), Direction::Left),
                ((0, -1), Direction::Up),
                ((0, 1), Direction::Down),
            ],
        };

        to_check
            .iter()
            .map(|((dx, dy), next_dir)| {
                (
                    (
                        (pos.0 as isize + dx) as usize,
                        (pos.1 as isize + dy) as usize,
                    ),
                    next_dir.clone(),
                )
            })
            .filter(|p| self.fields[p.0 .1][p.0 .0] != Fields::Wall)
            .collect()
    }
}

fn path_cost(map: &Map) -> (usize, usize) {
    let mut costs: HashMap<(Coords, Direction), (usize, Vec<HashSet<Coords>>)> = HashMap::new();
    let mut stack: VecDeque<((Coords, Direction), usize, HashSet<Coords>)> =
        VecDeque::from(vec![(
            (map.start, Direction::Right),
            0,
            HashSet::from_iter(vec![map.start]),
        )]);

    while let Some(((pos, dir), cost, marked)) = stack.pop_front() {
        for (n, dir_n) in map.neighbours(pos, dir) {
            if marked.contains(&n) {
                continue;
            }

            let new_cost = cost + if dir == dir_n { 1 } else { 1001 };

            let init = (usize::MAX, vec![]);
            let (neighbour_cost, paths) = costs.get(&(n, dir_n)).unwrap_or(&init);

            if neighbour_cost < &new_cost {
                continue;
            }

            if neighbour_cost == &new_cost {
                let mut new_paths = paths.clone();
                new_paths.push(marked.clone());

                costs.insert((n, dir_n), (new_cost, new_paths));
            } else {
                let new_paths = vec![marked.clone()];
                costs.insert((n, dir_n), (new_cost, new_paths));
            }

            let new_marked = marked
                .union(&HashSet::from_iter(vec![n]))
                .cloned()
                .collect();
            stack.push_back(((n, dir_n), new_cost, new_marked));
        }
    }

    // I know it's messy...
    let paths: Vec<(usize, Vec<HashSet<Coords>>)> = vec![
        costs.get(&(map.end, Direction::Up)),
        costs.get(&(map.end, Direction::Right)),
        costs.get(&(map.end, Direction::Down)),
        costs.get(&(map.end, Direction::Left)),
    ]
    .iter()
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .cloned()
    .collect::<Vec<_>>();

    let min_score = paths.iter().map(|(score, _)| score).min().unwrap();
    let mut uniq_paths: HashSet<Coords> = HashSet::new();
    uniq_paths.insert(map.end);

    for (score, paths) in paths.iter() {
        if score != min_score {
            continue;
        }
        for path in paths.iter() {
            uniq_paths.extend(path);
        }
    }

    (*min_score, uniq_paths.len())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let map = parse_input(input);

    let (result_1, result_2) = path_cost(&map);
    println!("{}", result_1);
    println!("{}", result_2);
}
