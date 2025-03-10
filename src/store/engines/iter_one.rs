use crate::store::{sort_board, who_won, Board, MakeMove, Player, Square};
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
// try to find if the opponent has a winning move and if so then we try to get in
// the way of it. Defensive bot? It'll have anything going for it now.
// not doing any spinning yet I don't know if we will at all for that matter
impl MakeMove for BetterBot {
    fn moves(board: &Board) -> usize {
        // nobody has won yet, it's our turn to play.
        // We take the board and for every square check if they place there
        // then do they win? if yes then we return that square as we want to block

        for i in board.pieces.iter().filter(|x| x.player.is_none()) {
            let mut copy = Board::default();
            for (j, refe) in copy.pieces.iter_mut().enumerate() {
                *refe = board.pieces[j];
            }

            copy.pieces[i.spot - 1] = Square::new(i.spot, Some(Player::O));

            match who_won(&copy) {
                Some(_) => return i.spot,
                None => continue,
            }
        }
        let mut rng = rand::rng();
        let mut num = rng.random_range(1..=9);
        loop {
            if board.pieces[num - 1].player.is_some() {
                num = rng.random_range(1..=9);
            } else {
                break num;
            }
        }
    }
}
