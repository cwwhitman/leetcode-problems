use std::collections::HashMap;

fn square_game(n: i32, saved: &mut HashMap<i32, bool>) -> bool {
    if let Some(result) = saved.get(&n) {
        return *result;
    }
    
    for a in (1..(((n as f32).sqrt() as i32) + 1)).rev() {
        let turn = n - a.pow(2);
        if !square_game(turn, saved) {
            saved.insert(n, true);
            return true;
        }
    }
    
    saved.insert(n, false);
    false
}

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut saved_values = HashMap::new();
        square_game(n, &mut saved_values)
    }
}