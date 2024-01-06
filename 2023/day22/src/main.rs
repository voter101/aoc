use std::cmp::{max, min};
use std::fs;

type Coords = (u32, u32, u32);

#[derive(Clone, PartialEq)]
struct Brick {
    end1: Coords,
    end2: Coords,
}

impl Brick {
    fn blocks(&self) -> Vec<Coords> {
        if self.end1.0 != self.end2.0 {
            (min(self.end1.0, self.end2.0)..=max(self.end1.0, self.end2.0))
                .map(|x| (x, self.end1.1, self.end1.2))
                .collect()
        } else if self.end1.1 != self.end2.1 {
            (min(self.end1.1, self.end2.1)..=max(self.end1.1, self.end2.1))
                .map(|y| (self.end1.0, y, self.end1.2))
                .collect()
        } else if self.end1.2 != self.end2.2 {
            (min(self.end1.2, self.end2.2)..=max(self.end1.2, self.end2.2))
                .map(|z| (self.end1.0, self.end1.1, z))
                .collect()
        } else {
            vec![self.end1]
        }
    }
}

fn parse_input(input: String) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let ends = line.split_once("~").unwrap();

            let end1_elems = ends
                .0
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let end2_elems = ends
                .1
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            Brick {
                end1: (end1_elems[0], end1_elems[1], end1_elems[2]),
                end2: (end2_elems[0], end2_elems[1], end2_elems[2]),
            }
        })
        .collect::<Vec<Brick>>()
}

fn stabilize_bricks(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut stabilized_bricks = bricks.clone();

    while !are_settled(&stabilized_bricks) {
        let to_move = stabilized_bricks
            .iter()
            .filter(|brick| can_move_down(brick, &stabilized_bricks))
            .map(|b| b.clone())
            .collect::<Vec<Brick>>();

        for brick in to_move {
            stabilized_bricks.iter_mut().for_each(|b| {
                if *b == brick {
                    b.end1 = (b.end1.0, b.end1.1, b.end1.2 - 1);
                    b.end2 = (b.end2.0, b.end2.1, b.end2.2 - 1);
                }
            });
        }
    }

    stabilized_bricks.clone()
}

fn are_settled(bricks: &Vec<Brick>) -> bool {
    bricks.iter().all(|brick| !can_move_down(brick, bricks))
}

fn can_move_down(brick: &Brick, bricks: &Vec<Brick>) -> bool {
    let brick_blocks = brick.blocks();

    if brick_blocks.iter().any(|b| b.2 == 1) {
        return false;
    }

    let potential_blocks = brick_blocks
        .iter()
        .map(|b| (b.0, b.1, b.2 - 1))
        .collect::<Vec<Coords>>();

    bricks.iter().filter(|b| **b != *brick).all(|b| {
        !potential_blocks
            .iter()
            .any(|potential_block| b.blocks().contains(potential_block))
    })
}

fn chain_reactions_count(bricks: &Vec<Brick>) -> usize {
    bricks
        .iter()
        .map(|brick| {
            let filtered_bricks = bricks
                .iter()
                .filter(|b| **b != *brick)
                .map(|b| b.clone())
                .collect::<Vec<Brick>>();

            let stabilized_bricks = stabilize_bricks(&filtered_bricks);

            (0..filtered_bricks.len())
                .filter(|i| filtered_bricks[*i] != stabilized_bricks[*i])
                .count()
        })
        .sum::<usize>()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let bricks = parse_input(input);

    let stabilized_bricks = stabilize_bricks(&bricks);

    let removable_bricks_count = stabilized_bricks
        .iter()
        .filter(|brick| {
            let filtered_bricks = stabilized_bricks
                .iter()
                .filter(|b| **b != **brick)
                .map(|b| b.clone())
                .collect::<Vec<Brick>>();

            are_settled(&filtered_bricks)
        })
        .count();

    println!("{:?}", removable_bricks_count);

    println!("{:?}", chain_reactions_count(&stabilized_bricks));
}
