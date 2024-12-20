use std::collections::HashMap;
use std::fs;

type Coords = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Wall,
    Empty,
}

struct Map {
    fields: Vec<Vec<Field>>,
    start: Coords,
    end: Coords,
}

impl Map {
    fn size(&self) -> Coords {
        (self.fields.len(), self.fields[0].len())
    }

    fn belonging_point(&self, coords: Coords, (dx, dy): (isize, isize)) -> Option<Coords> {
        let (x, y): (isize, isize) = (coords.0 as isize, coords.1 as isize);
        let candidate = (x + dx, y + dy);
        let map_size = self.size();

        if candidate.0 < 0
            || candidate.1 < 0
            || candidate.0 >= map_size.0 as isize
            || candidate.1 >= map_size.1 as isize
        {
            return None;
        }

        Some((candidate.0 as usize, candidate.1 as usize))
    }

    fn neighbours(&self, coords: Coords) -> Vec<Coords> {
        let (x, y) = coords;
        let mut neighbours = Vec::new();

        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if x < self.fields.len() - 1 {
            neighbours.push((x + 1, y));
        }
        if y < self.fields[0].len() - 1 {
            neighbours.push((x, y + 1));
        }

        neighbours
            .iter()
            .filter(|c| self.fields[c.1][c.0] != Field::Wall)
            .cloned()
            .collect()
    }
}

fn parse_input(input: String) -> Map {
    let mut start: Coords = (0, 0);
    let mut end: Coords = (0, 0);

    let fields: Vec<Vec<Field>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Field::Wall,
                    '.' => Field::Empty,
                    'S' => {
                        start = (x, y);
                        Field::Empty
                    }
                    'E' => {
                        end = (x, y);
                        Field::Empty
                    }
                    _ => unreachable!("Unrecognized character in the input"),
                })
                .collect()
        })
        .collect();

    Map { fields, start, end }
}

fn construct_legal_path(map: &Map) -> Vec<Coords> {
    let mut stack: Vec<(Coords, Vec<Coords>)> = vec![(map.start, vec![])];

    while !stack.is_empty() {
        let (current, path) = stack.pop().unwrap();

        let new_path: Vec<Coords> = vec![path, vec![current]].concat();

        if current == map.end {
            return new_path.clone();
        }

        for n in map.neighbours(current) {
            if new_path.contains(&n) {
                continue;
            }

            stack.push((n, new_path.clone()));
        }
    }

    unreachable!("Didn't find end in the maze")
}

fn possible_cheats(map: &Map, path: &Vec<Coords>, possible_step: usize) -> HashMap<usize, usize> {
    let mut cheats: HashMap<usize, usize> = HashMap::new();
    let max_step: isize = possible_step as isize;

    for (step, field) in path.iter().enumerate() {
        for x in (-max_step)..=(max_step) {
            for y in (-max_step + x.abs())..=(max_step - x.abs()) {
                let Some(candidate) = map.belonging_point(*field, (x, y)) else {
                    continue;
                };

                if map.fields[candidate.1][candidate.0] == Field::Wall {
                    continue;
                }

                let distance = x.abs() as usize + y.abs() as usize;
                let pos = path.iter().position(|cc| *cc == candidate).unwrap();
                if pos <= step + distance {
                    continue;
                }

                let diff = pos - step - distance;
                if diff >= 100 {
                    *cheats.entry(diff).or_insert(0) += 1;
                }
            }
        }
    }

    cheats
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let map = parse_input(input);
    let path = construct_legal_path(&map);

    let cheats_1 = possible_cheats(&map, &path, 2);
    let result_1 = cheats_1.iter().map(|(_, p)| p).sum::<usize>();

    println!("{}", result_1);

    let cheats_2 = possible_cheats(&map, &path, 20);
    let result_2 = cheats_2.iter().map(|(_, p)| p).sum::<usize>();

    println!("{}", result_2);
}
