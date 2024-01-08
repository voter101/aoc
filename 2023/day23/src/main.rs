use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Clone, Copy, Debug)]
enum Field {
    Tree,
    Open,
    SlopeUp,
    SlopeRight,
    SlopeDown,
    SlopeLeft,
}

type Coords = (usize, usize);

fn parse_input(input: String) -> Vec<Vec<Field>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Field::Tree,
                    '.' => Field::Open,
                    '^' => Field::SlopeUp,
                    '>' => Field::SlopeRight,
                    'v' => Field::SlopeDown,
                    '<' => Field::SlopeLeft,
                    _ => panic!("Invalid character in input"),
                })
                .collect()
        })
        .collect()
}

fn find_paths(map: &Vec<Vec<Field>>) -> Vec<usize> {
    let start = (0, 1);
    let end = (map.len() - 1, map[0].len() - 2);
    let mut paths: Vec<usize> = vec![];

    let mut queue: VecDeque<(Coords, usize, HashSet<Coords>)> = VecDeque::new();

    queue.push_back((start, 0, HashSet::new()));

    while !queue.is_empty() {
        let (coords, steps, visited) = queue.pop_front().unwrap();

        if coords == end {
            paths.push(steps);
            continue;
        }

        if visited.contains(&coords) {
            continue;
        }

        let mut new_visited = visited.clone();
        new_visited.insert(coords);

        match map[coords.0][coords.1] {
            Field::Tree => continue,
            Field::Open => {
                if coords.0 < map.len() - 1 {
                    queue.push_back(((coords.0 + 1, coords.1), steps + 1, new_visited.clone()));
                }
                if coords.0 > 0 {
                    queue.push_back(((coords.0 - 1, coords.1), steps + 1, new_visited.clone()));
                }
                if coords.1 < map.len() - 1 {
                    queue.push_back(((coords.0, coords.1 + 1), steps + 1, new_visited.clone()));
                }
                if coords.1 > 0 {
                    queue.push_back(((coords.0, coords.1 - 1), steps + 1, new_visited.clone()));
                }
            }
            Field::SlopeUp => {
                queue.push_back(((coords.0 - 1, coords.1), steps + 1, new_visited.clone()));
            }
            Field::SlopeRight => {
                queue.push_back(((coords.0, coords.1 + 1), steps + 1, new_visited.clone()));
            }
            Field::SlopeDown => {
                queue.push_back(((coords.0 + 1, coords.1), steps + 1, new_visited.clone()));
            }
            Field::SlopeLeft => {
                queue.push_back(((coords.0, coords.1 - 1), steps + 1, new_visited.clone()));
            }
        }
    }

    paths
}

fn find_junctions(map: &Vec<Vec<Field>>) -> HashSet<Coords> {
    let mut junctions: HashSet<Coords> = HashSet::new();

    let start = (0, 1);
    let end = (map.len() - 1, map[0].len() - 2);
    junctions.insert(start);
    junctions.insert(end);

    for row in 1..map.len() {
        for col in 1..map[row].len() {
            match map[row][col] {
                Field::Tree => {}
                _ => {
                    let n = neighbours(map, (row, col));

                    if n.len() > 2 {
                        junctions.insert((row, col));
                    }
                }
            }
        }
    }

    junctions
}

fn find_distances(
    junctions: &HashSet<Coords>,
    map: &Vec<Vec<Field>>,
) -> HashMap<Coords, HashMap<Coords, usize>> {
    let mut distances: HashMap<Coords, HashMap<Coords, usize>> = HashMap::new();

    for junction in junctions.iter() {
        distances.insert(*junction, HashMap::new());
    }

    for junction in junctions.iter() {
        let mut queue: VecDeque<(Coords, usize, HashSet<Coords>)> = VecDeque::new();

        queue.push_back((junction.clone(), 0, HashSet::new()));

        while !queue.is_empty() {
            let (coords, steps, visited) = queue.pop_front().unwrap();

            if visited.contains(&coords) {
                continue;
            }

            if junctions.contains(&coords) && coords != *junction {
                let to_update = distances.get_mut(junction).unwrap();
                to_update.insert(coords, steps);

                let to_update_other = distances.get_mut(&coords).unwrap();
                to_update_other.insert(*junction, steps);

                continue;
            }

            let mut new_visited = visited.clone();
            new_visited.insert(coords);

            match map[coords.0][coords.1] {
                Field::Tree => continue,
                Field::Open
                | Field::SlopeUp
                | Field::SlopeRight
                | Field::SlopeDown
                | Field::SlopeLeft => neighbours(map, coords).iter().for_each(|neighbour| {
                    queue.push_back((neighbour.clone(), steps + 1, new_visited.clone()));
                }),
            }
        }
    }

    distances
}

fn find_scenic_path(
    simplified_map: &HashMap<Coords, HashMap<Coords, usize>>,
    map_dim: usize,
) -> usize {
    let start = (0, 1);
    let end = (map_dim - 1, map_dim - 2);

    let mut result: Vec<usize> = vec![];

    let mut queue: VecDeque<(Coords, usize, HashSet<Coords>)> = VecDeque::new();

    queue.push_back((start, 0, HashSet::new()));

    while !queue.is_empty() {
        let (coords, steps, visited) = queue.pop_front().unwrap();

        if coords == end {
            result.push(steps);
            continue;
        }

        if visited.contains(&coords) {
            continue;
        }

        let mut new_visited = visited.clone();
        new_visited.insert(coords);

        simplified_map
            .get(&coords)
            .unwrap()
            .iter()
            .for_each(|neighbour| {
                queue.push_back((
                    neighbour.0.clone(),
                    steps + neighbour.1,
                    new_visited.clone(),
                ));
            });
    }

    result.iter().max().unwrap().clone()
}

fn neighbours(map: &Vec<Vec<Field>>, coords: Coords) -> Vec<Coords> {
    let mut result: Vec<Coords> = vec![];
    if coords.0 < map.len() - 1 {
        match map[coords.0 + 1][coords.1] {
            Field::Tree => {}
            _ => result.push((coords.0 + 1, coords.1)),
        }
    }
    if coords.0 > 0 {
        match map[coords.0 - 1][coords.1] {
            Field::Tree => {}
            _ => result.push((coords.0 - 1, coords.1)),
        }
    }
    if coords.1 < map.len() - 1 {
        match map[coords.0][coords.1 + 1] {
            Field::Tree => {}
            _ => result.push((coords.0, coords.1 + 1)),
        }
    }
    if coords.1 > 0 {
        match map[coords.0][coords.1 - 1] {
            Field::Tree => {}
            _ => result.push((coords.0, coords.1 - 1)),
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map = parse_input(input);
    let paths = find_paths(&map);
    println!("{:?}", paths.iter().max().unwrap());

    let junctions = find_junctions(&map);
    let distances = find_distances(&junctions, &map);
    println!("{:?}", find_scenic_path(&distances, map.len()));
}
