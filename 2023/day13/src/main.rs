use std::fs;

fn block_value(block: String, require_change: bool) -> usize {
    let lines: Vec<String> = block.lines().map(|line| line.to_string()).collect();

    'outer: for i in 0..lines.len() - 1 {
        let mut changed: bool = false;

        let line_diff_init = lines_equal(&lines[i], &lines[i + 1]);

        if line_diff_init == 1 && require_change {
            changed = true;
        } else if line_diff_init >= 1 {
            continue;
        }

        let mut ii = i;
        let mut jj = i + 1;
        loop {
            if ii == 0 || jj == lines.len() - 1 {
                break;
            }

            ii = ii - 1;
            jj = jj + 1;

            let l_diff = lines_equal(&lines[ii], &lines[jj]);

            if l_diff > 1 || !require_change && l_diff > 0 {
                continue 'outer;
            }

            if require_change {
                if changed && l_diff == 1 {
                    continue 'outer;
                } else if !changed && l_diff == 1 {
                    changed = true;
                }
            }
        }
        if require_change && !changed {
            continue;
        }

        return (i + 1) * 100;
    }

    let column_length = lines[0].len();

    'outer: for i in 0..(column_length - 1) {
        let mut changed: bool = false;

        let col_diff_init = lines_equal(&get_column(&block, i), &get_column(&block, i + 1));

        if col_diff_init == 1 && require_change {
            changed = true;
        } else if col_diff_init >= 1 {
            continue;
        }

        let mut ii = i;
        let mut jj = i + 1;
        loop {
            if ii == 0 || jj == column_length - 1 {
                break;
            }

            ii = ii - 1;
            jj = jj + 1;

            let c_diff = lines_equal(&get_column(&block, ii), &get_column(&block, jj));

            if c_diff > 1 || !require_change && c_diff > 0 {
                continue 'outer;
            }

            if require_change {
                if changed && c_diff == 1 {
                    continue 'outer;
                } else if !changed && c_diff == 1 {
                    changed = true;
                }
            }
        }
        if require_change && !changed {
            continue;
        }

        return i + 1;
    }
    0
}

fn lines_equal(line1: &String, line2: &String) -> usize {
    line1
        .chars()
        .zip(line2.chars())
        .map(|(a, b)| a == b)
        .filter(|x| !x)
        .count()
}

fn get_column(block: &String, i: usize) -> String {
    block
        .lines()
        .map(|line| line.chars().take(i + 1).last().unwrap())
        .collect()
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("File not loaded");
    let blocks = input.split("\n\n");

    let final_value: usize = blocks
        .clone()
        .map(|block| block_value(block.to_string(), false))
        .sum();
    let final_smudged_value: usize = blocks
        .map(|block| block_value(block.to_string(), true))
        .sum();

    println!("{:?}", final_value);
    println!("{:?}", final_smudged_value);
}
