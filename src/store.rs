use rand::Rng;
use std::error::Error;
pub mod engines;

const LOSER_MESSAGES: [&str; 11] = [
    "It's THREE in a row",
    "Suck my orbs",
    "You lost again. Have you considered blocking?",
    "Suck my tic-tac-toes",
    "OOOOOOOOOOOOOO you lost sucker",
    "You lost the game",
    "dig a hole",
    "to the gallows with you",
    "Consider other options",
    "Womp Womp",
    "Well played... by me!",
];
const WINNER_MESSAGES: [&str; 3] = [
    "This was unforseen",
    "ow :(",
    "You win! Yay! You've beaten technology and saved the world!",
];
const TIE_MESSAGES: [&str; 4] = [
    "We've acheived peak mediocrity",
    "I'd say we've mastered the art of mutual... non-winning. Shall we try again and aim for slighlty less impressive results?",
    "I'm not sure if I should congradulate you or myself on our shared talent for avoiding victory.",
    "reminicent of a beige wall"
];

pub fn play<H: MakeMove>(
    _mover: H,
    board: &mut Board,
    player_first: bool,
) -> Result<(), Box<dyn Error>> {
    use std::io::stdin;

    println!("Enter the square you would like to capture: \n 1 | 2 | 3 \n ---------- \n 4 | 5 | 6 \n ---------- \n 7 | 8 | 9 \n\n\n");

    print_board(&board);

    board.pieces = sort_board(&board);

    let mut winner = false;

    if player_first {
        while !winner {
            let f: usize;
            loop {
                let mut s = String::new();
                stdin().read_line(&mut s)?;
                let n: usize = s.trim().parse()?;
                if board.pieces[n - 1].player.is_some() {
                    println!("Please choose a square that is not already filled");
                    continue;
                }
                f = n;
                break;
            }
            match f {
                1 => board.pieces[f - 1].player = Some(Player::O),
                2 => board.pieces[f - 1].player = Some(Player::O),
                3 => board.pieces[f - 1].player = Some(Player::O),
                4 => board.pieces[f - 1].player = Some(Player::O),
                5 => board.pieces[f - 1].player = Some(Player::O),
                6 => board.pieces[f - 1].player = Some(Player::O),
                7 => board.pieces[f - 1].player = Some(Player::O),
                8 => board.pieces[f - 1].player = Some(Player::O),
                9 => board.pieces[f - 1].player = Some(Player::O),
                _ => panic!("provide a square that is on the board fool."),
            }

            match who_won(&board) {
                Some(Player::X) => {
                    loser();
                    print_board(&board);
                    winner = true;
                    continue;
                }
                Some(Player::O) => {
                    tryhard();
                    print_board(&board);
                    winner = true;
                    continue;
                }
                Some(Player::Both) => {
                    second_loser();
                    print_board(&board);
                    winner = true;
                    continue;
                }
                None => (),
            }

            board.pieces[H::moves(board) - 1].player = Some(Player::X);
            print_board(&board);

            match who_won(&board) {
                Some(Player::X) => {
                    loser();
                    print_board(&board);
                    winner = true;
                }
                Some(Player::O) => {
                    tryhard();
                    print_board(&board);
                    winner = true;
                }
                Some(Player::Both) => {
                    second_loser();
                    print_board(&board);
                    winner = true;
                }
                None => (),
            }
        }
    } else {
        while !winner {
            board.pieces[H::moves(board) - 1].player = Some(Player::X);
            print_board(&board);

            match who_won(&board) {
                Some(Player::X) => {
                    loser();
                    print_board(&board);
                    winner = true;
                }
                Some(Player::O) => {
                    tryhard();
                    print_board(&board);
                    winner = true;
                }
                Some(Player::Both) => {
                    second_loser();
                    print_board(&board);
                    winner = true;
                }
                None => (),
            }

            if winner == true {
                break;
            }
            let f: usize;
            loop {
                let mut s = String::new();
                stdin().read_line(&mut s)?;
                let n: usize = s.trim().parse()?;
                if board.pieces[n - 1].player.is_some() {
                    println!("Please choose a square that is not already filled");
                    continue;
                }
                f = n;
                break;
            }
            match f {
                1 => board.pieces[f - 1].player = Some(Player::O),
                2 => board.pieces[f - 1].player = Some(Player::O),
                3 => board.pieces[f - 1].player = Some(Player::O),
                4 => board.pieces[f - 1].player = Some(Player::O),
                5 => board.pieces[f - 1].player = Some(Player::O),
                6 => board.pieces[f - 1].player = Some(Player::O),
                7 => board.pieces[f - 1].player = Some(Player::O),
                8 => board.pieces[f - 1].player = Some(Player::O),
                9 => board.pieces[f - 1].player = Some(Player::O),
                _ => panic!("provide a square that is on the board fool."),
            }

            match who_won(&board) {
                Some(Player::X) => {
                    loser();
                    print_board(&board);
                    winner = true;
                }
                Some(Player::O) => {
                    tryhard();
                    print_board(&board);
                    winner = true;
                }
                Some(Player::Both) => {
                    second_loser();
                    print_board(&board);
                    winner = true;
                }
                None => (),
            }
        }
    }
    Ok(())
}

fn tryhard() {
    let mut rng = rand::rng();
    let num = rng.random_range(0..3);

    let messages = WINNER_MESSAGES[num];
    println!("{messages}");
}

fn loser() {
    let mut rng = rand::rng();
    let num = rng.random_range(0..11);

    let messages = LOSER_MESSAGES[num];
    println!("{messages}");
}

fn second_loser() {
    let mut rng = rand::rng();
    let num = rng.random_range(0..4);

    let messages = TIE_MESSAGES[num];
    println!("{messages}");
}

fn check3(first: Square, second: Square, third: Square) -> Option<Player> {
    match (first, second, third) {
        (first, second, third)
            if first.player == second.player && second.player == third.player =>
        {
            first.player
        }
        _ => None,
    }
}

pub fn who_won(board: &Board) -> Option<Player> {
    let mut holder: Vec<Option<Player>> = vec![];

    holder.push(check3(board.pieces[0], board.pieces[1], board.pieces[2]));
    holder.push(check3(board.pieces[3], board.pieces[4], board.pieces[5]));
    holder.push(check3(board.pieces[6], board.pieces[7], board.pieces[8]));
    holder.push(check3(board.pieces[0], board.pieces[3], board.pieces[6]));
    holder.push(check3(board.pieces[1], board.pieces[4], board.pieces[7]));
    holder.push(check3(board.pieces[2], board.pieces[5], board.pieces[8]));
    holder.push(check3(board.pieces[0], board.pieces[4], board.pieces[8]));
    holder.push(check3(board.pieces[2], board.pieces[4], board.pieces[6]));

    for item in holder.iter().filter(|&x| x.is_some()) {
        match item {
            Some(Player::X) => return Some(Player::X),
            Some(Player::O) => return Some(Player::O),
            Some(Player::Both) => unreachable!(),
            None => return None,
        }
    }
    if board.pieces.iter().all(|&y| y.player.is_some()) {
        return Some(Player::Both);
    }
    None
}

pub fn print_board(rock: &Board) {
    let mut num: usize = 1;
    for i in rock.pieces.iter() {
        if num == 3 {
            println!("{} ", i.display());
            num = 1;
        } else {
            print!("{} ", i.display());
            num += 1;
        }
    }
    println!("\n");
}

pub fn sort_board(board: &Board) -> [Square; 9] {
    let mut ret: [Square; 9] = [Square::default(); 9];

    for &i in board.pieces.iter() {
        ret[i.spot - 1] = i;
    }

    ret
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Player {
    X,
    O,
    Both,
}

pub trait MakeMove {
    fn moves(board: &Board) -> usize;
}

// so that we can rotate the board when doing move analysis
// 1 | 2 | 3
//----------
// 4 | 5 | 6
//-----------
// 7 | 8 | 9

#[derive(Copy, Clone, Debug)]
pub struct Square {
    spot: usize,
    player: Option<Player>,
}

impl Default for Square {
    fn default() -> Square {
        Square {
            spot: 0,
            player: None,
        }
    }
}

impl Square {
    pub fn new(spot: usize, player: Option<Player>) -> Square {
        Square { spot, player }
    }

    fn new_board() -> [Square; 9] {
        let mut ret: [Square; 9] = [Square::default(); 9];

        for (place, spot) in ret.iter_mut().enumerate() {
            *spot = Square::new(place, Some(Player::O));
        }
        ret
    }

    pub fn display(&self) -> char {
        match self.player {
            Some(Player::O) => 'O',
            Some(Player::X) => 'X',
            None => '+',
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub pieces: [Square; 9],
}

impl Board {
    pub fn new(pieces: [Square; 9]) -> Self {
        Self { pieces }
    }

    pub fn spin_clockwise(&self) -> Board {
        let mut new_board = Board::default();

        for square in &self.pieces {
            match square.spot {
                1 => new_board.pieces[2] = self.pieces[0],
                2 => new_board.pieces[5] = self.pieces[1],
                3 => new_board.pieces[8] = self.pieces[2],
                4 => new_board.pieces[1] = self.pieces[3],
                5 => new_board.pieces[4] = self.pieces[4],
                6 => new_board.pieces[7] = self.pieces[5],
                7 => new_board.pieces[0] = self.pieces[6],
                8 => new_board.pieces[3] = self.pieces[7],
                9 => new_board.pieces[6] = self.pieces[8],
                _ => println!("what the fuck happened we have impossible indexes?"),
            }
        }
        new_board
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new([
            Square::new(1, None),
            Square::new(2, None),
            Square::new(3, None),
            Square::new(4, None),
            Square::new(5, None),
            Square::new(6, None),
            Square::new(7, None),
            Square::new(8, None),
            Square::new(9, None),
        ])
    }
}
