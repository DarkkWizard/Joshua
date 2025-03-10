use crate::store::engines::iter_one::BetterBot;
use crate::store::engines::random_moves::RandomMoves;
use crate::store::{play, Board};
use clap::{arg, ArgAction, Parser, ValueEnum};
mod store;

#[derive(Debug, Clone, ValueEnum)]
enum Bots {
    BetterBot,
    RandomMoves,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, action = ArgAction::SetTrue, default_value_t = false)]
    player: bool,
    #[arg(short, long, value_parser, default_value = "BetterBot")]
    bot: Bots,
}

fn main() {
    let args = Args::parse();
    let mut board: Board = Board::default();

    if args.player {
        match args.bot {
            Bots::BetterBot => {
                let bot_gen: BetterBot = BetterBot::new();
                match play(bot_gen, &mut board, true) {
                    Err(..) => eprintln!("you entered the wrong thing fool"),
                    Ok(()) => (),
                }
            }
            Bots::RandomMoves => {
                let bot_gen: RandomMoves = RandomMoves::new();
                match play(bot_gen, &mut board, true) {
                    Err(..) => eprintln!("you entered the wrong thing fool"),
                    Ok(()) => (),
                }
            }
        }
    } else {
        match args.bot {
            Bots::BetterBot => {
                let bot_gen: BetterBot = BetterBot::new();
                match play(bot_gen, &mut board, false) {
                    Err(..) => eprintln!("you entered the wrong thing fool"),
                    Ok(()) => (),
                }
            }
            Bots::RandomMoves => {
                let bot_gen: RandomMoves = RandomMoves::new();
                match play(bot_gen, &mut board, false) {
                    Err(..) => eprintln!("you entered the wrong thing fool"),
                    Ok(()) => (),
                }
            }
        }
    }
}
