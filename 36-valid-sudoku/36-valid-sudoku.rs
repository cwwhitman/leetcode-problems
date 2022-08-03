use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
enum Num {
    Row(u8, char),
    Col(u8, char),
    Box(u8, u8, char),
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut entries = HashSet::new();
        for (r, row) in board.into_iter().enumerate() {
            for (c, n) in row.into_iter().enumerate() {
                let (r, c) = (r as u8, c as u8);
                if n == '.' {
                    continue;
                }
    
                if entries.insert(Num::Row(r, n)) && 
                   entries.insert(Num::Col(c, n)) && 
                   entries.insert(Num::Box(r/3, c/3, n)) 
                { } else {
                    return false;
                }
            }
        }
        true
    }
}