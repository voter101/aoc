use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

type Coords = (usize, usize);

struct Map {
    corrupted: Vec<Coords>,
    size: Coords,
}

impl Map {
    fn neighbours(&self, coords: Coords, corruptions_limit: usize) -> Vec<Coords> {
        let mut candidates = Vec::new();

        if coords.0 > 0 {
            candidates.push((coords.0 - 1, coords.1));
        }

        if coords.1 > 0 {
            candidates.push((coords.0, coords.1 - 1));
        }

        if coords.0 != self.size.0 - 1 {
            candidates.push((coords.0 + 1, coords.1));
        }

        if coords.1 != self.size.1 - 1 {
            candidates.push((coords.0, coords.1 + 1));
        }

        candidates
            .iter()
            .filter(|c| !self.corrupted[0..=corruptions_limit].contains(c))
            .cloned()
            .collect()
    }
}

fn parse_input(input: String, size: Coords) -> Map {
    let corrupted = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    Map { corrupted, size }
}

fn shortest_path(map: &Map, corruptions_limit: usize) -> Option<usize> {
    let mut queue: VecDeque<(Coords, usize)> = VecDeque::new();
    let mut visited: HashMap<Coords, usize> = HashMap::new();

    queue.push_back(((0, 0), 0));

    while let Some((coords, steps)) = queue.pop_front() {
        let existing_cost = visited.get(&coords).unwrap_or(&usize::MAX);

        if steps >= *existing_cost {
            continue;
        }
        visited.insert(coords, steps);

        for neighbour in map.neighbours(coords, corruptions_limit) {
            queue.push_back((neighbour, steps + 1));
        }
    }

    visited.get(&(map.size.0 - 1, map.size.1 - 1)).cloned()
}

fn main() {
    let map_size = 71;
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let map = parse_input(input, (map_size, map_size));

    println!("{}", shortest_path(&map, 1024).unwrap());

    let mut low = 1024;
    let mut high = map.corrupted.len() - 1;
    let mut none = usize::MAX;

    while low <= high {
        let mid = (low + high) / 2;
        if let None = shortest_path(&map, mid) {
            none = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    println!("{},{}", map.corrupted[none].0, map.corrupted[none].1);
}
