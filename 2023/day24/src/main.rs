use std::fs;
use std::ops::RangeInclusive;

type Coords = (f64, f64, f64);

#[derive(Copy, Clone, Debug)]
struct Hailstone {
    position: Coords,
    velocity: Coords,
}

impl Hailstone {
    // ax + by = c; Get a, b, c
    fn standard_form_representation(&self) -> (f64, f64, f64) {
        (
            self.velocity.1,
            -self.velocity.0,
            self.position.0 * self.velocity.1 - self.position.1 * self.velocity.0,
        )
    }

    fn is_parallel(&self, other: Hailstone) -> bool {
        let (a1, b1, _) = self.standard_form_representation();
        let (a2, b2, _) = other.standard_form_representation();

        a1 * b2 == b1 * a2
    }
}

fn parse_input(input: String) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let split = line.split_once(" @ ").unwrap();
            let position = split
                .0
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.trim())
                .collect::<Vec<&str>>();
            let velocity = split
                .1
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            Hailstone {
                position: (
                    position[0].parse().unwrap(),
                    position[1].parse().unwrap(),
                    position[2].parse().unwrap(),
                ),
                velocity: (
                    velocity[0].parse().unwrap(),
                    velocity[1].parse().unwrap(),
                    velocity[2].parse().unwrap(),
                ),
            }
        })
        .collect::<Vec<Hailstone>>()
}

fn find_collisions(hailstones: &Vec<Hailstone>, range: RangeInclusive<f64>) -> usize {
    let mut result = 0;
    for i in 0..hailstones.len() {
        let hailstone = hailstones[i];
        for j in (i + 1)..hailstones.len() {
            let other = hailstones[j];

            if hailstone.is_parallel(other) {
                continue;
            }

            let (a1, b1, c1) = hailstone.standard_form_representation();
            let (a2, b2, c2) = other.standard_form_representation();

            let x = (b2 * c1 - b1 * c2) / (a1 * b2 - a2 * b1);
            let y = (a1 * c2 - a2 * c1) / (a1 * b2 - a2 * b1);

            if range.contains(&x) && range.contains(&y) {
                // Did the colission happen in the past? (before starting point)
                if (x - hailstone.position.0) * hailstone.velocity.0 >= 0.0
                    && (y - hailstone.position.1) * hailstone.velocity.1 >= 0.0
                    && (x - other.position.0) * other.velocity.0 >= 0.0
                    && (y - other.position.1) * other.velocity.1 >= 0.0
                {
                    result += 1
                }
            }
        }
    }
    result
}

fn print_z3_list_perfect_rock(hailstones: &Vec<Hailstone>) -> () {
    println!(
        r#"
(declare-const x Int)
(declare-const y Int)
(declare-const z Int)
(declare-const vx Int)
(declare-const vy Int)
(declare-const vz Int)
(declare-const t1 Int)
(declare-const t2 Int)
(declare-const t3 Int)
(declare-const t4 Int)
(declare-const t5 Int)
    "#
    );
    for (i, hailstone) in hailstones.iter().take(5).enumerate() {
        let (x2, y2, z2, vx2, vy2, vz2) = (
            hailstone.position.0,
            hailstone.position.1,
            hailstone.position.2,
            hailstone.velocity.0,
            hailstone.velocity.1,
            hailstone.velocity.2,
        );
        println!("(assert (>= t{} 0))", i + 1);
        println!(
            "(assert (= (+ (* t{} vx) x) (+ (* {} t{}) {})))",
            i + 1,
            vx2,
            i + 1,
            x2
        );
        println!(
            "(assert (= (+ (* t{} vy) y) (+ (* {} t{}) {})))",
            i + 1,
            vy2,
            i + 1,
            y2
        );
        println!(
            "(assert (= (+ (* t{} vz) z) (+ (* {} t{}) {})))",
            i + 1,
            vz2,
            i + 1,
            z2
        );
    }
    println!(
        r#"
(check-sat)
(get-model)
    "#
    );
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let hailstones = parse_input(input);

    let part_1 = find_collisions(
        &hailstones,
        (200000000000000.0 as f64)..=(400000000000000.0 as f64),
    );

    println!("{:?}", part_1);

    // Take output of this print and put into `z3`
    print_z3_list_perfect_rock(&hailstones);
}
