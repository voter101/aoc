use std::cmp::{max, min};
use std::fs;
use std::ops::Range;

fn parse_seeds(line: &str) -> Vec<u64> {
    line.split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_map_segment(segment: &str) -> Vec<(u64, u64, u64)> {
    segment
        .lines()
        .skip(1)
        .map(|line| {
            let numbers: Vec<u64> = line
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<_>();
            (numbers[0], numbers[1], numbers[2])
        })
        .collect::<_>()
}

fn seed_ranges(seeds: &Vec<u64>) -> Vec<Range<u64>> {
    seeds
        .windows(2)
        .step_by(2)
        .map(|slice| {
            let start = slice.first().unwrap();
            let range = slice.last().unwrap();
            (start.clone())..(start + range).clone()
        })
        .collect()
}

fn process_map_segment(source: Vec<u64>, segment: &Vec<(u64, u64, u64)>) -> Vec<u64> {
    source
        .iter()
        .map(|source_value| {
            for (destination_start, source_start, range) in segment.iter() {
                if (source_start..=&(source_start + range)).contains(&source_value) {
                    let diff = source_value - source_start;
                    return destination_start + diff;
                }
            }
            source_value.clone()
        })
        .collect::<Vec<_>>()
}

fn process_map_segment_with_range(
    source: Vec<Range<u64>>,
    segment: &Vec<(u64, u64, u64)>,
) -> Vec<Range<u64>> {
    let mut stack = source.clone();
    let mut result: Vec<Range<u64>> = vec![];

    while !stack.is_empty() {
        let mut element = stack.pop().unwrap().clone();
        let mut result_element: Vec<Range<u64>> = vec![];

        for (destination_start, source_start, range) in segment.iter() {
            let source_end = source_start + range;

            if element.end < *source_start || element.start > source_end {
                continue;
            }

            let intersection = max(element.start, *source_start)..min(element.end, source_end);

            if intersection.start != element.start {
                stack.push(element.start..(intersection.start - 1));
                element.start = intersection.start;
            }

            if intersection.end != element.end {
                stack.push((intersection.end + 1)..element.end);
                element.end = intersection.end;
            }

            let diff_start = intersection.start - source_start;
            let diff_end = intersection.end - intersection.start;

            result_element.push(
                (destination_start + diff_start)..(destination_start + diff_start + diff_end),
            );
        }

        if result_element.is_empty() {
            result.push(element);
        } else {
            result.append(&mut result_element);
        }
    }

    result
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("File not loaded");
    let almanac_pieces = input.split("\n\n").collect::<Vec<_>>();
    let seeds = parse_seeds(almanac_pieces[0]);
    let maps = almanac_pieces
        .iter()
        .skip(1)
        .map(|piece| parse_map_segment(piece))
        .collect::<Vec<_>>();

    let seed_ranges = seed_ranges(&seeds);

    let single_seeds_locations = maps.iter().fold(seeds, |source, segment| {
        process_map_segment(source, segment)
    });

    let seeds_ranges_locations = maps.iter().fold(seed_ranges, |source, segment| {
        process_map_segment_with_range(source, segment)
    });

    println!("{:?}", single_seeds_locations.iter().min().unwrap());
    println!(
        "{:?}",
        seeds_ranges_locations
            .iter()
            .map(|x| x.start)
            .min()
            .unwrap()
    );
}
