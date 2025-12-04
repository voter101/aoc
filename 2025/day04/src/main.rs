use std::fs;

#[derive(Debug)]
struct Board {
    fields: Vec<Vec<bool>>,
    max_x: usize,
    max_y: usize,
    removed_rolls: usize,
}

impl Board {
    fn accessible_rolls(&self) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = vec![];
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.fields[y][x] && self.neighbouring_rolls(x, y) < 4 {
                    res.push((x, y));
                }
            }
        }

        res
    }

    fn neighbouring_rolls(&self, x: usize, y: usize) -> usize {
        let x_i = x as isize;
        let y_i = y as isize;

        [
            (x_i - 1, y_i - 1),
            (x_i - 1, y_i),
            (x_i - 1, y_i + 1),
            (x_i, y_i - 1),
            (x_i, y_i + 1),
            (x_i + 1, y_i - 1),
            (x_i + 1, y_i),
            (x_i + 1, y_i + 1),
        ]
        .iter()
        .filter(|(x, y)| *x >= 0 && *x < self.max_x as isize && *y >= 0 && *y < self.max_y as isize)
        .filter(|(x, y)| self.fields[*y as usize][*x as usize])
        .count()
    }

    fn remove_accessible_rolls(&mut self) {
        for (x, y) in self.accessible_rolls() {
            self.fields[y][x] = false;
            self.removed_rolls += 1;
        }
    }
}

fn parse_input() -> Board {
    // let input = fs::read_to_string("example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let max_y = lines.clone().count();
    let max_x = lines.clone().next().unwrap().len();

    let mut res = vec![vec![false; max_x]; max_y];

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                res[y][x] = true;
            }
        }
    }

    Board {
        fields: res,
        max_x,
        max_y,
        removed_rolls: 0,
    }
}

fn accessible_rolls_count(board: &Board) -> usize {
    let mut res = 0;
    for y in 0..board.max_y {
        for x in 0..board.max_x {
            if board.fields[y][x] && board.neighbouring_rolls(x, y) < 4 {
                res += 1;
            }
        }
    }

    res
}

fn main() {
    let mut board = parse_input();
    let res_1 = board.accessible_rolls().iter().count();

    println!("{}", res_1);

    while board.accessible_rolls().len() > 0 {
        board.remove_accessible_rolls();
    }

    println!("{}", board.removed_rolls);
}
