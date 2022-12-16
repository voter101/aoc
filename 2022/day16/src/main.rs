use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(input: &String) -> (HashMap<String, i32>, HashMap<String, Vec<String>>) {
    let mut flows: HashMap<String, i32> = HashMap::new();
    let mut neighbours: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let regex = Regex::new(
            r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnel[a-z]{0,1} lead[a-z]{0,1} to valve[a-z]{0,1} (.+)$",
        )
        .unwrap();

        let matches = regex
            .captures(line)
            .unwrap()
            .iter()
            .map(|e| String::from(e.unwrap().as_str()))
            .skip(1)
            .collect::<Vec<_>>();

        let vertex: String = matches[0].clone();
        let flow: i32 = matches[1].parse::<i32>().unwrap();
        let neighbours_raw: String = matches[2].clone();

        let neighbours_vertices = neighbours_raw
            .split(", ")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        flows.insert(vertex.clone(), flow);
        neighbours.insert(vertex, neighbours_vertices);
    }

    (flows, neighbours)
}

fn meaningful_clique(
    flows: &HashMap<String, i32>,
    neighbours_map: &HashMap<String, Vec<String>>,
) -> HashMap<String, HashMap<String, i32>> {
    let meaningful_vertices = flows
        .iter()
        .filter(|&(key, value)| key.clone() == String::from("AA") || value.clone() > 0)
        .map(|(k, _)| k.clone())
        .collect::<Vec<_>>();

    let mut shortest_paths: HashMap<String, HashMap<String, i32>> = meaningful_vertices
        .clone()
        .iter()
        .map(|key| (key.clone(), HashMap::<String, i32>::new()))
        .collect::<HashMap<_, _>>();

    for from in &meaningful_vertices {
        for to in &meaningful_vertices {
            if from == to {
                continue;
            }

            let distance = shortest_path(
                from.clone(),
                to.clone(),
                0,
                neighbours_map,
                &HashSet::<String>::new(),
            )
            .unwrap();

            shortest_paths.entry(from.clone()).and_modify(|paths_map| {
                paths_map.insert(to.clone(), distance);
            });
        }
    }
    shortest_paths
}

fn shortest_path(
    from: String,
    to: String,
    current_distance: i32,
    neighbours_map: &HashMap<String, Vec<String>>,
    visited: &HashSet<String>,
) -> Option<i32> {
    if from == to {
        return Some(current_distance);
    }

    if visited.contains(&from) {
        return None;
    }

    let mut new_visited = visited.clone();
    new_visited.insert(from.clone());

    let possible_paths = neighbours_map
        .get(&from)
        .unwrap()
        .iter()
        .map(|n| {
            shortest_path(
                n.clone(),
                to.clone(),
                current_distance + 1,
                neighbours_map,
                &new_visited,
            )
        })
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    if possible_paths.is_empty() {
        None
    } else {
        Some(possible_paths.iter().min().unwrap().clone())
    }
}

fn highest_output(
    vertex: &String,
    current_flow: i32,
    remaining_time: i32,
    flows: &HashMap<String, i32>,
    neighbours_map: &HashMap<String, HashMap<String, i32>>,
    opened_valves: &HashSet<String>,
) -> i32 {
    if remaining_time <= 2 || opened_valves.len() == neighbours_map.len() {
        return current_flow;
    }

    let neighbours = neighbours_map
        .get(vertex)
        .unwrap()
        .iter()
        .filter(|(k, _)| !opened_valves.contains(k.clone()))
        .collect::<HashMap<_, _>>();

    if neighbours.is_empty() {
        return current_flow;
    }

    let mut opened_valves_new: HashSet<String> = opened_valves.clone();
    opened_valves_new.insert(vertex.clone());

    neighbours
        .iter()
        .map(|(&n, &path)| {
            let time_after_open = remaining_time - path - 1;

            let flow = if time_after_open > 0 {
                time_after_open * flows.get(n).unwrap()
            } else {
                0
            };

            highest_output(
                n,
                current_flow + flow,
                time_after_open,
                flows,
                neighbours_map,
                &opened_valves_new,
            )
        })
        .max()
        .unwrap()
}

fn highest_output_with_elephant(
    vertex_me: &String,
    vertex_elephant: &String,
    current_flow: i32,
    remaining_time_me: i32,
    remaining_time_elephant: i32,
    flows: &HashMap<String, i32>,
    neighbours_map: &HashMap<String, HashMap<String, i32>>,
    opened_valves: &HashSet<String>,
) -> i32 {
    if (remaining_time_me <= 2 && remaining_time_elephant <= 2)
        || opened_valves.len() == neighbours_map.len()
    {
        return current_flow;
    }

    let neighbours_me = neighbours_map
        .get(vertex_me)
        .unwrap()
        .iter()
        .filter(|_| remaining_time_me > 2)
        .filter(|(k, _)| !opened_valves.contains(k.clone()))
        .collect::<HashMap<_, _>>();
    let neighbours_elephant = neighbours_map
        .get(vertex_elephant)
        .unwrap()
        .iter()
        .filter(|_| remaining_time_elephant > 2)
        .filter(|(k, _)| !opened_valves.contains(k.clone()))
        .collect::<HashMap<_, _>>();

    if [&neighbours_me, &neighbours_elephant]
        .iter()
        .all(|n| n.is_empty())
    {
        return current_flow;
    }

    let mut results: Vec<i32> = vec![];

    for (&n, &path) in &neighbours_elephant {
        let mut opened_valves_new: HashSet<String> = opened_valves.clone();
        opened_valves_new.insert(n.clone());

        let time_after_open = remaining_time_elephant - path - 1;
        let flow = if time_after_open > 0 {
            time_after_open * flows.get(n).unwrap()
        } else {
            0
        };

        results.push(highest_output_with_elephant(
            vertex_me,
            n,
            current_flow + flow,
            remaining_time_me,
            time_after_open,
            flows,
            neighbours_map,
            &opened_valves_new,
        ));
    }

    for (&n, &path) in &neighbours_me {
        let mut opened_valves_new: HashSet<String> = opened_valves.clone();
        opened_valves_new.insert(n.clone());

        let time_after_open = remaining_time_me - path - 1;
        let flow = if time_after_open > 0 {
            time_after_open * flows.get(n).unwrap()
        } else {
            0
        };

        results.push(highest_output_with_elephant(
            n,
            vertex_elephant,
            current_flow + flow,
            time_after_open,
            remaining_time_elephant,
            flows,
            neighbours_map,
            &opened_valves_new,
        ));
    }

    results.iter().max().unwrap().clone()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let (flows, neighbours_map) = parse_input(&input);
    let meaningful_flows = meaningful_clique(&flows, &neighbours_map);
    let opened_values = HashSet::<String>::new();

    let max_flow = highest_output(
        &String::from("AA"),
        0,
        30,
        &flows,
        &meaningful_flows,
        &opened_values,
    );

    let max_flow_elephant = highest_output_with_elephant(
        &String::from("AA"),
        &String::from("AA"),
        0,
        26,
        26,
        &flows,
        &meaningful_flows,
        &opened_values,
    );

    println!("{:?}", max_flow);
    println!("{:?}", max_flow_elephant);
}
