use std::collections::HashMap;
use std::fs;

fn parse_input(input: String) -> (Vec<String>, Vec<String>) {
    let (raw_stripes, raw_designs) = input.split_once("\n\n").unwrap();
    let stripes = raw_stripes.split(", ").map(|x| x.to_string()).collect();
    let designs = raw_designs.lines().map(|x| x.to_string()).collect();

    (stripes, designs)
}

fn valid_designs(
    design: &String,
    stripes: &Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.len() == 0 {
        return 1;
    }

    if cache.contains_key(design) {
        return cache[design];
    }

    let mut result = 0;

    for stripe in stripes.iter() {
        if design.starts_with(stripe) {
            let new_design = design.strip_prefix(stripe).unwrap().to_string();
            result += valid_designs(&new_design, stripes, cache);
        }
    }

    cache.insert(design.clone(), result);

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let (stripes, designs) = parse_input(input);
    let mut cache: HashMap<String, usize> = HashMap::new();
    let valid_designs_vec = designs
        .iter()
        .map(|x| valid_designs(x, &stripes, &mut cache))
        .collect::<Vec<usize>>();

    println!("{}", valid_designs_vec.iter().filter(|x| **x > 0).count());
    println!("{}", valid_designs_vec.iter().sum::<usize>());
}
