use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone)]
enum Element {
    Start,
    None,
    PipeVertical,
    PipeHorizontal,
    ConnectorNorthEast,
    ConnectorNorthWest,
    ConnectorSouthEast,
    ConnectorSouthWest,
}

#[derive(Clone)]
enum SimpleElement {
    Wall,
    None,
}

type Map = Vec<Vec<Element>>;
type SimpleMap = Vec<Vec<SimpleElement>>;
type Coords = (usize, usize);

fn parse_input(input: String) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Element::PipeVertical,
                    '-' => Element::PipeHorizontal,
                    'S' => Element::Start,
                    'L' => Element::ConnectorNorthEast,
                    'J' => Element::ConnectorNorthWest,
                    '7' => Element::ConnectorSouthWest,
                    'F' => Element::ConnectorSouthEast,
                    '.' => Element::None,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn loop_path(map: &Map) -> Vec<Vec<bool>> {
    let mut max_path: usize = 0;
    let mut visited = vec![vec![false; map.len()]; map.len()];

    let starting_point: (usize, usize) = find_starting_point(map);
    let queue_starters: Vec<(Coords, usize)> = neighbours(starting_point, map)
        .iter()
        .map(|coords| (*coords, 1))
        .collect();

    let mut queue = VecDeque::from(queue_starters);

    while !queue.is_empty() {
        let (current, new_length) = queue.pop_front().unwrap();

        if visited[current.0][current.1] {
            continue;
        }

        if new_length > max_path {
            max_path = new_length;
        }

        match map[current.0][current.1] {
            Element::PipeVertical => {
                if is_in_bounds(current, (1, 0), map.len()) {
                    queue.push_back(((current.0 + 1, current.1), new_length + 1));
                }
                if is_in_bounds(current, (-1, 0), map.len()) {
                    queue.push_back(((current.0 - 1, current.1), new_length + 1));
                }
            }
            Element::PipeHorizontal => {
                if is_in_bounds(current, (0, 1), map.len()) {
                    queue.push_back(((current.0, current.1 + 1), new_length + 1));
                }
                if is_in_bounds(current, (0, -1), map.len()) {
                    queue.push_back(((current.0, current.1 - 1), new_length + 1));
                }
            }
            Element::ConnectorNorthEast => {
                if is_in_bounds(current, (-1, 0), map.len()) {
                    queue.push_back(((current.0 - 1, current.1), new_length + 1));
                }
                if is_in_bounds(current, (0, 1), map.len()) {
                    queue.push_back(((current.0, current.1 + 1), new_length + 1));
                }
            }
            Element::ConnectorNorthWest => {
                if is_in_bounds(current, (0, -1), map.len()) {
                    queue.push_back(((current.0, current.1 - 1), new_length + 1));
                }
                if is_in_bounds(current, (-1, 0), map.len()) {
                    queue.push_back(((current.0 - 1, current.1), new_length + 1));
                }
            }
            Element::ConnectorSouthWest => {
                if is_in_bounds(current, (0, -1), map.len()) {
                    queue.push_back(((current.0, current.1 - 1), new_length + 1));
                }
                if is_in_bounds(current, (1, 0), map.len()) {
                    queue.push_back(((current.0 + 1, current.1), new_length + 1));
                }
            }
            Element::ConnectorSouthEast => {
                if is_in_bounds(current, (0, 1), map.len()) {
                    queue.push_back(((current.0, current.1 + 1), new_length + 1));
                }
                if is_in_bounds(current, (1, 0), map.len()) {
                    queue.push_back(((current.0 + 1, current.1), new_length + 1));
                }
            }
            _ => (),
        }

        visited[current.0][current.1] = true;
    }

    visited
}

fn loop_path_furthest_point(path: &Vec<Vec<bool>>) -> usize {
    path.iter()
        .map(|row| row.iter().filter(|b| **b).count())
        .sum::<usize>()
        / 2
}

fn modifiers_for_elements(element: &Element) -> Vec<(i32, i32)> {
    match element {
        Element::PipeVertical => vec![(1, 0), (-1, 0)],
        Element::PipeHorizontal => vec![(0, 1), (0, -1)],
        Element::ConnectorNorthEast => vec![(-1, 0), (0, 1)],
        Element::ConnectorNorthWest => vec![(-1, 0), (0, -1)],
        Element::ConnectorSouthWest => vec![(1, 0), (0, -1)],
        Element::ConnectorSouthEast => vec![(1, 0), (0, 1)],
        Element::Start => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
        _ => vec![],
    }
}

fn replace_starting_point(map: &Map, loop_path: &Vec<Vec<bool>>) -> Map {
    let starting_point: (usize, usize) = find_starting_point(map);
    let replacement = starting_point_element(map, starting_point);

    map.iter()
        .enumerate()
        .map(|(row, r)| {
            r.iter()
                .enumerate()
                .map(|(column, element)| {
                    if let Element::Start = element {
                        replacement.clone()
                    } else {
                        if loop_path[row][column] {
                            element.clone()
                        } else {
                            Element::None
                        }
                    }
                })
                .collect()
        })
        .collect()
}

fn find_starting_point(map: &Map) -> Coords {
    for (row, r) in map.iter().enumerate() {
        for (column, element) in r.iter().enumerate() {
            if let Element::Start = element {
                return (row, column);
            }
        }
    }

    unreachable!()
}

fn starting_point_element(map: &Map, coords: Coords) -> Element {
    let neighbours: Vec<(i32, i32)> = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .filter(|modifier| is_in_bounds(coords, **modifier, map.len()))
        .filter(|modifier| {
            can_connect_to(
                // Previous Filter ensures that this is in bounds
                (
                    (coords.0 as i32 + modifier.0) as usize,
                    (coords.1 as i32 + modifier.1) as usize,
                ),
                (-modifier.0, -modifier.1),
                map,
            )
        })
        .map(|m| m.clone())
        .collect();

    match (
        (neighbours[0].0, neighbours[0].1),
        (neighbours[1].0, neighbours[1].1),
    ) {
        ((-1, 0), (0, 1)) => Element::ConnectorNorthEast,
        ((-1, 0), (0, -1)) => Element::ConnectorNorthWest,
        ((1, 0), (0, -1)) => Element::ConnectorSouthWest,
        ((1, 0), (0, 1)) => Element::ConnectorSouthEast,
        ((1, 0), (-1, 0)) => Element::PipeVertical,
        ((0, 1), (0, -1)) => Element::PipeHorizontal,
        _ => unreachable!(),
    }
}

fn neighbours_modifiers(coords: Coords, map: &Map) -> Vec<(i32, i32)> {
    modifiers_for_elements(&map[coords.0][coords.1])
        .iter()
        .filter(|modifier| is_in_bounds(coords, **modifier, map.len()))
        .filter(|modifier| {
            can_connect_to(
                // Previous Filter ensures that this is in bounds
                (
                    (coords.0 as i32 + modifier.0) as usize,
                    (coords.1 as i32 + modifier.1) as usize,
                ),
                (-modifier.0, -modifier.1),
                map,
            )
        })
        .map(|m| m.clone())
        .collect()
}

fn neighbours(coords: Coords, map: &Map) -> Vec<Coords> {
    neighbours_modifiers(coords, map)
        .iter()
        .map(|modifier| {
            (
                (coords.0 as i32 + modifier.0) as usize,
                (coords.1 as i32 + modifier.1) as usize,
            )
        })
        .collect()
}

// Make map 3x the size to expose path between pipes
fn expand_map(map: &Map) -> SimpleMap {
    let mut expanded_map =
        vec![vec![SimpleElement::None; (map.len() + 1) * 3]; (map.len() + 1) * 3];

    map.iter().enumerate().for_each(|(row, r)| {
        r.iter()
            .enumerate()
            .map(|(column, element)| {
                let n_row = row * 3;
                let n_column = column * 3;

                match element {
                    Element::PipeVertical => {
                        expanded_map[n_row][n_column + 1] = SimpleElement::Wall;
                        expanded_map[n_row + 1][n_column + 1] = SimpleElement::Wall;
                        expanded_map[n_row + 2][n_column + 1] = SimpleElement::Wall;
                    }
                    Element::PipeHorizontal => {
                        expanded_map[n_row + 1][n_column] = SimpleElement::Wall;
                        expanded_map[n_row + 1][n_column + 1] = SimpleElement::Wall;
                        expanded_map[n_row + 1][n_column + 2] = SimpleElement::Wall;
                    }
                    Element::ConnectorNorthEast => {
                        expanded_map[n_row][n_column + 1] = SimpleElement::Wall;
                        expanded_map[n_row + 1][n_column + 1] = SimpleElement::Wall;
                        expanded_map[n_row + 1][n_column + 2] = SimpleElement::Wall;
                    }
                    Element::ConnectorNorthWest => {
                        expanded_map[n_row][n_column + 1] = SimpleElement::Wall;
                        expanded_map[n_row + 1][n_column] = SimpleElement::Wall;
                        expanded_map[n_row + 1][n_column + 1] = SimpleElement::Wall;
                    }
                    Element::ConnectorSouthWest => {
                        expanded_map[n_row + 1][n_column] = SimpleElement::Wall;
                        expanded_map[n_row + 1][n_column + 1] = SimpleElement::Wall;
                        expanded_map[n_row + 2][n_column + 1] = SimpleElement::Wall;
                    }
                    Element::ConnectorSouthEast => {
                        expanded_map[n_row + 1][n_column + 1] = SimpleElement::Wall;
                        expanded_map[n_row + 1][n_column + 2] = SimpleElement::Wall;
                        expanded_map[n_row + 2][n_column + 1] = SimpleElement::Wall;
                    }
                    _ => (),
                }
            })
            .collect()
    });
    expanded_map
}

fn fill_map(map: &SimpleMap) -> HashSet<Coords> {
    let mut result: HashSet<Coords> = HashSet::new();

    let mut queue = VecDeque::from(vec![(0, 0)]);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if result.contains(&current) {
            continue;
        }

        match map[current.0][current.1] {
            SimpleElement::Wall => continue,
            SimpleElement::None => {
                result.insert(current);
            }
        }

        for modifier in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if is_in_bounds(current, modifier, map.len()) {
                queue.push_back((
                    (current.0 as i32 + modifier.0) as usize,
                    (current.1 as i32 + modifier.1) as usize,
                ));
            }
        }
    }

    result
}

fn empty_fields(map: &Map, expanded_map: &SimpleMap, filled_map: &HashSet<Coords>) -> Vec<Coords> {
    map.iter()
        .enumerate()
        .map(|(row, r)| {
            r.iter()
                .enumerate()
                .filter(|(column, _)| {
                    let n_row = row * 3;
                    let n_column = column * 3;

                    for i in n_row..n_row + 3 {
                        for j in n_column..n_column + 3 {
                            if let SimpleElement::Wall = expanded_map[i][j] {
                                return false;
                            }

                            if filled_map.contains(&(i, j)) {
                                return false;
                            }
                        }
                    }

                    true
                })
                .map(|(column, _)| (row, column))
                .collect::<Vec<Coords>>()
        })
        .flatten()
        .collect()
}

fn can_connect_to(point: Coords, modifier: (i32, i32), map: &Map) -> bool {
    modifiers_for_elements(&map[point.0][point.1]).contains(&modifier)
}

fn is_in_bounds(coords: Coords, modifier: (i32, i32), map_length: usize) -> bool {
    let row_coords = coords.0 as i32 + modifier.0;
    let col_coords = coords.1 as i32 + modifier.1;

    if row_coords < 0 || col_coords < 0 {
        return false;
    }

    if col_coords >= map_length as i32 || row_coords >= map_length as i32 {
        return false;
    }

    return true;
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");

    let map = parse_input(input);

    let loop_path = loop_path(&map);
    let loop_furthest_point = loop_path_furthest_point(&loop_path);

    println!("{}", loop_furthest_point);

    let map_without_start = replace_starting_point(&map, &loop_path);
    let expanded_map = expand_map(&map_without_start);
    let filled_empty_spaces = fill_map(&expanded_map);
    let empty_fields_count = empty_fields(&map, &expanded_map, &filled_empty_spaces)
        .iter()
        .count();

    println!("{}", empty_fields_count);
}
