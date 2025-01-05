use std::collections::HashMap;
use std::fs;

type Coords = (isize, isize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum KeypadInput {
    Access,
    Direction(Direction),
    Number(usize),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type InputMap = HashMap<(KeypadInput, KeypadInput), Vec<KeypadInput>>;

fn create_graph(key_positions: HashMap<KeypadInput, Coords>, invalid_coords: Coords) -> InputMap {
    let mut res = HashMap::new();

    for (a, (x1, y1)) in key_positions.clone().iter() {
        for (b, (x2, y2)) in key_positions.iter() {
            let up = if x1 >= x2 {
                vec![KeypadInput::Direction(Direction::Up); (x1 - x2) as usize]
            } else {
                vec![]
            };
            let right = if y2 >= y1 {
                vec![KeypadInput::Direction(Direction::Right); (y2 - y1) as usize]
            } else {
                vec![]
            };
            let down = if x2 >= x1 {
                vec![KeypadInput::Direction(Direction::Down); (x2 - x1) as usize]
            } else {
                vec![]
            };
            let left = if y1 >= y2 {
                vec![KeypadInput::Direction(Direction::Left); (y1 - y2) as usize]
            } else {
                vec![]
            };
            let mut path: Vec<KeypadInput> = [left, down, up, right].concat();

            if invalid_coords == (*x1, *y2) || invalid_coords == (*x2, *y1) {
                path.reverse();
            }
            path.push(KeypadInput::Access);
            res.insert((*a, *b), path);
        }
    }

    res
}

fn robots_instructions_count(
    input: Vec<KeypadInput>,
    iterations: usize,
    (num_graph, dir_graph): (&InputMap, &InputMap),
    cache: &mut HashMap<(Vec<KeypadInput>, usize), usize>,
    is_first: bool,
) -> usize {
    if iterations == 0 {
        return input.len();
    }

    if let Some(res) = cache.get(&(input.clone(), iterations)) {
        return res.clone();
    }

    let mut prev = KeypadInput::Access;
    let mut res: usize = 0;

    for c in input.clone() {
        let curr_graph = if is_first { num_graph } else { dir_graph };
        res += robots_instructions_count(
            curr_graph.get(&(prev, c.clone())).unwrap().clone(),
            iterations - 1,
            (&num_graph, &dir_graph),
            cache,
            false,
        );
        prev = c;
    }

    cache.insert((input, iterations), res);

    res
}

fn parse_input(input: String) -> Vec<(Vec<KeypadInput>, usize)> {
    input
        .lines()
        .map(|line| {
            let mut num = line.to_string();
            num.pop();
            (
                line.chars()
                    .map(|c| match c {
                        'A' => KeypadInput::Access,
                        n => KeypadInput::Number(n as usize - '0' as usize),
                    })
                    .collect::<Vec<KeypadInput>>(),
                num.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn main() {
    let keypad_numeric: HashMap<KeypadInput, Coords> = HashMap::from([
        (KeypadInput::Number(1), (2, 0)),
        (KeypadInput::Number(2), (2, 1)),
        (KeypadInput::Number(3), (2, 2)),
        (KeypadInput::Number(4), (1, 0)),
        (KeypadInput::Number(5), (1, 1)),
        (KeypadInput::Number(6), (1, 2)),
        (KeypadInput::Number(7), (0, 0)),
        (KeypadInput::Number(8), (0, 1)),
        (KeypadInput::Number(9), (0, 2)),
        (KeypadInput::Number(0), (3, 1)),
        (KeypadInput::Access, (3, 2)),
    ]);

    let keypad_dir: HashMap<KeypadInput, Coords> = HashMap::from([
        (KeypadInput::Direction(Direction::Up), (0, 1)),
        (KeypadInput::Direction(Direction::Right), (1, 2)),
        (KeypadInput::Direction(Direction::Down), (1, 1)),
        (KeypadInput::Direction(Direction::Left), (1, 0)),
        (KeypadInput::Access, (0, 2)),
    ]);

    let graph_numeric = create_graph(keypad_numeric, (3, 0));
    let graph_dir = create_graph(keypad_dir, (0, 0));

    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let robots = parse_input(input);

    let mut cache: HashMap<(Vec<KeypadInput>, usize), usize> = HashMap::new();

    let complexities: usize = robots
        .iter()
        .map(|(input, num)| {
            robots_instructions_count(
                input.clone(),
                3,
                (&graph_numeric, &graph_dir),
                &mut cache,
                true,
            ) * num
        })
        .sum();

    println!("{}", complexities);

    let complexities_second: usize = robots
        .iter()
        .map(|(input, num)| {
            robots_instructions_count(
                input.clone(),
                26,
                (&graph_numeric, &graph_dir),
                &mut cache,
                true,
            ) * num
        })
        .sum();
    println!("{}", complexities_second);
}
