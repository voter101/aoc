use std::fs;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Add,
    Mul,
}

#[derive(Debug)]
struct Equation {
    ingredients: Vec<usize>,
    operand: Operand,
}

impl Equation {
    fn execute(&self) -> usize {
        match self.operand {
            Operand::Add => self.ingredients.iter().sum::<usize>(),
            Operand::Mul => self.ingredients.iter().product::<usize>(),
        }
    }
}

fn parse_input_1(file: &String) -> Vec<Equation> {
    let mut res = vec![];

    let ingredients_count = file.lines().count() - 1;

    let mut ingredient_rows: Vec<Vec<usize>> = vec![];
    let mut operands: Vec<Operand> = vec![];

    for (i, line) in file.lines().enumerate() {
        if i < ingredients_count {
            ingredient_rows.push(vec![]);
            let mut curr: Vec<char> = vec![];
            for c in line.chars() {
                if c >= '0' && c <= '9' {
                    curr.push(c);
                } else if !curr.is_empty() {
                    ingredient_rows[i]
                        .push(curr.iter().collect::<String>().parse::<usize>().unwrap());
                    curr = vec![];
                }
            }
            ingredient_rows[i].push(curr.iter().collect::<String>().parse::<usize>().unwrap());
        } else {
            for c in line.chars() {
                if c == '+' {
                    operands.push(Operand::Add);
                } else if c == '*' {
                    operands.push(Operand::Mul);
                }
            }
        }
    }

    let equations_count = operands.len();

    for i in 0..equations_count {
        res.push(Equation {
            ingredients: ingredient_rows.iter().map(|row| row[i]).collect(),
            operand: operands[i],
        })
    }

    res
}

fn parse_input_2(file: &String) -> Vec<Equation> {
    let mut res = vec![];

    let mut operands: Vec<(Operand, usize, usize)> = vec![];

    let operand_line = file.lines().last().unwrap();
    let mut curr: (Operand, usize) = (parse_operand(operand_line.chars().next().unwrap()), 0);
    for (i, c) in operand_line.chars().enumerate() {
        if i == 0 {
            continue;
        }
        if c == '*' || c == '+' {
            operands.push((curr.0, curr.1, i - curr.1 - 1));
            curr = (parse_operand(c), i);
        }
    }
    operands.push((
        curr.0,
        curr.1,
        file.lines().map(|l| l.len()).max().unwrap() - curr.1,
    ));

    let board: Vec<Vec<char>> = file
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .skip(1)
        .rev()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    for (operand, start, len) in operands {
        let ingredients = (start..(start + len))
            .map(|i| {
                let mut ingredient: Vec<char> = vec![];
                for line in board.iter() {
                    if i < line.len() {
                        ingredient.push(line[i]);
                    }
                }
                ingredient
                    .into_iter()
                    .filter(|c| *c >= '0' && *c <= '9')
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();
        res.push(Equation {
            ingredients,
            operand,
        })
    }

    res
}

fn parse_operand(o: char) -> Operand {
    match o {
        '+' => Operand::Add,
        '*' => Operand::Mul,
        _ => unreachable!(),
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let operands_1 = parse_input_1(&file);
    let part_1 = operands_1.iter().map(|o| o.execute()).sum::<usize>();

    println!("{:?}", part_1);

    let operands_2 = parse_input_2(&file);
    let part_2 = operands_2.iter().map(|o| o.execute()).sum::<usize>();
    println!("{:?}", part_2);
}
