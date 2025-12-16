use std::collections::HashMap;
use std::fs;

type Connections = HashMap<String, Vec<String>>;

fn parse_input() -> Connections {
    fs::read_to_string("input.txt")
        // fs::read_to_string("example.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (key, outputs_raw) = line.split_once(": ").unwrap();
            let outputs = outputs_raw
                .split(" ")
                .map(|e| e.to_string())
                .collect::<Vec<String>>();
            (key.to_string(), outputs)
        })
        .collect()
}

fn count_paths(
    current: String,
    end: String,
    conn: &Connections,
    visited: &Vec<String>,
    cache: &mut HashMap<(String, String), usize>,
) -> usize {
    if let Some(val) = cache.get(&(current.clone(), end.clone())) {
        return *val;
    }

    if current == end {
        cache.insert((current, end), 1);
        return 1;
    }

    if let Some(candidates) = conn.get(&current) {
        let mut new_visited = visited.clone();
        new_visited.push(current);
        return candidates
            .iter()
            .map(|candidate| {
                if visited.contains(candidate) {
                    return 0;
                }

                let res = count_paths(candidate.clone(), end.clone(), conn, &new_visited, cache);
                cache.insert((candidate.clone(), end.clone()), res);
                res
            })
            .sum::<usize>();
    } else {
        return 0;
    }
}

fn count_paths_with_points(start: String, end: String, conn: &Connections) -> usize {
    let mut cache = HashMap::new();
    let start_dac = count_paths(
        start.clone(),
        String::from("dac"),
        conn,
        &vec![],
        &mut cache,
    );
    let start_fft = count_paths(
        start.clone(),
        String::from("fft"),
        conn,
        &vec![],
        &mut cache,
    );
    let dac_fft = count_paths(
        String::from("dac"),
        String::from("fft"),
        conn,
        &vec![],
        &mut cache,
    );
    let fft_dac = count_paths(
        String::from("fft"),
        String::from("dac"),
        conn,
        &vec![],
        &mut cache,
    );
    let dac_end = count_paths(String::from("dac"), end.clone(), conn, &vec![], &mut cache);
    let fft_end = count_paths(String::from("fft"), end.clone(), conn, &vec![], &mut cache);

    if fft_dac == 0 {
        start_dac * dac_fft * fft_end
    } else {
        start_fft * fft_dac * dac_end
    }
}

fn main() {
    let connections: HashMap<String, Vec<String>> = parse_input();
    println!(
        "{}",
        count_paths(
            String::from("you"),
            String::from("out"),
            &connections,
            &vec![],
            &mut HashMap::new()
        )
    );

    println!(
        "{}",
        count_paths_with_points(String::from("svr"), String::from("out"), &connections,)
    );
}
