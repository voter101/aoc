use std::fs;

fn sorted_coordinate_lists(input: String) -> (Vec<u32>, Vec<u32>) {
    let mut list_1: Vec<u32> = vec![];
    let mut list_2: Vec<u32> = vec![];
    input.lines().for_each(|line| {
        let (num_1, num_2) = line.split_once("   ").unwrap();
        list_1.push(num_1.parse::<u32>().unwrap());
        list_2.push(num_2.parse::<u32>().unwrap());
    });

    list_1.sort();
    list_2.sort();

    (list_1, list_2)
}

fn difference_list(list_1: &Vec<u32>, list_2: &Vec<u32>) -> Vec<u32> {
    list_1
        .iter()
        .zip(list_2.iter())
        .map(|(e1, e2)| ((e1.clone() as i32) - (e2.clone() as i32)).abs() as u32)
        .collect::<_>()
}

fn similarity_score(list_1: &Vec<u32>, list_2: &Vec<u32>) -> u32 {
    list_1.iter().fold(0 as u32, |acc, e| {
        acc + (list_2.iter().filter(|el| *el == e).count() as u32) * e
    })
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let coordinate_lists: (Vec<u32>, Vec<u32>) = sorted_coordinate_lists(input);

    let result_1: u32 = difference_list(&coordinate_lists.0, &coordinate_lists.1)
        .iter()
        .sum();

    println!("{}", result_1);

    println!(
        "{}",
        similarity_score(&coordinate_lists.0, &coordinate_lists.1)
    )
}
