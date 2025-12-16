use std::fs;

struct Region {
    shape: (usize, usize),
    required_shapes: Vec<usize>,
}

fn parse_input() -> (Vec<usize>, Vec<Region>) {
    let file = fs::read_to_string("input.txt").unwrap();

    let shape_areas = (0..=5)
        .map(|i| {
            let rows_start = 1 + (i * 5);

            let mut area = 0;
            (rows_start..(rows_start + 3)).for_each(|i| {
                file.lines().skip(i).next().unwrap().chars().for_each(|c| {
                    if c == '#' {
                        area += 1;
                    }
                });
            });
            area
        })
        .collect::<Vec<usize>>();

    let regions = file
        .lines()
        .skip(30)
        .map(|l| {
            let (dim_raw, required_raw) = l.split_once(": ").unwrap();
            let (dim_x, dim_y) = dim_raw.split_once("x").unwrap();
            let required_shapes = required_raw
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Region {
                shape: (
                    dim_x.parse::<usize>().unwrap(),
                    dim_y.parse::<usize>().unwrap(),
                ),
                required_shapes,
            }
        })
        .collect();

    (shape_areas, regions)
}

fn can_fit(shape_areas: &Vec<usize>, region: &Region) -> bool {
    let board_area = region.shape.0 * region.shape.1;
    let shapes_area = region
        .required_shapes
        .iter()
        .enumerate()
        .map(|(i, req)| req * shape_areas[i])
        .sum::<usize>();
    shapes_area <= board_area
}

fn main() {
    let (shapes, regions) = parse_input();

    let result = regions
        .iter()
        .map(|region| can_fit(&shapes, region))
        .filter(|x| *x)
        .count();

    println!("{}", result)
}
