use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
enum DiskField {
    Empty,
    File(usize),
}

fn construct_disk_space(input: String) -> Vec<DiskField> {
    let mut res = vec![];
    let mut current_file = 0;

    input
        .chars()
        .take_while(|c| *c != '\n')
        .enumerate()
        .for_each(|(i, c)| {
            let is_file = i % 2 == 0;
            let block_size = c.to_digit(10).unwrap();
            let element = if is_file {
                DiskField::File(current_file)
            } else {
                DiskField::Empty
            };

            if is_file {
                current_file += 1;
            }

            res.extend(vec![element; block_size as usize]);
        });

    res
}

fn compact_disk_space_fragmented(disk_space: &Vec<DiskField>) -> Vec<DiskField> {
    let mut res = disk_space.clone();

    let mut l = 0;
    let mut r = res.len() - 1;

    while l < r {
        if let DiskField::File(_) = res[l] {
            l += 1;
            continue;
        }

        while r > l {
            if let DiskField::File(_) = res[r] {
                break;
            }
            r -= 1;
        }

        res[l] = res[r].clone();
        res[r] = DiskField::Empty;
    }

    res
}

fn compact_disk_space(disk: &Vec<DiskField>) -> Vec<DiskField> {
    let mut res = disk.clone();

    let mut r = res.len() - 1;

    while r > 0 {
        if let Some((file_index, file_size, file)) = find_file(&res, r) {
            if let Some(gap_index) = find_gap(&res, file_size, r) {
                for i in 0..file_size {
                    res[file_index - i] = DiskField::Empty;
                    res[gap_index + i] = DiskField::File(file);
                }
            }
            r = file_index - file_size;
        } else {
            break;
        }
    }

    res
}

fn find_file(disk: &Vec<DiskField>, starting_point: usize) -> Option<(usize, usize, usize)> {
    let mut r = starting_point;

    while r > 0 {
        let current_file: usize;

        match disk[r] {
            DiskField::File(f) => current_file = f,
            DiskField::Empty => {
                r -= 1;
                continue;
            }
        }

        let mut file_size = 1;
        let mut i = r - 1;

        while i > 0 {
            match disk[i] {
                DiskField::File(f) => {
                    if f != current_file {
                        break;
                    }
                    file_size += 1;
                }
                _ => break,
            }
            i -= 1;
        }

        return Some((r, file_size, current_file));
    }

    None
}

fn find_gap(disk: &Vec<DiskField>, required_size: usize, max_r: usize) -> Option<usize> {
    let mut i = 0;

    while i < max_r {
        if let DiskField::File(_) = disk[i] {
            i += 1;
            continue;
        }

        let mut gap_size = 1;
        let mut j = i + 1;

        while j < max_r {
            if let DiskField::Empty = disk[j] {
                gap_size += 1;
                j += 1;
            } else {
                break;
            }
        }

        if gap_size >= required_size {
            return Some(i);
        } else {
            i += gap_size;
        }
    }
    None
}

fn disk_checksum(disk: &Vec<DiskField>) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, field)| {
            if let DiskField::File(f) = field {
                f * i
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let disk_space = construct_disk_space(input);
    let fragmented_disk_space = compact_disk_space_fragmented(&disk_space);
    let result_1 = disk_checksum(&fragmented_disk_space);

    println!("{:?}", result_1);

    let compacted_disk_space = compact_disk_space(&disk_space);
    let result_2 = disk_checksum(&compacted_disk_space);

    println!("{:?}", result_2);
}
