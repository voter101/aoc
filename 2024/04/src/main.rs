use std::fs;

type Board = Vec<Vec<char>>;

fn count_words(word: &str, board: &Board) -> usize {
    let word_chars: Vec<char> = word.chars().collect();
    let rows = board.len();
    let cols = board[0].len();

    let mut res = 0;

    for r in 0..rows {
        for c in 0..cols {
            if board[r][c] == word_chars[0] {
                res += horizontals(&board, &word_chars, r, c);
                res += verticals(&board, &word_chars, r, c);
                res += diagonals(&board, &word_chars, r, c);
            }
        }
    }

    res
}

fn horizontals(board: &Board, word: &Vec<char>, row: usize, col: usize) -> usize {
    let mut res = 0;
    // L -> R
    if col + (word.len() - 1) < board[0].len() {
        if (1..word.len()).all(|i| board[row][col + i] == word[i]) {
            res += 1;
        }
    }

    // R -> L
    if col >= (word.len() - 1) {
        if (1..word.len()).all(|i| board[row][col - i] == word[i]) {
            res += 1;
        }
    }

    res
}

fn verticals(board: &Board, word: &Vec<char>, row: usize, col: usize) -> usize {
    let mut res = 0;
    // U -> D
    if row + (word.len() - 1) < board.len() {
        if (1..word.len()).all(|i| board[row + i][col] == word[i]) {
            res += 1;
        }
    }
    // D -> U
    if row >= (word.len() - 1) {
        if (1..word.len()).all(|i| board[row - i][col] == word[i]) {
            res += 1;
        }
    }

    res
}

fn diagonals(board: &Board, word: &Vec<char>, row: usize, col: usize) -> usize {
    let mut res = 0;
    // LU -> RD
    if row + (word.len() - 1) < board.len() && col + (word.len() - 1) < board[0].len() {
        if (1..word.len()).all(|i| board[row + i][col + i] == word[i]) {
            res += 1;
        }
    }
    // LD -> RU
    if col + (word.len() - 1) < board[0].len() && row >= (word.len() - 1) {
        if (1..word.len()).all(|i| board[row - i][col + i] == word[i]) {
            res += 1;
        }
    }
    // RD -> LU
    if col >= (word.len() - 1) && row >= (word.len() - 1) {
        if (1..word.len()).all(|i| board[row - i][col - i] == word[i]) {
            res += 1;
        }
    }

    // RU -> LD
    if col >= (word.len() - 1) && row + (word.len() - 1) < board.len() {
        if (1..word.len()).all(|i| board[row + i][col - i] == word[i]) {
            res += 1;
        }
    }

    res
}

fn count_crosses(board: &Board) -> usize {
    let rows = board.len();
    let cols = board[0].len();

    let mut res = 0;

    for r in 1..(rows - 1) {
        for c in 1..(cols - 1) {
            if board[r][c] == 'A' {
                // Diagonals are different characters
                if board[r - 1][c - 1] != board[r + 1][c + 1]
                    && board[r - 1][c + 1] != board[r + 1][c - 1]
                {
                    // Check if there are only 2 M and S characters
                    if [
                        board[r - 1][c - 1],
                        board[r - 1][c + 1],
                        board[r + 1][c - 1],
                        board[r + 1][c + 1],
                    ]
                    .iter()
                    .fold((0, 0), |(m, s), &c| {
                        (
                            if c == 'M' { m + 1 } else { m },
                            if c == 'S' { s + 1 } else { s },
                        )
                    }) == (2, 2)
                    {
                        res += 1;
                    }
                }
            }
        }
    }

    res
}

fn main() {
    let input: Board = fs::read_to_string("./input.txt")
        .expect("File not loaded")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let res_1 = count_words("XMAS", &input);
    println!("{}", res_1);

    let res_2 = count_crosses(&input);
    println!("{}", res_2);
}
