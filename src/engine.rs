use chess::{Board, MoveGen};
use std::str::FromStr;
use rand::Rng;

pub fn make_move(fen: &str) -> String {
    let chess_board = Board::from_str(fen).unwrap();

    let moves = MoveGen::new_legal(&chess_board).collect::<Vec<_>>();

    let mut rng = rand::thread_rng();

    moves[rng.gen_range(0..moves.len())].to_string()
}