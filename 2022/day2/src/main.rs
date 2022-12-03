use std::fs;

fn part_1(input: &String) -> i32 {
    let mut score = 0;

    for line in input.lines() {
        let round_elements: Vec<&str> = line.split_whitespace().collect();
        let picked_element_score = match round_elements[1] {
            "X" =>  1,
            "Y" =>  2,
            _ => 3
        };
        
        let to_add: i32 = match round_elements[0] {
            "A" => {
                match picked_element_score {
                    1 => 4,
                    2 => 8,
                    _ => 3
                }
            },
            "B" => {
                match picked_element_score {
                    1 => 1,
                    2 => 5,
                    _ => 9
                }
            },
            _ => {
                match picked_element_score {
                    1 => 7,
                    2 => 2,
                    _ => 6
                }
            }
        };

        score += to_add;
    }
    score
}

fn part_2(input: &String) -> i32 {
    let mut score = 0;

    for line in input.lines() {
        let round_elements: Vec<&str> = line.split_whitespace().collect();
        
        let to_add: i32 = match round_elements[0] {
            "A" => {
                match round_elements[1] {
                    "X" => 0 + 3,
                    "Y" => 3 + 1,
                    _ => 6 + 2
                }
            },
            "B" => {
                match round_elements[1] {
                    "X" => 0 + 1,
                    "Y" => 3 + 2,
                    _ => 6 + 3
                }
            },
            _ => {
                match round_elements[1] {
                    "X" => 0 + 2,
                    "Y" => 3 + 3,
                    _ => 6 + 1
                }
            }
        };

        score += to_add;
    }
    score
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}
