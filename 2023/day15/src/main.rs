use std::fs;
use std::ops::Rem;

type Lens = (String, usize);

fn sequence_values(input: &String) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|chars| hash_algo_value(chars.to_string()))
        .collect()
}

fn hash_algo_value(chars: String) -> usize {
    let mut val = 0;
    for c in chars.chars() {
        val = ((val + ((c as u8) as usize)) * 17).rem(256);
    }
    val
}

fn put_lenses_to_boxes(input: &String) -> Vec<Vec<Lens>> {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    input.trim().split(',').for_each(|chars| {
        if chars.chars().last().unwrap() == '-' {
            let label = chars.chars().take(chars.len() - 1).collect::<String>();
            let box_num = hash_algo_value(label.clone());
            let lens_index = match boxes[box_num].iter().position(|x| x.0 == label) {
                Some(i) => i,
                None => return,
            };
            boxes[box_num].remove(lens_index);
        } else {
            let (label, value) = chars.split_once('=').unwrap();
            let box_num = hash_algo_value(label.to_string());

            if boxes[box_num]
                .iter()
                .any(|(label_in_box, _)| label == label_in_box)
            {
                let lens_index = boxes[box_num]
                    .iter()
                    .position(|(label_in_box, _)| label == label_in_box)
                    .unwrap();
                boxes[box_num][lens_index].1 = value.parse::<usize>().unwrap();
            } else {
                boxes[box_num].push((label.to_string(), value.parse::<usize>().unwrap()));
            }
        }
    });

    boxes
}

fn boxes_value(boxes: &Vec<Vec<Lens>>) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(box_i, box_single)| {
            box_single
                .iter()
                .enumerate()
                .map(|(lens_i, (_, value))| (box_i + 1) * (lens_i + 1) * value)
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("File not loaded");
    println!("{}", sequence_values(&input).iter().sum::<usize>());

    let boxes = put_lenses_to_boxes(&input);

    println!("{:?}", boxes_value(&boxes));
}
