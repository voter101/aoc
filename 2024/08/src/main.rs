use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::{fs, usize};

type Coords = (usize, usize);

fn create_antennas_map(input: String) -> (HashMap<char, Vec<Coords>>, Coords) {
    let mut map_size = (0, 0);
    let mut result: HashMap<char, Vec<Coords>> = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        map_size.1 = max(map_size.1, y);
        line.chars().enumerate().for_each(|(x, c)| {
            map_size.0 = max(map_size.0, x);

            match c {
                '.' => {}
                _ => {
                    let col = result.entry(c).or_insert(vec![]);
                    col.push((x, y));
                }
            }
        });
    });

    (result, map_size)
}

fn antinodes(
    antennas: &HashMap<char, Vec<Coords>>,
    map_size: Coords,
    use_resonant_harmonics: bool,
) -> HashSet<Coords> {
    let mut result = HashSet::new();

    for same_frequency_antennas in antennas.values() {
        for candidate in
            antinodes_candidates(same_frequency_antennas, map_size, use_resonant_harmonics)
        {
            result.insert(candidate);
        }
    }

    result
}

fn antinodes_candidates(
    antennas: &Vec<Coords>,
    map_size: Coords,
    use_resonant_harmonics: bool,
) -> Vec<Coords> {
    let mut result = vec![];

    for i in 0..antennas.len() {
        for ii in (i + 1)..antennas.len() {
            let (x_1, y_1) = antennas[i];
            let (x_2, y_2) = antennas[ii];

            let delta_x = (x_1.clone() as isize - x_2.clone() as isize).abs();
            let delta_y = (y_1.clone() as isize - y_2.clone() as isize).abs();

            let (delta_x_1, delta_x_2) = if x_1 < x_2 {
                (-delta_x, delta_x)
            } else {
                (delta_x, -delta_x)
            };
            let (delta_y_1, delta_y_2) = if y_1 < y_2 {
                (-delta_y, delta_y)
            } else {
                (delta_y, -delta_y)
            };

            let range = if use_resonant_harmonics {
                0..(usize::MAX)
            } else {
                1..2
            };

            for delta_modifier in range.clone() {
                let p_x = x_1 as isize + (delta_x_1 * delta_modifier as isize);
                let p_y = y_1 as isize + (delta_y_1 * delta_modifier as isize);

                if out_of_bounds((p_x, p_y), map_size) {
                    break;
                }

                result.push((p_x as usize, p_y as usize));
            }

            for delta_modifier in range {
                let p_x = x_2 as isize + (delta_x_2 * delta_modifier as isize);
                let p_y = y_2 as isize + (delta_y_2 * delta_modifier as isize);

                if out_of_bounds((p_x, p_y), map_size) {
                    break;
                }

                result.push((p_x as usize, p_y as usize));
            }
        }
    }

    result
}

fn out_of_bounds(p: (isize, isize), map_size: Coords) -> bool {
    p.0 < 0 || p.1 < 0 || p.0 > map_size.0 as isize || p.1 > map_size.1 as isize
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let (antennas, map_size) = create_antennas_map(input);

    let result_1 = antinodes(&antennas, map_size, false).len();
    println!("{:?}", result_1);

    let result_2 = antinodes(&antennas, map_size, true).len();
    println!("{:?}", result_2);
}
