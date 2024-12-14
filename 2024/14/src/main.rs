use std::fs;
use text_io::scan;

#[derive(Debug)]
struct Machine {
    pos: (usize, usize),
    vel: (isize, isize),
}

impl Machine {
    fn step(&mut self, map_size: (usize, usize)) {
        self.pos.0 = (self.pos.0 as isize + self.vel.0).rem_euclid(map_size.0 as isize) as usize;
        self.pos.1 = (self.pos.1 as isize + self.vel.1).rem_euclid(map_size.1 as isize) as usize;
    }
}

fn parse_machines(input: String) -> Vec<Machine> {
    let mut res = vec![];
    input.lines().for_each(|line| {
        let mut pos: (usize, usize) = (0, 0);
        let mut vel: (isize, isize) = (0, 0);
        scan!(line.bytes() => "p={},{} v={},{}", pos.0, pos.1, vel.0, vel.1);
        res.push(Machine { pos, vel });
    });

    res
}

fn machines_in_quadrants(machines: &Vec<Machine>, map_size: (usize, usize)) -> Vec<usize> {
    let mut res = vec![0, 0, 0, 0];
    let middle_x = map_size.0 / 2;
    let middle_y = map_size.1 / 2;

    machines.iter().for_each(|m| {
        if m.pos.0 < middle_x && m.pos.1 < middle_y {
            res[0] += 1;
        } else if m.pos.0 > middle_x && m.pos.1 > middle_y {
            res[1] += 1;
        } else if m.pos.0 < middle_x && m.pos.1 > middle_y {
            res[2] += 1;
        } else if m.pos.0 > middle_x && m.pos.1 < middle_y {
            res[3] += 1;
        }
    });

    res
}

fn create_map(machines: &Vec<Machine>, map_size: (usize, usize)) -> Vec<String> {
    let mut res = vec![vec!['.'; map_size.0]; map_size.1];
    machines.iter().for_each(|m| {
        res[m.pos.1][m.pos.0] = '#';
    });

    res.iter().map(|row| String::from_iter(row)).collect()
}

fn display_map(machines: &Vec<Machine>, map_size: (usize, usize)) {
    for y in 0..map_size.1 {
        for x in 0..map_size.0 {
            if machines.iter().any(|m| m.pos == (x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let map_size = (101, 103);
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut machines = parse_machines(input);

    for _ in 0..100 {
        machines.iter_mut().for_each(|m| m.step(map_size));
    }

    let result_1 = machines_in_quadrants(&machines, map_size)
        .iter()
        .product::<usize>();

    let mut i = 101;

    loop {
        if map_size.0 < 20 || machines.len() < 20 {
            println!("Part 2 doesn't work for example input");
            i = 0;
            break;
        }
        machines.iter_mut().for_each(|m| m.step(map_size));
        let map = create_map(&machines, map_size);
        if map.iter().any(|line| line.contains("############")) {
            display_map(&machines, map_size);
            break;
        }
        i += 1;
    }

    println!("{}", result_1);
    println!("{}", i);
}
