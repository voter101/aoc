use std::collections::HashMap;
use std::fs;
use text_io::scan;

fn folder_path(path: Vec<String>) -> String {
    path.join("/")
}

fn add_file_to_path(file_size: usize, path: &mut Vec<String>, folders: &mut HashMap<String, usize>) {
    let prefixes = (0..=path.len())
        .map(|len| &path[..len])
        .map(|arr| arr.iter().cloned().collect::<Vec<String>>());
    for p in prefixes {
        let entry_path = folder_path(p);
        let entry = folders.entry(entry_path).or_insert(0);
        *entry += file_size;
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let mut folders: HashMap<String, usize> = HashMap::new();
    let mut current_path: Vec<String> = vec![];

    for line in input.lines() {
        match &line[..1] {
            "$" => {
                match &line[..4] {
                    "$ ls" => {},
                    _ => {
                        let command_param: String;
                        scan!(line.bytes() => "$ cd {}", command_param);
                        if command_param == ".." {
                            current_path.pop();
                        } else {
                            current_path.push(command_param);
                        }
                    }
                }
            },
            "d" => {
                let dir_name: String;
                scan!(line.bytes() => "dir {}", dir_name);
                folders.insert(dir_name, 0);
            },
            _ => {
                let size: usize;
                let mut _file_name: String;
                scan!(line.bytes() => "{} {}", size, _file_name);
                add_file_to_path(size, &mut current_path, &mut folders);
            },
        };
    }

    let result_1: usize = folders
        .iter()
        .filter(|&(_, size)| size <= &100000)
        .map(|(_, size)| size)
        .sum();
    
    println!("{}", result_1);

    let required_space: usize = 30000000;
    let total_space: usize = 70000000;
    let leftover_space: usize = total_space - folders.entry(String::from("/")).or_insert(0).clone();

    let result_2 = folders
        .iter()
        .map(|(_, size)| leftover_space + size)
        .filter(|size| size.clone() >= required_space)
        .min()
        .unwrap() - leftover_space;

    println!("{}", result_2);
}
