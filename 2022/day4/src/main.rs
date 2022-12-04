use std::cmp::{max, min};
use std::fs;
use text_io::scan;

fn main() {
    let mut overlap_count = 0;
    let mut nonoverlap_count = 0;
    let input = fs::read_to_string("./input.txt").expect("File not loaded");

    for line in input.lines() {
        let mut first_elf: (u32, u32) = (0, 0);
        let mut second_elf: (u32, u32) = (0, 0);
        scan!(line.bytes() => "{}-{},{}-{}", first_elf.0, first_elf.1, second_elf.0, second_elf.1);

        if (first_elf.0 >= second_elf.0 && first_elf.1 <= second_elf.1)
            || (second_elf.0 >= first_elf.0 && second_elf.1 <= first_elf.1)
        {
            overlap_count += 1;
        }

        if max(first_elf.0, second_elf.0) <= min(first_elf.1, second_elf.1) {
            nonoverlap_count += 1;
        }
    }

    println!("{}", overlap_count);
    println!("{}", nonoverlap_count);
}
