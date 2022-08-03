fn search<T>(board: &mut Vec<Vec<char>>, r: usize, c: usize, mut left: T) -> bool
    where T: Iterator<Item = char> + Clone {
    let next_char;
    if let Some(next) = left.next() {
        next_char = next;
    } else {
        return true;
    }

    if let Some(row) = board.get(r-1) {
        if row[c] == next_char {
            board[r-1][c] = ' ';
            if search(board, r-1, c, left.clone()) {
                return true;
            }
            board[r-1][c] = next_char;
        }
    }

    if let Some(row) = board.get(r+1) {
        if row[c] == next_char {
            board[r+1][c] = ' ';
            if search(board, r+1, c, left.clone()) {
                return true;
            }
            board[r+1][c] = next_char;
        }
    }

    if let Some(&character) = board[r].get(c-1) {
        if character == next_char {
            board[r][c-1] = ' ';
            if search(board, r, c-1, left.clone()) {
                return true;
            }
            board[r][c-1] = next_char;
        }
    }


    if let Some(&character) = board[r].get(c+1) {
        if character == next_char {
            board[r][c+1] = ' ';
            if search(board, r, c+1, left.clone()) {
                return true;
            }
            board[r][c+1] = next_char;
        }
    }

    false
}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let (m, n) = (board.len(), board[0].len());
        for r in 0..m{
            for c in 0..n {
                if word.starts_with(board[r][c]) {
                    let temp = board[r][c];
                    board[r][c] = ' ';
                    if search(&mut board, r, c, word.chars().skip(1)) {
                        return true;
                    }
                    board[r][c] = temp;
                }
            }
        }
        false
    }
}