use std::fs;

type Coords = (u64, u64);

fn parse_input(input: String) -> Vec<Coords> {
    let mut result: Vec<Coords> = Vec::new();

    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(column, c)| {
            if c == '#' {
                result.push((row as u64, column as u64));
            }
        });
    });

    result
}

fn empty_rows(galaxies: &Vec<Coords>, map_length: u64) -> Vec<u64> {
    (0..map_length)
        .into_iter()
        .filter(|row| !galaxies.iter().any(|(r, _)| r == row))
        .map(|row| row)
        .collect()
}

fn empty_columns(galaxies: &Vec<Coords>, map_length: u64) -> Vec<u64> {
    (0..map_length)
        .into_iter()
        .filter(|column| !galaxies.iter().any(|(_, c)| c == column))
        .map(|column| column)
        .collect()
}

fn scale_galaxies(
    galaxies: &Vec<Coords>,
    empty_rows: &Vec<u64>,
    empty_columns: &Vec<u64>,
    scale_factor: u64,
) -> Vec<Coords> {
    galaxies
        .iter()
        .map(|(row, column)| {
            let expandable_rows = (0..*row)
                .into_iter()
                .filter(|r| empty_rows.contains(r))
                .count() as u64;
            let row_extension = row - expandable_rows + expandable_rows * scale_factor;

            let exandable_columns = (0..*column)
                .into_iter()
                .filter(|c| empty_columns.contains(c))
                .count() as u64;
            let column_extension = column - exandable_columns + exandable_columns * scale_factor;

            (row_extension, column_extension)
        })
        .collect()
}

fn generate_pairs(elements: &Vec<Coords>) -> Vec<(&Coords, &Coords)> {
    elements
        .iter()
        .enumerate()
        .flat_map(move |e| std::iter::repeat(e.1).zip(elements.iter().skip(e.0 + 1)))
        .collect()
}

fn distance_between_galaxies((x1, y1): Coords, (x2, y2): Coords) -> u64 {
    ((x1 as i32 - x2 as i32).abs() as u64) + ((y1 as i32 - y2 as i32).abs() as u64)
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("File not loaded");
    let length = input.len();

    let galaxies = parse_input(input);
    let empty_rows = empty_rows(&galaxies, length as u64);
    let empty_columns = empty_columns(&galaxies, length as u64);

    let scaled_galaxies: Vec<Coords> = scale_galaxies(&galaxies, &empty_rows, &empty_columns, 2);
    let scaled_old_galaxies: Vec<Coords> =
        scale_galaxies(&galaxies, &empty_rows, &empty_columns, 1000000);
    let pairs = generate_pairs(&scaled_galaxies);
    let pairs_old = generate_pairs(&scaled_old_galaxies);

    let distances: u64 = pairs
        .iter()
        .map(|(g1, g2)| distance_between_galaxies(**g1, **g2))
        .sum();

    let distances_old: u64 = pairs_old
        .iter()
        .map(|(g1, g2)| distance_between_galaxies(**g1, **g2))
        .sum();

    println!("{:?}", distances);
    println!("{:?}", distances_old);
}
