use crate::store::{sort_board, Board, MakeMove};
use rand::Rng;

pub struct BetterBot {}

impl BetterBot {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BetterBot {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unreachable_code)]
impl MakeMove for BetterBot {
    fn moves(board: &Board) -> usize {
        let mut rng = rand::rng();
        let mut num = rng.random_range(1..10);

        loop {
            if board.pieces[num - 1].player == None {
                return num;
            } else {
                num = rng.random_range(1..10);
            }
        }
        unreachable!("never chose a square")
    }
}
