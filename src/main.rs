use crate::store::engines::iter_one::BetterBot;
use crate::store::engines::random_moves::RandomMoves;
use crate::store::play;
use crate::store::Board;
mod store;

fn main() {
    let mover: RandomMoves = RandomMoves::new();
    let mut board: Board = Board::default();
    match play(mover, &mut board, false) {
        Err(..) => eprintln!("you entered the wrong thing fool"),
        Ok(()) => (),
    }
}
