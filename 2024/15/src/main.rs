use std::fs;

type Coords = (usize, usize);

#[derive(Eq, PartialEq)]
enum Field {
    Wall,
    Item,
    Empty,
    ItemBigLeft,
    ItemBigRight,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Map {
    robot: Coords,
    fields: Vec<Vec<Field>>,
}

impl Map {
    fn move_robot(&mut self, direction: Direction) {
        let pos = self.robot;
        let next_field: Coords = match direction {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
        };

        if self.can_push(next_field, direction) {
            self.push(next_field, direction);
            self.fields[pos.1][pos.0] = Field::Empty;
            self.fields[next_field.1][next_field.0] = Field::Empty;
            self.robot = next_field;
        }
    }

    fn push(&mut self, pos: Coords, direction: Direction) {
        if self.fields[pos.1][pos.0] == Field::Item {
            let next_field: Coords = match direction {
                Direction::Up => (pos.0, pos.1 - 1),
                Direction::Right => (pos.0 + 1, pos.1),
                Direction::Down => (pos.0, pos.1 + 1),
                Direction::Left => (pos.0 - 1, pos.1),
            };

            self.push(next_field, direction);
            self.fields[next_field.1][next_field.0] = Field::Item;
            self.fields[pos.1][pos.0] = Field::Empty;
        }

        if self.fields[pos.1][pos.0] == Field::ItemBigLeft {
            match direction {
                Direction::Up => {
                    self.push((pos.0, pos.1 - 1), direction);
                    self.push((pos.0 + 1, pos.1 - 1), direction);
                    self.fields[pos.1 - 1][pos.0] = Field::ItemBigLeft;
                    self.fields[pos.1 - 1][pos.0 + 1] = Field::ItemBigRight;
                    self.fields[pos.1][pos.0] = Field::Empty;
                    self.fields[pos.1][pos.0 + 1] = Field::Empty;
                }
                Direction::Right => {
                    self.push((pos.0 + 2, pos.1), direction);
                    self.fields[pos.1][pos.0 + 1] = Field::ItemBigLeft;
                    self.fields[pos.1][pos.0 + 2] = Field::ItemBigRight;
                    self.fields[pos.1][pos.0] = Field::Empty;
                }
                Direction::Down => {
                    self.push((pos.0, pos.1 + 1), direction);
                    self.push((pos.0 + 1, pos.1 + 1), direction);
                    self.fields[pos.1 + 1][pos.0] = Field::ItemBigLeft;
                    self.fields[pos.1 + 1][pos.0 + 1] = Field::ItemBigRight;
                    self.fields[pos.1][pos.0] = Field::Empty;
                    self.fields[pos.1][pos.0 + 1] = Field::Empty;
                }
                Direction::Left => {
                    unreachable!("Cannot hit left wall from the left")
                }
            }
        } else if self.fields[pos.1][pos.0] == Field::ItemBigRight {
            match direction {
                Direction::Up => {
                    self.push((pos.0, pos.1 - 1), direction);
                    self.push((pos.0 - 1, pos.1 - 1), direction);
                    self.fields[pos.1 - 1][pos.0] = Field::ItemBigRight;
                    self.fields[pos.1 - 1][pos.0 - 1] = Field::ItemBigLeft;
                    self.fields[pos.1][pos.0] = Field::Empty;
                    self.fields[pos.1][pos.0 - 1] = Field::Empty;
                }
                Direction::Right => {
                    unreachable!("Cannot hit right, from right");
                }
                Direction::Down => {
                    self.push((pos.0, pos.1 + 1), direction);
                    self.push((pos.0 - 1, pos.1 + 1), direction);
                    self.fields[pos.1 + 1][pos.0] = Field::ItemBigRight;
                    self.fields[pos.1 + 1][pos.0 - 1] = Field::ItemBigLeft;
                    self.fields[pos.1][pos.0] = Field::Empty;
                    self.fields[pos.1][pos.0 - 1] = Field::Empty;
                }
                Direction::Left => {
                    self.push((pos.0 - 2, pos.1), direction);
                    self.fields[pos.1][pos.0 - 1] = Field::ItemBigRight;
                    self.fields[pos.1][pos.0 - 2] = Field::ItemBigLeft;
                    self.fields[pos.1][pos.0] = Field::Empty;
                }
            }
        }
    }

    fn can_push(&self, coords: Coords, direction: Direction) -> bool {
        match direction {
            Direction::Up => {
                match self.fields[coords.1][coords.0] {
                    Field::Wall => return false,
                    Field::Item => return self.can_push((coords.0, coords.1 - 1), direction),
                    Field::Empty => return true,
                    Field::ItemBigLeft => {
                        return self.can_push((coords.0, coords.1 - 1), direction)
                            && self.can_push((coords.0 + 1, coords.1 - 1), direction);
                    }
                    Field::ItemBigRight => {
                        return self.can_push((coords.0, coords.1 - 1), direction)
                            && self.can_push((coords.0 - 1, coords.1 - 1), direction);
                    }
                };
            }
            Direction::Down => {
                match self.fields[coords.1][coords.0] {
                    Field::Wall => return false,
                    Field::Item => return self.can_push((coords.0, coords.1 + 1), direction),
                    Field::Empty => return true,
                    Field::ItemBigLeft => {
                        return self.can_push((coords.0, coords.1 + 1), direction)
                            && self.can_push((coords.0 + 1, coords.1 + 1), direction);
                    }
                    Field::ItemBigRight => {
                        return self.can_push((coords.0, coords.1 + 1), direction)
                            && self.can_push((coords.0 - 1, coords.1 + 1), direction);
                    }
                };
            }
            Direction::Left => {
                match self.fields[coords.1][coords.0] {
                    Field::Wall => return false,
                    Field::Item => return self.can_push((coords.0 - 1, coords.1), direction),
                    Field::Empty => return true,
                    Field::ItemBigLeft => {
                        return self.can_push((coords.0 - 1, coords.1), direction)
                    }
                    Field::ItemBigRight => {
                        return self.can_push((coords.0 - 2, coords.1), direction)
                    }
                };
            }
            Direction::Right => {
                match self.fields[coords.1][coords.0] {
                    Field::Wall => return false,
                    Field::Item => return self.can_push((coords.0 + 1, coords.1), direction),
                    Field::Empty => return true,
                    Field::ItemBigLeft => {
                        return self.can_push((coords.0 + 2, coords.1), direction)
                    }
                    Field::ItemBigRight => {
                        return self.can_push((coords.0 + 1, coords.1), direction)
                    }
                };
            }
        }
    }
}

fn parse_input(input: String) -> (Map, Map, Vec<Direction>) {
    let (input_map, input_directions) = input.split_once("\n\n").unwrap();
    (
        parse_map(input_map),
        parse_map_wide(input_map),
        parse_directions(input_directions),
    )
}

fn parse_map(input: &str) -> Map {
    let mut robot: (usize, usize) = (0, 0);
    let fields = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Field::Wall,
                    'O' => Field::Item,
                    '.' => Field::Empty,
                    '@' => {
                        robot = (x, y);
                        Field::Empty
                    }
                    _ => panic!("Invalid field"),
                })
                .collect()
        })
        .collect();

    Map { robot, fields }
}

fn parse_map_wide(input: &str) -> Map {
    let mut robot: (usize, usize) = (0, 0);
    let fields = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(x, c)| match c {
                    '#' => [Field::Wall, Field::Wall],
                    'O' => [Field::ItemBigLeft, Field::ItemBigRight],
                    '.' => [Field::Empty, Field::Empty],
                    '@' => {
                        robot = (2 * x, y);
                        [Field::Empty, Field::Empty]
                    }
                    _ => panic!("Invalid field"),
                })
                .collect()
        })
        .collect();

    Map { robot, fields }
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => unreachable!("Invalid direction"),
                })
                .collect::<Vec<Direction>>()
        })
        .collect()
}

fn map_score(map: &Map) -> usize {
    map.fields
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, field)| match field {
                    Field::Item => 100 * y + x,
                    Field::ItemBigLeft => 100 * y + x,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let (mut map, mut wide_map, directions) = parse_input(input);

    directions.iter().for_each(|d| map.move_robot(*d));
    println!("{:?}", map_score(&map));

    directions.iter().for_each(|d| wide_map.move_robot(*d));
    println!("{:?}", map_score(&wide_map));
}
