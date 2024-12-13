use std::cmp;
use std::collections::HashSet;
use std::fs;

type Coords = (usize, usize);

struct Map {
    fields: Vec<Vec<char>>,
}

impl Map {
    fn size(&self) -> Coords {
        (self.fields[0].len(), self.fields.len())
    }

    fn neighbours(&self, field: Coords) -> Vec<Coords> {
        let mut neighbours: Vec<Coords> = vec![];

        if field.0 != 0 {
            neighbours.push((field.0 - 1, field.1));
        }

        if field.0 != self.size().0 - 1 {
            neighbours.push((field.0 + 1, field.1));
        }

        if field.1 != 0 {
            neighbours.push((field.0, field.1 - 1));
        }

        if field.1 != self.size().1 - 1 {
            neighbours.push((field.0, field.1 + 1));
        }

        neighbours
    }

    fn perimiter(&self, field: Coords) -> usize {
        let neighbours = self.neighbours(field);
        let mut res = 4 - neighbours.len();

        neighbours.iter().for_each(|n| {
            if self.fields[n.1][n.0] != self.fields[field.1][field.0] {
                res += 1;
            }
        });

        res
    }
}

fn parse_input(input: String) -> Map {
    Map {
        fields: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

fn fence_cost(map: &Map) -> usize {
    let mut calculated: HashSet<Coords> = HashSet::new();
    let mut result = 0;

    for y in 0..map.size().1 {
        for x in 0..map.size().0 {
            if calculated.contains(&(x, y)) {
                continue;
            }
            let area = explore_area((x, y), map, &mut calculated);
            result += area.len() * area.iter().map(|c| map.perimiter(*c)).sum::<usize>();
        }
    }

    result
}

fn fence_discounted_cost(map: &Map) -> usize {
    let mut calculated: HashSet<Coords> = HashSet::new();
    let mut result = 0;

    for y in 0..map.size().1 {
        for x in 0..map.size().0 {
            if calculated.contains(&(x, y)) {
                continue;
            }
            let area = explore_area((x, y), map, &mut calculated);
            result += area.len() * count_edges(area, &map);
        }
    }

    result
}

fn explore_area(
    starting_point: Coords,
    map: &Map,
    used_fields: &mut HashSet<Coords>,
) -> HashSet<Coords> {
    let current_symbol: char = map.fields[starting_point.1][starting_point.0];
    let mut queue = vec![starting_point];
    let mut area: HashSet<Coords> = HashSet::new();

    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        if map.fields[current.1][current.0] != current_symbol || used_fields.contains(&current) {
            continue;
        }
        area.insert(current);
        used_fields.insert(current);

        map.neighbours(current).iter().for_each(|f| queue.push(*f));
    }

    area
}

fn count_edges(area: HashSet<Coords>, map: &Map) -> usize {
    let size = map.size();
    let mut res = 0;
    let x_bounds: (usize, usize) = area.iter().fold((size.0, 0), |acc, e| {
        (cmp::min(acc.0, e.0), cmp::max(acc.1, e.0))
    });
    let y_bounds: (usize, usize) = area.iter().fold((size.1, 0), |acc, e| {
        (cmp::min(acc.0, e.1), cmp::max(acc.1, e.1))
    });

    // Left and right edges
    for x in x_bounds.0..=x_bounds.1 {
        let mut broke_edge_l = true;
        let mut broke_edge_r = true;
        for y in y_bounds.0..=y_bounds.1 {
            if !area.contains(&(x, y)) {
                broke_edge_l = true;
                broke_edge_r = true;
                continue;
            }

            if broke_edge_l {
                if x == x_bounds.0 || !area.contains(&(x - 1, y)) {
                    broke_edge_l = false;
                    res += 1;
                }
            } else {
                if x != x_bounds.0 && area.contains(&(x - 1, y)) {
                    broke_edge_l = true;
                }
            }

            if broke_edge_r {
                if !area.contains(&(x + 1, y)) {
                    broke_edge_r = false;
                    res += 1;
                }
            } else {
                if x != x_bounds.1 && area.contains(&(x + 1, y)) {
                    broke_edge_r = true;
                }
            }
        }
    }

    // Top and bottom edges
    for y in y_bounds.0..=y_bounds.1 {
        let mut broke_edge_t = true;
        let mut broke_edge_b = true;
        for x in x_bounds.0..=x_bounds.1 {
            if !area.contains(&(x, y)) {
                broke_edge_t = true;
                broke_edge_b = true;
                continue;
            }

            if broke_edge_t {
                if y == y_bounds.0 || !area.contains(&(x, y - 1)) {
                    broke_edge_t = false;
                    res += 1;
                }
            } else {
                if y != y_bounds.0 && area.contains(&(x, y - 1)) {
                    broke_edge_t = true;
                }
            }

            if broke_edge_b {
                if y == y_bounds.1 || !area.contains(&(x, y + 1)) {
                    broke_edge_b = false;
                    res += 1;
                }
            } else {
                if y != y_bounds.1 && area.contains(&(x, y + 1)) {
                    broke_edge_b = true;
                }
            }
        }
    }

    res
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let map = parse_input(input);

    let result_1 = fence_cost(&map);
    println!("{}", result_1);

    let result_2 = fence_discounted_cost(&map);
    println!("{}", result_2);
}
