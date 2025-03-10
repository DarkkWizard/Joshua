use crate::store::{Board, MakeMove};
use rand::Rng;

pub struct RandomMoves {}

impl RandomMoves {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RandomMoves {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unreachable_code)]
impl MakeMove for RandomMoves {
    fn moves(board: &Board) -> usize {
        loop {
            let mut rng = rand::rng();
            let num = rng.random_range(1..=9);

            match board.pieces[num - 1].player {
                Some(_) => continue,
                None => {
                    break num;
                }
            }
        }
    }
}
