use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

type Point = (usize, usize, usize);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord)]
struct DistanceBetween {
    point_1: Point,
    point_2: Point,
    distance: usize,
}

impl PartialOrd for DistanceBetween {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

fn parse_input() -> Vec<Point> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let splits: Vec<usize> = line
                .split(",")
                .map(|e| e.parse::<usize>().unwrap())
                .collect();
            (splits[0], splits[1], splits[2])
        })
        .collect()
}

fn distance_between(p_1: Point, p_2: Point) -> usize {
    let (x_1, y_1, z_1) = (p_1.0 as isize, p_1.1 as isize, p_1.2 as isize);
    let (x_2, y_2, z_2) = (p_2.0 as isize, p_2.1 as isize, p_2.2 as isize);

    ((x_1 - x_2).pow(2) + (y_1 - y_2).pow(2) + (z_1 - z_2).pow(2)).isqrt() as usize
}

fn distances_heap(points: &Vec<Point>) -> BinaryHeap<DistanceBetween> {
    let mut res: BinaryHeap<DistanceBetween> = BinaryHeap::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (point_1, point_2) = (points[i], points[j]);
            res.push(DistanceBetween {
                point_1,
                point_2,
                distance: distance_between(point_1, point_2),
            });
        }
    }

    res
}

fn build_circuits(
    distances: &BinaryHeap<DistanceBetween>,
    pairs_to_pick: usize,
) -> Vec<HashSet<Point>> {
    let mut dist = distances.clone();
    let mut circuits: Vec<HashSet<Point>> = vec![];

    for _ in 0..pairs_to_pick {
        let pair = dist.pop().unwrap();

        let mut existing_indices: Vec<usize> = vec![];

        for i in 0..circuits.len() {
            if circuits[i].contains(&pair.point_1) || circuits[i].contains(&pair.point_2) {
                existing_indices.push(i);
            }
        }

        let mut new_circuit: HashSet<Point> = HashSet::new();
        new_circuit.insert(pair.point_1);
        new_circuit.insert(pair.point_2);

        for idx in existing_indices.into_iter().rev() {
            let removed = circuits.remove(idx);
            new_circuit.extend(removed);
        }
        circuits.push(new_circuit);
    }

    circuits
}

fn build_complete_circuit(distances: &BinaryHeap<DistanceBetween>, pairs: usize) -> (Point, Point) {
    let mut dist = distances.clone();
    let mut circuits: Vec<HashSet<Point>> = vec![];
    let mut curr: DistanceBetween = DistanceBetween {
        point_1: (0, 0, 0),
        point_2: (0, 0, 0),
        distance: 0,
    };

    loop {
        if circuits.len() == 1 && circuits[0].len() == pairs {
            break;
        }

        curr = dist.pop().unwrap();

        let mut existing_indices: Vec<usize> = vec![];

        for i in 0..circuits.len() {
            if circuits[i].contains(&curr.point_1) || circuits[i].contains(&curr.point_2) {
                existing_indices.push(i);
            }
        }

        let mut new_circuit: HashSet<Point> = HashSet::new();
        new_circuit.insert(curr.point_1);
        new_circuit.insert(curr.point_2);

        for idx in existing_indices.into_iter().rev() {
            let removed = circuits.remove(idx);
            new_circuit.extend(removed);
        }
        circuits.push(new_circuit);
    }

    (curr.point_1, curr.point_2)
}

fn main() {
    let points = parse_input();

    let distances = distances_heap(&points);
    let circuits = build_circuits(&distances, 1000);

    let mut circuit_lengths = circuits.iter().map(|c| c.len()).collect::<Vec<usize>>();
    circuit_lengths.sort();

    let part_1 = circuit_lengths.iter().rev().take(3).product::<usize>();

    println!("{}", part_1);

    let (closing_p_1, closing_p_2) = build_complete_circuit(&distances, points.len());
    let part_2 = closing_p_1.0 * closing_p_2.0;

    println!("{:?}", part_2);
}
