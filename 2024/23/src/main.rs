use std::collections::HashSet;
use std::fs;

type AdjacencyMatrix = HashSet<(String, String)>;

fn parse_input(input: String) -> (AdjacencyMatrix, HashSet<String>) {
    let mut adjacency_matrix = HashSet::new();
    let mut interesting_nodes: HashSet<String> = HashSet::new();

    input.lines().for_each(|l| {
        let (a, b) = l.split_once("-").unwrap();

        if a.starts_with("t") {
            interesting_nodes.insert(a.to_string());
        }

        if b.starts_with("t") {
            interesting_nodes.insert(b.to_string());
        }

        adjacency_matrix.insert((a.to_string(), b.to_string()));
        adjacency_matrix.insert((b.to_string(), a.to_string()));
    });

    (adjacency_matrix, interesting_nodes)
}

fn find_lan_connections(
    matrix: &AdjacencyMatrix,
    interesting_nodes: &HashSet<String>,
) -> HashSet<(String, String, String)> {
    let mut res = HashSet::new();

    for n_1 in interesting_nodes.iter() {
        let candidates = matrix
            .iter()
            .filter(|(a, _)| a == n_1)
            .map(|(_, b)| b)
            .collect::<Vec<_>>();

        for i in 0..candidates.len() {
            for j in i + 1..candidates.len() {
                let n_2 = candidates[i];
                let n_3 = candidates[j];

                if matrix.contains(&(n_2.clone(), n_3.clone())) {
                    let mut triple = vec![n_1, n_2, n_3];
                    triple.sort();
                    res.insert((triple[0].clone(), triple[1].clone(), triple[2].clone()));
                }
            }
        }
    }

    res
}

fn largest_clique(matrix: &AdjacencyMatrix) -> Vec<String> {
    let mut cliques: Vec<Vec<String>> = vec![];
    let r: HashSet<String> = HashSet::new();
    let mut p: HashSet<String> = HashSet::new();
    let mut x: HashSet<String> = HashSet::new();

    matrix.iter().for_each(|(a, _)| {
        p.insert(a.to_string());
    });

    bron_kerbosch(&r, &mut p, &mut x, matrix, &mut cliques);

    cliques
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .clone()
}

fn bron_kerbosch(
    r: &HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    matrix: &AdjacencyMatrix,
    cliques: &mut Vec<Vec<String>>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > cliques.iter().map(|c| c.len()).max().unwrap_or(0) {
            let mut clique: Vec<String> = r.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }

        return;
    }

    for v in p.clone().iter() {
        let mut new_r = r.clone();
        new_r.insert(v.clone());

        let neighbours_v = neighbours(v, matrix);
        let mut new_p = p
            .intersection(&neighbours_v)
            .cloned()
            .collect::<HashSet<String>>();

        let mut new_x = x
            .intersection(&neighbours_v)
            .cloned()
            .collect::<HashSet<String>>();

        bron_kerbosch(&new_r, &mut new_p, &mut new_x, matrix, cliques);

        p.remove(v);
        x.insert(v.to_string());
    }
}

fn neighbours(v: &String, matrix: &AdjacencyMatrix) -> HashSet<String> {
    matrix
        .iter()
        .filter(|(a, _)| a == v)
        .map(|(_, b)| b.clone())
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file not loaded");
    let (adjacency_matrix, interesting_nodes) = parse_input(input);
    let triples = find_lan_connections(&adjacency_matrix, &interesting_nodes);

    println!("{}", triples.len());

    let largest_clique = largest_clique(&adjacency_matrix);

    println!("{}", largest_clique.join(","));
}
