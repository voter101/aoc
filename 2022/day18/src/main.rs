use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use text_io::scan;

fn parse_input(input: &String) -> HashSet<(i32, i32, i32)> {
    let mut set: HashSet<(i32, i32, i32)> = HashSet::new();
    input.lines().for_each(|line| {
        let (x, y, z): (i32, i32, i32);

        scan!(line.bytes() => "{},{},{}", x,y,z);

        set.insert((x, y, z));
    });
    set
}

// Optimization: This could have been done during input parsing
fn max_dim(lava_blocks: &HashSet<(i32, i32, i32)>) -> (i32, i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z: i32 = 0;

    for &(x, y, z) in lava_blocks {
        max_x = max(max_x, x);
        max_y = max(max_y, y);
        max_z = max(max_z, z);
    }

    (max_x + 1, max_y + 1, max_z + 1)
}

fn add_next_neighbours(
    pos: &(i32, i32, i32),
    stack: &mut Vec<(i32, i32, i32)>,
    air_blocks: &HashSet<(i32, i32, i32)>,
    lava_blocks: &HashSet<(i32, i32, i32)>,
    (max_x, max_y, max_z): (i32, i32, i32),
) {
    for modifier in [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ] {
        let mut block_temp = pos.clone();
        block_temp.0 += modifier.0;
        block_temp.1 += modifier.1;
        block_temp.2 += modifier.2;

        if block_temp.0 > max_x
            || block_temp.0 < -1
            || block_temp.1 > max_y
            || block_temp.1 < -1
            || block_temp.2 > max_z
            || block_temp.2 < -1
        {
            continue;
        }

        if !air_blocks.contains(&block_temp) && !lava_blocks.contains(&block_temp) {
            stack.push(block_temp);
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let lava_blocks = parse_input(&input);
    let maximums = max_dim(&lava_blocks);

    let mut counter_1 = 0;
    let mut counter_2 = 0;

    for block in &lava_blocks {
        for modifier in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let mut block_temp = block.clone();
            block_temp.0 += modifier.0;
            block_temp.1 += modifier.1;
            block_temp.2 += modifier.2;

            if !lava_blocks.contains(&block_temp) {
                counter_1 += 1;
            }
        }
    }

    // Part 2:
    //
    // Idea: Put lava blob into a rectangul cuboid (special case for point on
    //       a 0-dimension, hence start from (-1, -1, -1)) and then look for all
    //       neighbouring lava blocks.

    // Part 1: create all possible air blocks. There is probably a lot of excess
    //         I did not mind it.
    let mut air_blocks: HashSet<(i32, i32, i32)> = HashSet::new();
    air_blocks.insert((-1, -1, -1));
    let mut stack: Vec<(i32, i32, i32)> = vec![];
    let mut pos = (-1, -1, -1);
    loop {
        if !lava_blocks.contains(&pos) {
            air_blocks.insert(pos);
        }

        add_next_neighbours(&pos, &mut stack, &air_blocks, &lava_blocks, maximums);
        let new_pos = stack.pop();
        match new_pos {
            Some(x) => pos = x,
            None => break,
        }
    }

    // Part 2: Find all lava blocks neighbouring
    //
    // Optimization idea: we can do that counting during part 1 and call it a
    //                    day. For me it was way easier to think about it this
    //                    way.
    for block in &air_blocks {
        for modifier in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let mut block_temp = block.clone();
            block_temp.0 += modifier.0;
            block_temp.1 += modifier.1;
            block_temp.2 += modifier.2;

            if lava_blocks.contains(&block_temp) {
                counter_2 += 1;
            }
        }
    }
    println!("{}", counter_1);
    println!("{}", counter_2);
}
