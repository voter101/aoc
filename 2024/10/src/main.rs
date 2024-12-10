use std::collections::HashSet;
use std::fs;

type Coords = (usize, usize);

struct Map {
    fields: Vec<Vec<usize>>,
}

impl Map {
    fn size(&self) -> Coords {
        (self.fields[0].len(), self.fields.len())
    }

    fn starting_points(&self) -> Vec<Coords> {
        let mut res = vec![];
        for (y, row) in self.fields.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                if val == 0 {
                    res.push((x, y))
                }
            }
        }

        res
    }

    fn neighbours(&self, coords: Coords) -> Vec<Coords> {
        let mut res = vec![];
        let (x, y) = coords;
        let (width, height) = self.size();
        let val = self.fields[y][x];

        if x > 0 {
            res.push((x - 1, y));
        }

        if x < width - 1 {
            res.push((x + 1, y));
        }

        if y > 0 {
            res.push((x, y - 1));
        }

        if y < height - 1 {
            res.push((x, y + 1));
        }

        res.iter()
            .filter(|e| self.fields[e.1][e.0] == (val + 1))
            .cloned()
            .collect::<_>()
    }
}

fn parse_map(input: String) -> Map {
    let fields = input
        .lines()
        .map(|l| l.chars().map(|c| c as usize - '0' as usize).collect())
        .collect();

    Map { fields }
}

fn trailheads_score(map: &Map) -> usize {
    map.starting_points()
        .iter()
        .map(|&p| trailhead_score(p, map).len())
        .sum::<usize>()
}

fn trailhead_score(point: Coords, map: &Map) -> HashSet<Coords> {
    if map.fields[point.1][point.0] == 9 {
        return vec![point].into_iter().collect();
    }

    map.neighbours(point)
        .iter()
        .map(|&p| trailhead_score(p, map))
        .fold(HashSet::new(), |acc, e| acc.union(&e).cloned().collect())
}

fn trailheads_rating(map: &Map) -> usize {
    map.starting_points()
        .iter()
        .map(|&p| trailhead_rating(p, map))
        .sum::<usize>()
}

fn trailhead_rating(point: Coords, map: &Map) -> usize {
    if map.fields[point.1][point.0] == 9 {
        return 1;
    }

    map.neighbours(point)
        .iter()
        .map(|&p| trailhead_rating(p, map))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let map = parse_map(input);

    let result_1 = trailheads_score(&map);
    println!("{}", result_1);

    let result_2 = trailheads_rating(&map);
    println!("{}", result_2);
}
