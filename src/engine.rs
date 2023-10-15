use chess::{Board, MoveGen, Color, BoardStatus, Game, GameResult};
use std::borrow::Borrow;
use std::cmp::{min, max};
use std::str::FromStr;

static CHESS_PIECES_VALUE: [i32; 5] = [100, 320, 330, 500, 900];

static PAWN_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

static KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

static BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

static ROOK_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

static QUEEN_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

fn checkmate_check(mut alpha: f32, beta: f32, board: Board, fen: &str) -> f32 {
    let evaluation = evaluate_position(fen, board);
    if evaluation >= beta {
        return beta
    }
    if alpha < evaluation {
        alpha = evaluation;
    }

    let mut moves = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    moves.set_iterator_mask(*targets);

    for target in moves {
        let new_board = board.make_move_new(target);
        let out = -checkmate_check(-beta, -alpha, new_board, new_board.to_string().as_str());

        if beta <= out {
            return beta
        }
        if alpha < out {
            alpha = out;
        }
    }
    
    return alpha
}

fn search_root(depth: i32, board: Board) -> (f32, String) {
    let moves = MoveGen::new_legal(&board);

    let mut best_value = f32::INFINITY;
    //let copied_moves: Vec<_> = moves.cloned();
    let mut best_move = " ".to_string();

    for move_element in moves {
        let new_board = board.make_move_new(move_element);

        let out = minmax(depth - 1, new_board.to_string().as_str(), new_board, f32::NEG_INFINITY, f32::INFINITY, true);

        if out <= best_value {
            best_value = out;
            best_move = move_element.to_string();
        }
    }

    return (best_value, best_move)
}

fn minmax(depth: i32, fen: &str, board: Board, mut alpha: f32, mut beta: f32, maximizing: bool) -> f32{
    if depth == 0 {
        return checkmate_check(alpha, beta, board, fen)
    }

    let moves = MoveGen::new_legal(&board);

    if moves.len() == 0 {
        return evaluate_position(fen, board)
    }

    if maximizing {
        let mut best_value = f32::NEG_INFINITY;
        for move_element in moves {
            let new_board = board.make_move_new(move_element);

            let out = minmax(depth - 1, new_board.to_string().as_str(), new_board, alpha, beta, false);
            best_value = best_value.max(out);
            alpha = alpha.max(best_value);
            if beta <= alpha {
                return best_value
            }
        }
        return best_value
    }else {
        let mut best_value = f32::INFINITY;
        for move_element in moves {
            let new_board = board.make_move_new(move_element);

            let out = minmax(depth - 1, new_board.to_string().as_str(), new_board, alpha, beta, true);
            best_value = best_value.min(out);
            beta = beta.min(best_value);
            if beta <= alpha {
                return best_value
            }
        }
        return best_value
    }
}

fn evaluate_position(fen: &str, board: Board) -> f32 {
    let fen_splited = fen.split(" ").collect::<Vec<_>>()[0].split("/").collect::<Vec<_>>();

    let mut black_evaluation = 0;
    let mut white_evaluation = 0;

    let mut square_index = 0;
    for row in fen_splited.iter() {
        for piece in row.chars() {
            match piece {
                'p' => {
                    black_evaluation += CHESS_PIECES_VALUE[0];
                    black_evaluation += PAWN_TABLE.iter().copied().rev().collect::<Vec<_>>()[square_index];      
                },
                'P' => {
                    white_evaluation += CHESS_PIECES_VALUE[0];
                    white_evaluation += PAWN_TABLE[square_index];    
                }
                'n' => {
                    black_evaluation += CHESS_PIECES_VALUE[1]; 
                    black_evaluation += KNIGHT_TABLE.iter().copied().rev().collect::<Vec<_>>()[square_index];      
                },
                'N' => {
                    white_evaluation += CHESS_PIECES_VALUE[1];
                    white_evaluation += KNIGHT_TABLE[square_index]; 
                }
                'b' => {
                    black_evaluation += CHESS_PIECES_VALUE[2];
                    black_evaluation += BISHOP_TABLE.iter().copied().rev().collect::<Vec<_>>()[square_index];         
                },
                'B' => {
                    white_evaluation += CHESS_PIECES_VALUE[2];
                    white_evaluation += BISHOP_TABLE[square_index]; 
                }
                'r' => {
                    black_evaluation += CHESS_PIECES_VALUE[3];
                    black_evaluation += ROOK_TABLE.iter().copied().rev().collect::<Vec<_>>()[square_index];           
                },
                'R' => {
                    white_evaluation += CHESS_PIECES_VALUE[3];
                    white_evaluation += ROOK_TABLE[square_index]; 
                }
                'q' => {
                    black_evaluation += CHESS_PIECES_VALUE[4];
                    black_evaluation += QUEEN_TABLE.iter().copied().rev().collect::<Vec<_>>()[square_index];           
                },
                'Q' => {
                    white_evaluation += CHESS_PIECES_VALUE[4];  
                    white_evaluation += QUEEN_TABLE[square_index]; 
                }
                _ => (),
            }
            square_index += 1;
        }
    }

    let game: Game = Game::from_str(fen).expect("Valid FEN");
    let status = game.result();
    if status != None {
        if status.unwrap() == GameResult::BlackCheckmates {
            black_evaluation = 200000000;
        }

        if status.unwrap() == GameResult::WhiteCheckmates {
            white_evaluation = 200000000;
        }
    }

    if board.side_to_move() == Color::Black {
        return (-1 * (white_evaluation - black_evaluation)) as f32
    }else {
        return (white_evaluation - black_evaluation) as f32
    }
}

pub fn make_move(fen: &str) -> String {
    let chess_board = Board::from_str(fen).unwrap();
//"1nbqk2r/6pp/8/r7/3p4/3p1KP1/5P1P/4q3 b k - 1 32" 
    let (out, best_move) = search_root(4, chess_board);
    println!("Best: {}, {}", out, best_move);

    return best_move
}