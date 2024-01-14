use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(input: &String) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    input.lines().for_each(|line| {
        let elements = line.split_once(": ").unwrap();
        let source = elements.0.to_string();
        let other: HashSet<String> =
            HashSet::from_iter(elements.1.split(" ").map(|s| s.to_string()));

        if map.contains_key(&source) {
            let key = map.get_mut(&source).unwrap();
            other.iter().for_each(|s| {
                key.insert(s.to_string());
            });
        } else {
            map.insert(source.clone(), other.clone());

            other.iter().for_each(|key| {
                if map.contains_key(key) {
                    let key = map.get_mut(key).unwrap();
                    key.insert(source.to_string());
                } else {
                    let mut set = HashSet::new();
                    set.insert(source.to_string());
                    map.insert(key.to_string(), set);
                }
            });
        }
    });

    map
}

fn subgraph_size(map: &HashMap<String, HashSet<String>>, root: String) -> usize {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: Vec<String> = Vec::new();

    queue.push(root);

    while !queue.is_empty() {
        let current = queue.pop().unwrap();

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current.clone());

        match map.get(&current) {
            Some(neighbours) => {
                neighbours.iter().for_each(|neighbour| {
                    queue.push(neighbour.clone());
                });
            }
            None => {}
        }
    }

    visited.len()
}

// To get proper output, modify input to graphviz format
// And run it with `sfdp -Tpdf input_graphviz.txt -o output.pdf`
fn main() {
    // Remember to manually remove the connections
    let input = fs::read_to_string("input.txt").unwrap();
    let map = parse_input(&input);

    // Find some nodes on both graphs
    let result = subgraph_size(&map, "std".to_string()) * subgraph_size(&map, "vgs".to_string());

    println!("{}", result);
}
