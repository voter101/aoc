use std::collections::HashSet;
use std::fs;

type Coords = (usize, usize);

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Machine,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
struct Guard {
    pos: Coords,
    direction: GuardDirection,
}

#[derive(Clone)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    guard: Guard,
}

impl Guard {
    fn next_pos(&self, board_size: Coords) -> Option<Coords> {
        let (x, y) = self.pos;
        let (dx, dy) = match self.direction {
            GuardDirection::Up => (0, -1),
            GuardDirection::Right => (1, 0),
            GuardDirection::Down => (0, 1),
            GuardDirection::Left => (-1, 0),
        };

        let new_pos = (x as isize + dx, y as isize + dy);

        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= board_size.0 as isize
            || new_pos.1 >= board_size.1 as isize
        {
            return None;
        }

        Some((new_pos.0 as usize, new_pos.1 as usize))
    }

    fn next_direction(&self) -> GuardDirection {
        match self.direction {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Right => GuardDirection::Down,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
        }
    }
}

impl Board {
    fn size(&self) -> Coords {
        (self.tiles[0].len(), self.tiles.len())
    }
}

fn parse_input(input: String) -> Board {
    let mut guard_pos: Coords = (0, 0);

    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Machine,
                    '^' => {
                        guard_pos = (x, y);
                        Tile::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let guard = Guard {
        pos: guard_pos,
        direction: GuardDirection::Up,
    };

    Board { tiles, guard }
}

// None if detected a loop
fn simulate_walk(base_board: &Board) -> Option<HashSet<Coords>> {
    let mut board = base_board.clone();

    // I could have had only one HashSet, but I am too lazy to transform in the end of the function
    let mut visited: HashSet<(Coords, GuardDirection)> = HashSet::new();
    let mut visited_pos: HashSet<Coords> = HashSet::new();
    visited.insert((board.guard.pos, board.guard.direction));
    visited_pos.insert(board.guard.pos);

    loop {
        if let Some(new_pos) = board.guard.next_pos(board.size()) {
            if visited.contains(&(new_pos, board.guard.direction)) {
                return None;
            }
            let new_tile = board.tiles[new_pos.1 as usize][new_pos.0 as usize];

            match new_tile {
                Tile::Empty => {
                    board.guard.pos = (new_pos.0 as usize, new_pos.1 as usize);

                    visited.insert((
                        (new_pos.0 as usize, new_pos.1 as usize),
                        board.guard.direction,
                    ));
                    visited_pos.insert((new_pos.0 as usize, new_pos.1 as usize));
                }
                Tile::Machine => {
                    board.guard.direction = board.guard.next_direction();
                }
            }
        } else {
            break;
        }
    }

    Some(visited_pos)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not loaded");
    let board = parse_input(input);
    let visited_coords = simulate_walk(&board).unwrap();
    let result_1 = visited_coords.len();
    println!("{}", result_1);

    // Observation: an obstacle can be placed only on the walked path. Let's try walking all paths
    let result_2 = visited_coords
        .iter()
        .filter(|&&coords| {
            let mut board_aux = board.clone();
            board_aux.tiles[coords.1][coords.0] = Tile::Machine;
            match simulate_walk(&board_aux) {
                Some(_) => false,
                None => true,
            }
        })
        .count();

    println!("{}", result_2);
}
