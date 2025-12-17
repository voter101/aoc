use std::collections::BinaryHeap;
use std::fs::read_to_string;
type Point = (isize, isize);

#[derive(Eq, PartialEq, Ord, Copy, Clone)]
struct Rectangle {
    points: (Point, Point),
    area: usize,
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.area.partial_cmp(&other.area)
    }
}

fn to_segments(points: &Vec<Point>) -> (Vec<(Point, Point)>, Vec<(Point, Point)>) {
    let mut h_edges: Vec<(Point, Point)> = vec![];
    let mut v_edges: Vec<(Point, Point)> = vec![];

    for i in 0..(points.len() - 1) {
        let (x_1, y_1) = points[i];
        let (x_2, y_2) = points[i + 1];
        if y_1 == y_2 {
            h_edges.push(((x_1.min(x_2), y_1), (x_1.max(x_2), y_1)));
        } else {
            v_edges.push(((x_1, y_1.min(y_2)), (x_1, y_1.max(y_2))));
        }
    }

    let (x_1, y_1) = points[points.len() - 1];
    let (x_2, y_2) = points[0];
    if y_1 == y_2 {
        h_edges.push(((x_1.min(x_2), y_1), (x_1.max(x_2), y_1)));
    } else {
        v_edges.push(((x_1, y_1.min(y_2)), (x_1, y_1.max(y_2))));
    }

    (h_edges, v_edges)
}

fn parse_input() -> Vec<Point> {
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
        })
        .collect()
}

fn construct_areas(points: &Vec<Point>) -> BinaryHeap<Rectangle> {
    let mut areas: BinaryHeap<Rectangle> = BinaryHeap::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = (((points[i].0 - points[j].0).abs() + 1)
                * ((points[i].1 - points[j].1).abs() + 1)) as usize;
            let a_p_1 = points[i];
            let a_p_2 = points[j];

            let p_1 = (a_p_1.0.min(a_p_2.0), a_p_1.1.min(a_p_2.1));
            let p_2 = (a_p_1.0.max(a_p_2.0), a_p_1.1.max(a_p_2.1));
            areas.push(Rectangle {
                points: (p_1, p_2),
                area,
            })
        }
    }

    areas
}

fn rectangle_inside_boundaries(
    (x_1, y_1): &Point,
    (x_2, y_2): &Point,
    h_edges: &Vec<(Point, Point)>,
    v_edges: &Vec<(Point, Point)>,
) -> bool {
    for ((e_x_1, e_y), (e_x_2, _)) in h_edges {
        if e_y > y_1 && e_y < y_2 && e_x_1.max(x_1) < e_x_2.min(x_2) {
            return false;
        }
    }

    for ((e_x, e_y_1), (_, e_y_2)) in v_edges {
        if e_x > x_1 && e_x < x_2 && e_y_1.max(y_1) < e_y_2.min(y_2) {
            return false;
        }
    }

    true
}

fn main() {
    let points = parse_input();
    let mut areas = construct_areas(&points);

    println!("{}", areas.peek().unwrap().area);

    let (h_boundaries, v_boundaries) = to_segments(&points);

    let mut area = areas.pop().unwrap();
    while !rectangle_inside_boundaries(&area.points.0, &area.points.1, &h_boundaries, &v_boundaries)
    {
        area = areas.pop().unwrap();
    }
    println!("{}", area.area);
}
