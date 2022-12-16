use std::cmp::{max, min};
use std::fs;
use std::ops::RangeInclusive;
use text_io::scan;

#[derive(Clone, Debug)]
struct Coords(i32, i32);

fn parse_input(input: String) -> (Vec<(Coords, i32)>, Vec<Coords>) {
    let parsed_input = input
        .lines()
        .map(|line| {
            let mut coords_sensor: Coords = Coords(0, 0);
            let mut coords_beacons: Coords = Coords(0, 0);

            scan!(
                line.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                coords_sensor.0, coords_sensor.1, coords_beacons.0, coords_beacons.1
            );

            (coords_sensor, coords_beacons)
        })
        .collect::<Vec<_>>();

    let sensors_vec: Vec<(Coords, i32)> = parsed_input
        .clone()
        .iter()
        .map(|(sensor, beacon)| {
            let manhattan_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            (sensor.clone(), manhattan_distance)
        })
        .collect::<Vec<_>>();
    let beacons_vec: Vec<Coords> = parsed_input
        .iter()
        .map(|(_, beacon)| beacon.clone())
        .collect::<Vec<_>>();

    (sensors_vec, beacons_vec)
}

fn covered_ranges_for_y(y: i32, sensors: &Vec<(Coords, i32)>) -> Vec<RangeInclusive<i32>> {
    let mut ranges = sensors
        .iter()
        .filter_map(|(sensor, range)| {
            let distance_vertical = (sensor.1 - y).abs();

            if distance_vertical <= range.clone() {
                let remaining_distance = range - distance_vertical;
                Some((sensor.0 - remaining_distance)..=(sensor.0 + remaining_distance))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    minimized_ranges(&mut ranges)
}

fn minimized_ranges(ranges: &mut Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    ranges.sort_by_key(|r| r.start().clone());
    let mut result: Vec<RangeInclusive<i32>> = vec![];

    for range in ranges {
        let mut any_modified = false;
        for i in 0..result.len() {
            if are_ranges_overlapping(range.clone(), result[i].clone()) {
                let low = min(range.start(), result[i].start()).clone();
                let high = max(range.end(), result[i].end()).clone();
                result[i] = low..=high;
                any_modified = true;
            }
        }
        if !any_modified {
            result.push(range.clone());
        }
    }
    result
}

fn impossible_beacons(y_coord: i32, beacons: Vec<Coords>, ranges: Vec<RangeInclusive<i32>>) -> i32 {
    let mut uniq_beacons = beacons
        .iter()
        .filter_map(|beacon| {
            if beacon.1 == y_coord {
                Some(beacon.0)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    uniq_beacons.dedup();
    let covered_fields: i32 = ranges
        .iter()
        .map(|range| (range.start() - range.end()).abs() + 1)
        .sum();

    covered_fields - (uniq_beacons.len() as i32)
}

fn are_ranges_overlapping(a: RangeInclusive<i32>, b: RangeInclusive<i32>) -> bool {
    max(a.start(), b.start()) <= min(a.end(), b.end())
}

fn is_point_covered_by_sensors(point: Coords, sensors: &Vec<(Coords, i32)>) -> bool {
    sensors.iter().any(|(sensor, radius)| {
        (sensor.0 - point.0).abs() + (sensor.1 - point.1).abs() <= radius.clone()
    })
}

fn lone_beacon_coords(sensors: &Vec<(Coords, i32)>, max_dimension: i32) -> Option<Coords> {
    let boundary = 0..=max_dimension;
    for (sensor, radius) in sensors {
        // This could be a generator, collapse `search` iteration
        for dx in 0..=(radius + 2) {
            let dy = radius + 1 - dx;
            let search = [
                Coords(sensor.0 + dx, sensor.1 + dy),
                Coords(sensor.0 + dx, sensor.1 - dy),
                Coords(sensor.0 - dx, sensor.1 + dy),
                Coords(sensor.0 - dx, sensor.1 - dy),
            ]
            .iter()
            .filter(|point| boundary.contains(&point.0) && boundary.contains(&point.1))
            .filter_map(|point| {
                if !is_point_covered_by_sensors(point.clone(), sensors) {
                    Some(point.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

            if !search.is_empty() {
                return Some(search[0].clone());
            }
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let part_a_y_coord = 2000000;
    let max_dimension = 4000000;

    let (sensors, beacons) = parse_input(input);
    let covered_ranges = covered_ranges_for_y(part_a_y_coord, &sensors);

    let result_1 = impossible_beacons(part_a_y_coord, beacons, covered_ranges);
    let missing_beacon_coords = lone_beacon_coords(&sensors, max_dimension).unwrap();

    let result_2 =
        missing_beacon_coords.0 as u64 * max_dimension as u64 + missing_beacon_coords.1 as u64;

    println!("{:?}", result_1);
    println!("{:?}", result_2);
}
