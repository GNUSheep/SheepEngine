use chess::{Board, MoveGen, ChessMove, Color};
use std::str::FromStr;
use rand::Rng;

static CHESS_PIECES_VALUE: [i32; 5] = [100, 300, 300, 500, 900];

fn search(depth: i32, fen: &str, board: Board) -> (i32, String) {
    if depth == 0 {
        return (evaluate_position(fen, board), "".to_string())
    }

    let moves = MoveGen::new_legal(&board);

    let mut best = -100000000;
    let mut best_move = "".to_string();

    for move_element in moves {
        let new_board = board.make_move_new(move_element);
        let (out, _) = search(depth - 1, new_board.to_string().as_str(), new_board);
        if best <= -out {
            best = -out;
            best_move = move_element.to_string(); 
        }
    }

    return (best, best_move);
}

fn evaluate_position(fen: &str, board: Board) -> i32 {
    let fen_splited = fen.split(" ").collect::<Vec<_>>()[0].split("/").collect::<Vec<_>>();

    let mut black_evaluation = 0;
    let mut white_evaluation = 0;
    for row in fen_splited.iter() {
        for piece in row.chars() {
            match piece {
                'p' => {
                    black_evaluation += CHESS_PIECES_VALUE[0];           
                },
                'P' => {
                    white_evaluation += CHESS_PIECES_VALUE[0];  
                }
                'n' => {
                    black_evaluation += CHESS_PIECES_VALUE[1];           
                },
                'N' => {
                    white_evaluation += CHESS_PIECES_VALUE[1];  
                }
                'b' => {
                    black_evaluation += CHESS_PIECES_VALUE[2];           
                },
                'B' => {
                    white_evaluation += CHESS_PIECES_VALUE[2];  
                }
                'r' => {
                    black_evaluation += CHESS_PIECES_VALUE[3];           
                },
                'R' => {
                    white_evaluation += CHESS_PIECES_VALUE[3];  
                }
                'q' => {
                    black_evaluation += CHESS_PIECES_VALUE[4];           
                },
                'Q' => {
                    white_evaluation += CHESS_PIECES_VALUE[4];  
                }
                _ => (),
            }
        }
    }
    if board.side_to_move() == Color::Black {
        return -1 * (white_evaluation - black_evaluation)
    }else {
        return white_evaluation - black_evaluation
    }
}

pub fn make_move(fen: &str) -> String {
    let chess_board = Board::from_str(fen).unwrap();

    let (out, best_move) = search(3, fen, chess_board);
    println!("{}, {}", out, best_move);

    return best_move
}